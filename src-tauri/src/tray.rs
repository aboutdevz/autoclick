use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};

pub fn setup_tray<R: Runtime>(
    app: &AppHandle<R>,
    on_toggle: impl Fn(&AppHandle<R>) + Send + Sync + 'static,
) -> tauri::Result<()> {
    let toggle_item = MenuItem::with_id(app, "toggle", "Start / Stop Clicking", true, None::<&str>)?;
    let show_item = MenuItem::with_id(app, "show", "Show / Hide Window", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit AutoClick", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&toggle_item, &show_item, &quit_item])?;

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .menu(&menu)
        .tooltip("AutoClick — Idle")
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "toggle" => on_toggle(app),
            "show" => toggle_window_visibility(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_window_visibility(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;
    Ok(())
}

pub fn update_tray_tooltip<R: Runtime>(app: &AppHandle<R>, running: bool, clicks: u64) {
    if let Some(tray) = app.tray_by_id("main-tray") {
        let text = if running {
            format!("AutoClick — Running ({clicks} clicks)")
        } else {
            "AutoClick — Idle".to_string()
        };
        let _ = tray.set_tooltip(Some(&text));
    }
}

pub fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
}

pub fn toggle_window_visibility<R: Runtime>(app: &AppHandle<R>) {
    if let Some(win) = app.get_webview_window("main") {
        let visible = win.is_visible().unwrap_or(false);
        let minimized = win.is_minimized().unwrap_or(false);
        if visible && !minimized {
            let _ = win.hide();
        } else {
            show_main_window(app);
        }
    }
}
