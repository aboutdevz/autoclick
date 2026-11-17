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
                spawn_hook_thread();
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
fn spawn_hook_thread() {
    std::thread::spawn(|| {
        use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
        use windows::Win32::UI::Input::KeyboardAndMouse::{
            GetAsyncKeyState, VK_CONTROL, VK_MENU, VK_SHIFT,
        };
        use windows::Win32::UI::WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
            KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN,
            WM_SYSKEYUP,
        };

        fn is_mod_down(vk: i32) -> bool {
            unsafe { (GetAsyncKeyState(vk) as u16 & 0x8000) != 0 }
        }

        unsafe extern "system" fn hook_proc(
            code: i32,
            wparam: WPARAM,
            lparam: LPARAM,
        ) -> LRESULT {
            if code >= 0 {
                if let Some(ctx) = HOOK_CTX.get() {
                    let target_opt = *ctx.target.lock();
                    if let Some(target) = target_opt {
                        let kbd = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
                        let msg = wparam.0 as u32;
                        let is_down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
                        let is_up = msg == WM_KEYUP || msg == WM_SYSKEYUP;

                        if kbd.vkCode == target.vk_code {
                            if is_down {
                                let ctrl_ok = !target.ctrl || is_mod_down(VK_CONTROL.0 as i32);
                                let alt_ok = !target.alt || is_mod_down(VK_MENU.0 as i32);
                                let shift_ok = !target.shift || is_mod_down(VK_SHIFT.0 as i32);

                                if ctrl_ok
                                    && alt_ok
                                    && shift_ok
                                    && !ctx.is_pressed.swap(true, Ordering::SeqCst)
                                {
                                    (ctx.callback)(HotkeyEvent::Pressed);
                                }
                            } else if is_up && ctx.is_pressed.swap(false, Ordering::SeqCst) {
                                (ctx.callback)(HotkeyEvent::Released);
                            }
                        }
                    }
                }
            }
            CallNextHookEx(None, code, wparam, lparam)
        }

        unsafe {
            let _hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(hook_proc),
                HINSTANCE(std::ptr::null_mut()),
                0,
            );
            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).into() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    });
}

#[cfg(not(windows))]
fn spawn_hook_thread() {}
