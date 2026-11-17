import { useEffect, useState } from "react";
import { X, Keyboard, AppWindow, Palette } from "lucide-react";
import type { AppConfig, HotkeyMode, Language, ThemeMode } from "../types";
import type { TranslationKey } from "../i18n/translations";

interface SettingsModalProps {
  open: boolean;
  config: AppConfig;
  hotkeyError: string | null;
  onClose: () => void;
  onUpdate: (partial: Partial<AppConfig>) => void;
  t: (key: TranslationKey) => string;
}

function keyEventToShortcut(e: KeyboardEvent): string | null {
  const ignored = new Set(["Control", "Shift", "Alt", "Meta"]);
  if (ignored.has(e.key)) return null;

  const parts: string[] = [];
  if (e.ctrlKey) parts.push("Ctrl");
  if (e.altKey) parts.push("Alt");
  if (e.shiftKey) parts.push("Shift");

  let key = e.key;
  if (key === " ") key = "Space";
  else if (key.length === 1) key = key.toUpperCase();
  else if (/^f\d+$/i.test(key)) key = key.toUpperCase();

  parts.push(key);
  return parts.join("+");
}

export function SettingsModal({
  open,
  config,
  hotkeyError,
  onClose,
  onUpdate,
  t,
}: SettingsModalProps) {
  const [recording, setRecording] = useState(false);

  useEffect(() => {
    if (!recording) return;
    const handler = (e: KeyboardEvent) => {
      e.preventDefault();
      e.stopPropagation();
      if (e.key === "Escape") {
        setRecording(false);
        return;
      }
      const shortcut = keyEventToShortcut(e);
      if (shortcut) {
        onUpdate({ hotkey: shortcut });
        setRecording(false);
      }
    };
    window.addEventListener("keydown", handler, true);
    return () => window.removeEventListener("keydown", handler, true);
  }, [recording, onUpdate]);

  if (!open) return null;

  const ToggleRow = ({
    label,
    checked,
    onChange,
  }: {
    label: string;
    checked: boolean;
    onChange: (val: boolean) => void;
  }) => (
    <div className="flex items-center justify-between py-1.5">
      <span className="text-xs font-medium text-slate-700 dark:text-slate-300">
        {label}
      </span>
      <button
        type="button"
        role="switch"
        aria-checked={checked}
        onClick={() => onChange(!checked)}
        className={`w-9 h-5 rounded-full transition-colors p-0.5 cursor-pointer shrink-0 ${
          checked ? "bg-indigo-500" : "bg-slate-300 dark:bg-slate-700"
        }`}
      >
        <div
          className={`w-4 h-4 rounded-full bg-white shadow transition-transform ${
            checked ? "translate-x-4" : "translate-x-0"
          }`}
        />
      </button>
    </div>
  );

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-xs p-4">
      <div className="w-full max-w-md rounded-2xl bg-white dark:bg-slate-900 border border-slate-200 dark:border-white/10 shadow-2xl overflow-hidden">
        <div className="flex items-center justify-between px-4 py-3 border-b border-slate-200/80 dark:border-white/[0.07]">
          <h2 className="text-sm font-bold text-slate-900 dark:text-white">
            {t("settings")}
          </h2>
          <button
            onClick={onClose}
            className="p-1 rounded-lg text-slate-400 hover:text-slate-700 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-white/10 cursor-pointer"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        <div className="p-4 space-y-4 max-h-[75vh] overflow-y-auto">
          {/* Hotkey Section */}
          <div className="space-y-2.5">
            <div className="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
              <Keyboard className="w-3.5 h-3.5 text-indigo-500" />
              <span>{t("hotkeySection")}</span>
            </div>

            <div className="flex items-center justify-between gap-2">
              <span className="text-xs font-medium text-slate-700 dark:text-slate-300">
                {t("hotkeyLabel")}
              </span>
              <button
                onClick={() => setRecording(true)}
                className={`px-3 py-1.5 rounded-xl text-xs font-semibold border transition-all cursor-pointer ${
                  recording
                    ? "bg-indigo-500 text-white border-indigo-400 animate-pulse"
                    : "bg-slate-100 dark:bg-slate-800 text-slate-900 dark:text-white border-slate-200 dark:border-white/10 hover:border-indigo-500"
                }`}
              >
                {recording ? t("hotkeyRecord") : config.hotkey || "None"}
              </button>
            </div>

            {hotkeyError && (
              <div className="text-[11px] text-rose-500 font-medium">
                {t("hotkeyError")}: {hotkeyError}
              </div>
            )}

            <div className="space-y-1.5">
              <div className="text-xs font-medium text-slate-700 dark:text-slate-300">
                {t("hotkeyMode")}
              </div>
              <div className="grid grid-cols-2 gap-1.5 p-1 rounded-xl bg-slate-100 dark:bg-slate-950/70">
                {(["toggle", "hold"] as HotkeyMode[]).map((m) => (
                  <button
                    key={m}
                    onClick={() => onUpdate({ hotkey_mode: m })}
                    className={`py-1.5 px-2 rounded-lg text-[11px] font-medium transition-all cursor-pointer ${
                      config.hotkey_mode === m
                        ? "bg-white dark:bg-slate-800 text-slate-900 dark:text-white shadow-xs"
                        : "text-slate-500 dark:text-slate-400"
                    }`}
                  >
                    {m === "toggle" ? t("toggleMode") : t("holdMode")}
                  </button>
                ))}
              </div>
            </div>
          </div>

          <hr className="border-slate-200/70 dark:border-white/[0.06]" />

          {/* System Tray & Startup */}
          <div className="space-y-1.5">
            <div className="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400 mb-1">
              <AppWindow className="w-3.5 h-3.5 text-indigo-500" />
              <span>{t("traySection")}</span>
            </div>
            <ToggleRow
              label={t("minimizeToTray")}
              checked={config.minimize_to_tray}
              onChange={(v) => onUpdate({ minimize_to_tray: v })}
            />
            <ToggleRow
              label={t("startMinimized")}
              checked={config.start_minimized}
              onChange={(v) => onUpdate({ start_minimized: v })}
            />
            <ToggleRow
              label={t("launchOnStartup")}
              checked={config.launch_on_startup}
              onChange={(v) => onUpdate({ launch_on_startup: v })}
            />
          </div>

          <hr className="border-slate-200/70 dark:border-white/[0.06]" />

          {/* Appearance & Language */}
          <div className="space-y-2.5">
            <div className="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
              <Palette className="w-3.5 h-3.5 text-indigo-500" />
              <span>{t("appearanceSection")}</span>
            </div>

            <div className="flex items-center justify-between gap-2">
              <span className="text-xs font-medium text-slate-700 dark:text-slate-300">
                {t("theme")}
              </span>
              <div className="flex p-0.5 rounded-lg bg-slate-100 dark:bg-slate-950/70">
                {(["dark", "light", "system"] as ThemeMode[]).map((th) => (
                  <button
                    key={th}
                    onClick={() => onUpdate({ theme: th })}
                    className={`px-2.5 py-1 rounded-md text-[11px] font-medium cursor-pointer ${
                      config.theme === th
                        ? "bg-white dark:bg-slate-800 text-slate-900 dark:text-white shadow-xs"
                        : "text-slate-500 dark:text-slate-400"
                    }`}
                  >
                    {t(th)}
                  </button>
                ))}
              </div>
            </div>

            <div className="flex items-center justify-between gap-2">
              <span className="text-xs font-medium text-slate-700 dark:text-slate-300">
                {t("language")}
              </span>
              <div className="flex p-0.5 rounded-lg bg-slate-100 dark:bg-slate-950/70">
                {(
                  [
                    { code: "en", label: "English" },
                    { code: "id", label: "Indonesia" },
                  ] as { code: Language; label: string }[]
                ).map((lang) => (
                  <button
                    key={lang.code}
                    onClick={() => onUpdate({ language: lang.code })}
                    className={`px-2.5 py-1 rounded-md text-[11px] font-medium cursor-pointer ${
                      config.language === lang.code
                        ? "bg-white dark:bg-slate-800 text-slate-900 dark:text-white shadow-xs"
                        : "text-slate-500 dark:text-slate-400"
                    }`}
                  >
                    {lang.label}
                  </button>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
