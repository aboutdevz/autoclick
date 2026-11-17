use crate::config::{ClickType, MouseButton};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub trait InputBackend: Send + Sync {
    fn click(&self, button: MouseButton, click_type: ClickType, pos: Option<(i32, i32)>);
    fn mouse_down(&self, button: MouseButton, pos: Option<(i32, i32)>);
    fn mouse_up(&self, button: MouseButton);
    fn get_cursor_pos(&self) -> (i32, i32);
    fn is_left_button_down(&self) -> bool;
    fn is_escape_pressed(&self) -> bool;
}

pub fn default_backend() -> Arc<dyn InputBackend> {
    #[cfg(windows)]
    {
        Arc::new(Win32InputBackend)
    }
    #[cfg(not(windows))]
    {
        Arc::new(MockInputBackend::new())
    }
}

#[derive(Debug, Default)]
pub struct MockInputBackend {
    pub click_count: AtomicU64,
    pub down_count: AtomicU64,
    pub up_count: AtomicU64,
    pub cursor_pos: Mutex<(i32, i32)>,
}

impl MockInputBackend {
    pub fn new() -> Self {
        Self::default()
    }
}

impl InputBackend for MockInputBackend {
    fn click(&self, _button: MouseButton, click_type: ClickType, pos: Option<(i32, i32)>) {
        if let Some(p) = pos {
            *self.cursor_pos.lock() = p;
        }
        let n = match click_type {
            ClickType::Double => 2,
            _ => 1,
        };
        self.click_count.fetch_add(n, Ordering::SeqCst);
    }

    fn mouse_down(&self, _button: MouseButton, pos: Option<(i32, i32)>) {
        if let Some(p) = pos {
            *self.cursor_pos.lock() = p;
        }
        self.down_count.fetch_add(1, Ordering::SeqCst);
    }

    fn mouse_up(&self, _button: MouseButton) {
        self.up_count.fetch_add(1, Ordering::SeqCst);
    }

    fn get_cursor_pos(&self) -> (i32, i32) {
        *self.cursor_pos.lock()
    }

    fn is_left_button_down(&self) -> bool {
        false
    }

    fn is_escape_pressed(&self) -> bool {
        false
    }
}

#[cfg(windows)]
pub struct Win32InputBackend;

#[cfg(windows)]
impl Win32InputBackend {
    fn button_flags(button: MouseButton) -> (u32, u32) {
        use windows::Win32::UI::Input::KeyboardAndMouse::{
            MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
            MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
        };
        match button {
            MouseButton::Left => (MOUSEEVENTF_LEFTDOWN.0, MOUSEEVENTF_LEFTUP.0),
            MouseButton::Right => (MOUSEEVENTF_RIGHTDOWN.0, MOUSEEVENTF_RIGHTUP.0),
            MouseButton::Middle => (MOUSEEVENTF_MIDDLEDOWN.0, MOUSEEVENTF_MIDDLEUP.0),
        }
    }

    fn send_mouse_flags(flags: u32) {
        use std::mem::size_of;
        use windows::Win32::UI::Input::KeyboardAndMouse::{
            SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEINPUT, MOUSE_EVENT_FLAGS,
        };

        let input = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSE_EVENT_FLAGS(flags),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        unsafe {
            SendInput(&[input], size_of::<INPUT>() as i32);
        }
    }
}

#[cfg(windows)]
impl InputBackend for Win32InputBackend {
    fn click(&self, button: MouseButton, click_type: ClickType, pos: Option<(i32, i32)>) {
        use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;
        if let Some((x, y)) = pos {
            unsafe {
                let _ = SetCursorPos(x, y);
            }
        }
        let (down, up) = Self::button_flags(button);
        Self::send_mouse_flags(down);
        std::thread::sleep(std::time::Duration::from_millis(10));
        Self::send_mouse_flags(up);
        if click_type == ClickType::Double {
            std::thread::sleep(std::time::Duration::from_millis(15));
            Self::send_mouse_flags(down);
            std::thread::sleep(std::time::Duration::from_millis(10));
            Self::send_mouse_flags(up);
        }
    }

    fn mouse_down(&self, button: MouseButton, pos: Option<(i32, i32)>) {
        use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;
        if let Some((x, y)) = pos {
            unsafe {
                let _ = SetCursorPos(x, y);
            }
        }
        let (down, _) = Self::button_flags(button);
        Self::send_mouse_flags(down);
    }

    fn mouse_up(&self, button: MouseButton) {
        let (_, up) = Self::button_flags(button);
        Self::send_mouse_flags(up);
    }

    fn get_cursor_pos(&self) -> (i32, i32) {
        use windows::Win32::Foundation::POINT;
        use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
        let mut pt = POINT::default();
        unsafe {
            let _ = GetCursorPos(&mut pt);
        }
        (pt.x, pt.y)
    }

    fn is_left_button_down(&self) -> bool {
        use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON};
        unsafe { (GetAsyncKeyState(VK_LBUTTON.0 as i32) as u16 & 0x8000) != 0 }
    }

    fn is_escape_pressed(&self) -> bool {
        use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_ESCAPE};
        unsafe { (GetAsyncKeyState(VK_ESCAPE.0 as i32) as u16 & 0x8000) != 0 }
    }
}
