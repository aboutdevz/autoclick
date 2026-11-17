pub mod config;
pub mod engine;
pub mod hotkey;
pub mod hotkey_hook;
pub mod input;

pub use config::{
    load_config, resolve_config_path, save_config, AppConfig, ClickType, HotkeyMode, Language,
    MouseButton, PositionMode, RepeatMode, ThemeMode,
};
pub use engine::{compute_delay_ms, ClickEngine, EngineStatus};
pub use hotkey::{parse_hotkey, HotkeyEvent, ParsedHotkey};
pub use hotkey_hook::GlobalHotkeyListener;
pub use input::{default_backend, InputBackend, MockInputBackend};
