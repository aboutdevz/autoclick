use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClickType {
    Single,
    Double,
    Hold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RepeatMode {
    Infinite,
    Count,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PositionMode {
    Current,
    Fixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HotkeyMode {
    Toggle,
    Hold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    En,
    Id,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub interval_ms: u64,
    pub jitter_enabled: bool,
    pub jitter_ms: u64,
    pub button: MouseButton,
    pub click_type: ClickType,
    pub repeat_mode: RepeatMode,
    pub repeat_count: u64,
    pub position_mode: PositionMode,
    pub fixed_x: i32,
    pub fixed_y: i32,
    pub hotkey: String,
    pub hotkey_mode: HotkeyMode,
    pub minimize_to_tray: bool,
    pub start_minimized: bool,
    pub launch_on_startup: bool,
    pub theme: ThemeMode,
    pub language: Language,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            interval_ms: 100,
            jitter_enabled: false,
            jitter_ms: 15,
            button: MouseButton::Left,
            click_type: ClickType::Single,
            repeat_mode: RepeatMode::Infinite,
            repeat_count: 100,
            position_mode: PositionMode::Current,
            fixed_x: 0,
            fixed_y: 0,
            hotkey: "F6".to_string(),
            hotkey_mode: HotkeyMode::Toggle,
            minimize_to_tray: true,
            start_minimized: false,
            launch_on_startup: false,
            theme: ThemeMode::Dark,
            language: Language::En,
        }
    }
}

/// Resolves where `config.json` should be stored and whether the app is running in portable mode.
/// Portable mode is active when `cli_portable` is true, or when `portable.flag` or `config.json`
/// exists alongside the executable in `exe_dir`.
pub fn resolve_config_path(
    app_data_dir: Option<&Path>,
    exe_dir: Option<&Path>,
    cli_portable: bool,
) -> (PathBuf, bool) {
    if let Some(dir) = exe_dir {
        let has_flag = dir.join("portable.flag").exists();
        let has_local_cfg = dir.join("config.json").exists();
        if cli_portable || has_flag || has_local_cfg {
            return (dir.join("config.json"), true);
        }
    }

    if let Some(data_dir) = app_data_dir {
        return (data_dir.join("config.json"), false);
    }

    (PathBuf::from("config.json"), false)
}

pub fn load_config(path: &Path) -> AppConfig {
    match fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => AppConfig::default(),
    }
}

pub fn save_config(path: &Path, config: &AppConfig) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create config dir: {e}"))?;
        }
    }
    let json =
        serde_json::to_string_pretty(config).map_err(|e| format!("Failed to serialize config: {e}"))?;
    fs::write(path, json).map_err(|e| format!("Failed to write config: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("autoclick_test_{name}_{nanos}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn default_config_roundtrip() {
        let dir = temp_dir("roundtrip");
        let cfg_path = dir.join("config.json");
        let mut cfg = AppConfig::default();
        cfg.interval_ms = 250;
        cfg.jitter_enabled = true;
        cfg.language = Language::Id;

        save_config(&cfg_path, &cfg).unwrap();
        let loaded = load_config(&cfg_path);
        assert_eq!(loaded, cfg);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn portable_detection_via_flag() {
        let exe_dir = temp_dir("portable_flag");
        let app_data = temp_dir("appdata");
        fs::write(exe_dir.join("portable.flag"), "").unwrap();

        let (path, is_portable) = resolve_config_path(Some(&app_data), Some(&exe_dir), false);
        assert!(is_portable);
        assert_eq!(path, exe_dir.join("config.json"));

        let _ = fs::remove_dir_all(&exe_dir);
        let _ = fs::remove_dir_all(&app_data);
    }

    #[test]
    fn standard_appdata_when_not_portable() {
        let exe_dir = temp_dir("std_exe");
        let app_data = temp_dir("std_appdata");

        let (path, is_portable) = resolve_config_path(Some(&app_data), Some(&exe_dir), false);
        assert!(!is_portable);
        assert_eq!(path, app_data.join("config.json"));

        let _ = fs::remove_dir_all(&exe_dir);
        let _ = fs::remove_dir_all(&app_data);
    }
}
