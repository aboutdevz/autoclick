mod tray;

use autoclick_core::{
    default_backend, load_config, resolve_config_path, save_config, AppConfig, ClickEngine,
    EngineStatus, HotkeyMode, InputBackend,
};
use parking_lot::Mutex;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, Runtime, State, WindowEvent};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt as AutostartExt};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

struct AppState {
    engine: ClickEngine,
    backend: Arc<dyn InputBackend>,
    config: Mutex<AppConfig>,
    config_path: PathBuf,
    is_portable: bool,
    picking: AtomicBool,
}

#[derive(Debug, Clone, Serialize)]
struct ConfigResponse {
    config: AppConfig,
    is_portable: bool,
}

#[derive(Debug, Clone, Serialize)]
struct CoordPayload {
    x: i32,
    y: i32,
}

fn start_engine_with_app<R: Runtime>(app: &AppHandle<R>, state: &AppState) -> bool {
    let cfg = state.config.lock().clone();
    let app_handle = app.clone();
    state.engine.start(
        cfg,
        Arc::new(move |status: EngineStatus| {
            let _ = app_handle.emit("clicker://status", &status);
            tray::update_tray_tooltip(&app_handle, status.running, status.clicks);
        }),
    )
}

fn toggle_clicking_with_app<R: Runtime>(app: &AppHandle<R>) {
    let state = app.state::<AppState>();
    if state.engine.is_running() {
        state.engine.stop();
    } else {
        start_engine_with_app(app, &state);
    }
}

fn register_hotkey<R: Runtime>(app: &AppHandle<R>, hotkey: &str) -> Result<(), String> {
    let gs = app.global_shortcut();
    let _ = gs.unregister_all();
    if hotkey.trim().is_empty() {
        return Ok(());
    }
    gs.register(hotkey)
        .map_err(|e| format!("Invalid or unavailable hotkey '{hotkey}': {e}"))
}

#[tauri::command]
fn get_config(state: State<'_, AppState>) -> ConfigResponse {
    ConfigResponse {
        config: state.config.lock().clone(),
        is_portable: state.is_portable,
    }
}

#[tauri::command]
fn save_app_config<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
    new_config: AppConfig,
) -> Result<ConfigResponse, String> {
    let old_config = state.config.lock().clone();

    if old_config.hotkey != new_config.hotkey {
        register_hotkey(&app, &new_config.hotkey)?;
    }

    if old_config.launch_on_startup != new_config.launch_on_startup {
        let autolaunch = app.autolaunch();
        if new_config.launch_on_startup {
            let _ = autolaunch.enable();
        } else {
            let _ = autolaunch.disable();
        }
    }

    save_config(&state.config_path, &new_config)?;
    *state.config.lock() = new_config.clone();

    Ok(ConfigResponse {
        config: new_config,
        is_portable: state.is_portable,
    })
}

#[tauri::command]
fn get_status(state: State<'_, AppState>) -> EngineStatus {
    state.engine.status()
}

#[tauri::command]
fn start_clicking<R: Runtime>(app: AppHandle<R>, state: State<'_, AppState>) -> bool {
    start_engine_with_app(&app, &state)
}

#[tauri::command]
fn stop_clicking(state: State<'_, AppState>) -> bool {
    state.engine.stop()
}

#[tauri::command]
fn start_coordinate_picker<R: Runtime>(app: AppHandle<R>, state: State<'_, AppState>) -> bool {
    if state
        .picking
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return false;
    }

    let backend = Arc::clone(&state.backend);
    let app_handle = app.clone();

    std::thread::spawn(move || {
        // Brief debounce so clicking the UI button doesn't immediately register
        std::thread::sleep(std::time::Duration::from_millis(280));
        let mut was_down = backend.is_left_button_down();

        loop {
            let state_ref = app_handle.state::<AppState>();
            if !state_ref.picking.load(Ordering::SeqCst) {
                break;
            }

            if backend.is_escape_pressed() {
                state_ref.picking.store(false, Ordering::SeqCst);
                let _ = app_handle.emit("picker://cancelled", ());
                break;
            }

            let (x, y) = backend.get_cursor_pos();
            let _ = app_handle.emit("picker://move", CoordPayload { x, y });

            let is_down = backend.is_left_button_down();
            if is_down && !was_down {
                state_ref.picking.store(false, Ordering::SeqCst);
                let _ = app_handle.emit("picker://picked", CoordPayload { x, y });
                break;
            }
            was_down = is_down;

            std::thread::sleep(std::time::Duration::from_millis(25));
        }
    });

    true
}

#[tauri::command]
fn cancel_coordinate_picker(state: State<'_, AppState>) {
    state.picking.store(false, Ordering::SeqCst);
}

#[tauri::command]
fn hide_to_tray<R: Runtime>(app: AppHandle<R>) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.hide();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let cli_portable = args.iter().any(|a| a == "--portable");
    let cli_minimized = args.iter().any(|a| a == "--minimized");

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    let state = app.state::<AppState>();
                    let mode = state.config.lock().hotkey_mode;
                    match mode {
                        HotkeyMode::Toggle => {
                            if event.state == ShortcutState::Pressed {
                                toggle_clicking_with_app(app);
                            }
                        }
                        HotkeyMode::Hold => match event.state {
                            ShortcutState::Pressed => {
                                start_engine_with_app(app, &state);
                            }
                            ShortcutState::Released => {
                                state.engine.stop();
                            }
                        },
                    }
                })
                .build(),
        )
        .setup(move |app| {
            let app_data_dir = app.path().app_data_dir().ok();
            let exe_dir = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.to_path_buf()));

            let (config_path, is_portable) =
                resolve_config_path(app_data_dir.as_deref(), exe_dir.as_deref(), cli_portable);
            let config = load_config(&config_path);

            let backend = default_backend();
            let engine = ClickEngine::new(Arc::clone(&backend));

            let start_minimized = config.start_minimized || cli_minimized;
            let initial_hotkey = config.hotkey.clone();

            app.manage(AppState {
                engine,
                backend,
                config: Mutex::new(config),
                config_path,
                is_portable,
                picking: AtomicBool::new(false),
            });

            tray::setup_tray(app.handle(), |handle| {
                toggle_clicking_with_app(handle);
            })?;

            let _ = register_hotkey(app.handle(), &initial_hotkey);

            if !start_minimized {
                tray::show_main_window(app.handle());
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let state = window.state::<AppState>();
                if state.config.lock().minimize_to_tray {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_app_config,
            get_status,
            start_clicking,
            stop_clicking,
            start_coordinate_picker,
            cancel_coordinate_picker,
            hide_to_tray
        ])
        .run(tauri::generate_context!())
        .expect("error while running AutoClick application");
}
