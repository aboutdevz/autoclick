export type MouseButton = "left" | "right" | "middle";
export type ClickType = "single" | "double" | "hold";
export type RepeatMode = "infinite" | "count";
export type PositionMode = "current" | "fixed";
export type HotkeyMode = "toggle" | "hold";
export type ThemeMode = "dark" | "light" | "system";
export type Language = "en" | "id";

export interface AppConfig {
  interval_ms: number;
  jitter_enabled: boolean;
  jitter_ms: number;
  button: MouseButton;
  click_type: ClickType;
  repeat_mode: RepeatMode;
  repeat_count: number;
  position_mode: PositionMode;
  fixed_x: number;
  fixed_y: number;
  hotkey: string;
  hotkey_mode: HotkeyMode;
  minimize_to_tray: boolean;
  start_minimized: boolean;
  launch_on_startup: boolean;
  theme: ThemeMode;
  language: Language;
}

export interface ConfigResponse {
  config: AppConfig;
  is_portable: boolean;
}

export interface EngineStatus {
  running: boolean;
  clicks: number;
  elapsed_ms: number;
}

export interface CoordPayload {
  x: number;
  y: number;
}

export const DEFAULT_CONFIG: AppConfig = {
  interval_ms: 100,
  jitter_enabled: false,
  jitter_ms: 15,
  button: "left",
  click_type: "single",
  repeat_mode: "infinite",
  repeat_count: 100,
  position_mode: "current",
  fixed_x: 0,
  fixed_y: 0,
  hotkey: "F6",
  hotkey_mode: "toggle",
  minimize_to_tray: true,
  start_minimized: false,
  launch_on_startup: false,
  theme: "dark",
  language: "en",
};
