import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { MousePointerClick, Settings, Minus, X, Sun, Moon, Monitor } from "lucide-react";
import type { Language, ThemeMode } from "../types";
import type { TranslationKey } from "../i18n/translations";

interface TitlebarProps {
  running: boolean;
  isPortable: boolean;
  theme: ThemeMode;
  language: Language;
  minimizeToTray: boolean;
  onCycleTheme: () => void;
  onToggleLanguage: () => void;
  onOpenSettings: () => void;
  t: (key: TranslationKey) => string;
}

export function Titlebar({
  running,
  isPortable,
  theme,
  language,
  minimizeToTray,
  onCycleTheme,
  onToggleLanguage,
  onOpenSettings,
  t,
}: TitlebarProps) {
  const appWindow = getCurrentWindow();

  const handleMinimize = () => {
    void appWindow.minimize();
  };

  const handleClose = () => {
    if (minimizeToTray) {
      void invoke("hide_to_tray");
    } else {
      void appWindow.close();
    }
  };

  return (
    <header
      data-tauri-drag-region
      className="flex items-center justify-between px-4 py-2.5 border-b border-slate-200/80 dark:border-white/[0.07] bg-white/80 dark:bg-slate-900/80 backdrop-blur-md shrink-0"
    >
      <div className="flex items-center gap-2.5 pointer-events-none">
        <div className="w-7 h-7 rounded-lg bg-gradient-to-br from-indigo-500 to-violet-600 flex items-center justify-center shadow-sm shadow-indigo-500/30">
          <MousePointerClick className="w-4 h-4 text-white" />
        </div>
        <div className="flex items-center gap-2">
          <span className="font-semibold text-sm tracking-tight text-slate-900 dark:text-slate-100">
            {t("appName")}
          </span>
          {isPortable && (
            <span className="px-1.5 py-0.5 text-[10px] font-medium rounded bg-indigo-500/10 text-indigo-600 dark:text-indigo-400 border border-indigo-500/20">
              {t("portable")}
            </span>
          )}
          <span
            className={`inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[10px] font-semibold tracking-wider ${
              running
                ? "bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border border-emerald-500/25"
                : "bg-slate-200/70 dark:bg-slate-800 text-slate-500 dark:text-slate-400 border border-slate-300/60 dark:border-white/5"
            }`}
          >
            <span
              className={`w-1.5 h-1.5 rounded-full ${
                running ? "bg-emerald-500 animate-pulse" : "bg-slate-400 dark:bg-slate-500"
              }`}
            />
            {running ? t("running") : t("idle")}
          </span>
        </div>
      </div>

      <div className="flex items-center gap-1">
        <button
          onClick={onToggleLanguage}
          title={t("language")}
          className="px-2 py-1 rounded-md text-[11px] font-semibold text-slate-600 dark:text-slate-300 hover:bg-slate-200/70 dark:hover:bg-white/10 transition-colors cursor-pointer"
        >
          {language.toUpperCase()}
        </button>

        <button
          onClick={onCycleTheme}
          title={`${t("theme")}: ${t(theme)}`}
          className="p-1.5 rounded-md text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-100 hover:bg-slate-200/70 dark:hover:bg-white/10 transition-colors cursor-pointer"
        >
          {theme === "dark" ? (
            <Moon className="w-3.5 h-3.5" />
          ) : theme === "light" ? (
            <Sun className="w-3.5 h-3.5" />
          ) : (
            <Monitor className="w-3.5 h-3.5" />
          )}
        </button>

        <button
          onClick={onOpenSettings}
          title={t("settings")}
          className="p-1.5 rounded-md text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-100 hover:bg-slate-200/70 dark:hover:bg-white/10 transition-colors cursor-pointer"
        >
          <Settings className="w-3.5 h-3.5" />
        </button>

        <div className="w-px h-4 bg-slate-200 dark:bg-white/10 mx-0.5" />

        <button
          onClick={handleMinimize}
          title={t("minimize")}
          className="p-1.5 rounded-md text-slate-500 dark:text-slate-400 hover:bg-slate-200/70 dark:hover:bg-white/10 transition-colors cursor-pointer"
        >
          <Minus className="w-3.5 h-3.5" />
        </button>

        <button
          onClick={handleClose}
          title={minimizeToTray ? t("hideToTray") : t("close")}
          className="p-1.5 rounded-md text-slate-500 dark:text-slate-400 hover:bg-rose-500 hover:text-white transition-colors cursor-pointer"
        >
          <X className="w-3.5 h-3.5" />
        </button>
      </div>
    </header>
  );
}
