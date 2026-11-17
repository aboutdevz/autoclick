use crate::hotkey::{parse_hotkey, HotkeyEvent, ParsedHotkey};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

struct HookContext {
    target: Mutex<Option<ParsedHotkey>>,
    is_pressed: AtomicBool,
    #[cfg_attr(not(windows), allow(dead_code))]
    callback: Arc<dyn Fn(HotkeyEvent) + Send + Sync>,
}

static HOOK_CTX: OnceLock<Arc<HookContext>> = OnceLock::new();

pub struct GlobalHotkeyListener {
    ctx: Arc<HookContext>,
}

impl GlobalHotkeyListener {
    pub fn start(
        initial_hotkey: &str,
        callback: Arc<dyn Fn(HotkeyEvent) + Send + Sync>,
    ) -> Result<Self, String> {
        let parsed = parse_hotkey(initial_hotkey);
        let ctx = HOOK_CTX
            .get_or_init(|| {
                let c = Arc::new(HookContext {
                    target: Mutex::new(parsed),
                    is_pressed: AtomicBool::new(false),
                    callback: Arc::clone(&callback),
                });
                spawn_hotkey_thread();
                c
            })
            .clone();

        *ctx.target.lock() = parsed;
        ctx.is_pressed.store(false, Ordering::SeqCst);

        Ok(Self { ctx })
    }

    pub fn set_hotkey(&self, hotkey: &str) -> Result<(), String> {
        let trimmed = hotkey.trim();
        if trimmed.is_empty() {
            *self.ctx.target.lock() = None;
            self.ctx.is_pressed.store(false, Ordering::SeqCst);
            return Ok(());
        }
        let parsed = parse_hotkey(trimmed)
            .ok_or_else(|| format!("Unsupported hotkey '{hotkey}'"))?;
        *self.ctx.target.lock() = Some(parsed);
        self.ctx.is_pressed.store(false, Ordering::SeqCst);
        Ok(())
    }
}

#[cfg(windows)]
fn spawn_hotkey_thread() {
    std::thread::spawn(|| {
        use std::time::Duration;
        use windows::Win32::UI::Input::KeyboardAndMouse::{
            GetAsyncKeyState, VK_CONTROL, VK_MENU, VK_SHIFT,
        };

        fn is_vk_down(vk: i32) -> bool {
            unsafe { (GetAsyncKeyState(vk) as u16 & 0x8000) != 0 }
        }

        loop {
            if let Some(ctx) = HOOK_CTX.get() {
                let target_opt = *ctx.target.lock();
                if let Some(target) = target_opt {
                    let main_down = is_vk_down(target.vk_code as i32);
                    let ctrl_ok = !target.ctrl || is_vk_down(VK_CONTROL.0 as i32);
                    let alt_ok = !target.alt || is_vk_down(VK_MENU.0 as i32);
                    let shift_ok = !target.shift || is_vk_down(VK_SHIFT.0 as i32);

                    if main_down && ctrl_ok && alt_ok && shift_ok {
                        if !ctx.is_pressed.swap(true, Ordering::SeqCst) {
                            (ctx.callback)(HotkeyEvent::Pressed);
                        }
                    } else if !main_down && ctx.is_pressed.swap(false, Ordering::SeqCst) {
                        (ctx.callback)(HotkeyEvent::Released);
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(15));
        }
    });
}

#[cfg(not(windows))]
fn spawn_hotkey_thread() {}
