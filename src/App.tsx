import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  DEFAULT_CONFIG,
  type AppConfig,
  type ConfigResponse,
  type CoordPayload,
  type EngineStatus,
  type ThemeMode,
} from "./types";
import { translations, type TranslationKey } from "./i18n/translations";
import { Titlebar } from "./components/Titlebar";
import { StatusBanner } from "./components/StatusBanner";
import { IntervalCard } from "./components/IntervalCard";
import { ClickOptionsCard } from "./components/ClickOptionsCard";
import { PositionCard } from "./components/PositionCard";
import { SettingsModal } from "./components/SettingsModal";

export function App() {
  const [config, setConfig] = useState<AppConfig>(DEFAULT_CONFIG);
  const [isPortable, setIsPortable] = useState(false);
  const [status, setStatus] = useState<EngineStatus>({
    running: false,
    clicks: 0,
    elapsed_ms: 0,
  });
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [hotkeyError, setHotkeyError] = useState<string | null>(null);
  const [picking, setPicking] = useState(false);
  const [liveCoord, setLiveCoord] = useState<CoordPayload | null>(null);

  const t = useCallback(
    (key: TranslationKey) => translations[config.language][key],
    [config.language]
  );

  // Apply theme to documentElement
  useEffect(() => {
    const root = document.documentElement;
    const apply = (isDark: boolean) => {
      root.classList.toggle("dark", isDark);
    };

    if (config.theme === "system") {
      const mq = window.matchMedia("(prefers-color-scheme: dark)");
      apply(mq.matches);
      const handler = (e: MediaQueryListEvent) => apply(e.matches);
      mq.addEventListener("change", handler);
      return () => mq.removeEventListener("change", handler);
    }
    apply(config.theme === "dark");
  }, [config.theme]);

  // Initial load & Tauri event listeners
  useEffect(() => {
    void invoke<ConfigResponse>("get_config")
      .then((res) => {
        setConfig(res.config);
        setIsPortable(res.is_portable);
      })
      .catch(() => {});

    void invoke<EngineStatus>("get_status")
      .then(setStatus)
      .catch(() => {});

    const unlistenStatus = listen<EngineStatus>("clicker://status", (e) => {
      setStatus(e.payload);
    });
    const unlistenMove = listen<CoordPayload>("picker://move", (e) => {
      setLiveCoord(e.payload);
    });
    const unlistenPicked = listen<CoordPayload>("picker://picked", (e) => {
      setPicking(false);
      setLiveCoord(null);
      setConfig((prev) => {
        const next = { ...prev, fixed_x: e.payload.x, fixed_y: e.payload.y };
        void invoke("save_app_config", { newConfig: next }).catch(() => {});
        return next;
      });
    });
    const unlistenCancel = listen("picker://cancelled", () => {
      setPicking(false);
      setLiveCoord(null);
    });

    return () => {
      void unlistenStatus.then((fn) => fn());
      void unlistenMove.then((fn) => fn());
      void unlistenPicked.then((fn) => fn());
      void unlistenCancel.then((fn) => fn());
    };
  }, []);

  const updateConfig = useCallback((partial: Partial<AppConfig>) => {
    setConfig((prev) => {
      const next = { ...prev, ...partial };
      void invoke<ConfigResponse>("save_app_config", { newConfig: next })
        .then(() => setHotkeyError(null))
        .catch((err) => {
          setHotkeyError(String(err));
        });
      return next;
    });
  }, []);

  const handleToggleClicking = () => {
    if (status.running) {
      void invoke("stop_clicking");
    } else {
      void invoke("start_clicking");
    }
  };

  const handleStartPick = () => {
    if (picking) {
      void invoke("cancel_coordinate_picker");
      setPicking(false);
      setLiveCoord(null);
    } else {
      setPicking(true);
      void invoke("start_coordinate_picker");
    }
  };

  const handleCycleTheme = () => {
    const order: ThemeMode[] = ["dark", "light", "system"];
    const next = order[(order.indexOf(config.theme) + 1) % order.length];
    updateConfig({ theme: next });
  };

  const handleToggleLanguage = () => {
    updateConfig({ language: config.language === "en" ? "id" : "en" });
  };

  return (
    <div className="h-screen w-screen flex flex-col bg-slate-100 dark:bg-slate-950 text-slate-900 dark:text-slate-100 transition-colors">
      <Titlebar
        running={status.running}
        isPortable={isPortable}
        theme={config.theme}
        language={config.language}
        minimizeToTray={config.minimize_to_tray}
        onCycleTheme={handleCycleTheme}
        onToggleLanguage={handleToggleLanguage}
        onOpenSettings={() => setSettingsOpen(true)}
        t={t}
      />

      <main className="flex-1 overflow-y-auto p-3.5 space-y-3">
        <StatusBanner
          status={status}
          hotkey={config.hotkey}
          hotkeyMode={config.hotkey_mode}
          onToggle={handleToggleClicking}
          t={t}
        />

        <IntervalCard
          intervalMs={config.interval_ms}
          jitterEnabled={config.jitter_enabled}
          jitterMs={config.jitter_ms}
          disabled={status.running}
          onChangeInterval={(ms) => updateConfig({ interval_ms: ms })}
          onChangeJitter={(enabled, ms) =>
            updateConfig({ jitter_enabled: enabled, jitter_ms: ms })
          }
          t={t}
        />

        <ClickOptionsCard
          button={config.button}
          clickType={config.click_type}
          repeatMode={config.repeat_mode}
          repeatCount={config.repeat_count}
          disabled={status.running}
          onChangeButton={(button) => updateConfig({ button })}
          onChangeClickType={(click_type) => updateConfig({ click_type })}
          onChangeRepeat={(repeat_mode, repeat_count) =>
            updateConfig({ repeat_mode, repeat_count })
          }
          t={t}
        />

        <PositionCard
          mode={config.position_mode}
          x={config.fixed_x}
          y={config.fixed_y}
          picking={picking}
          liveCoord={liveCoord}
          disabled={status.running}
          onChangeMode={(position_mode) => updateConfig({ position_mode })}
          onChangeCoords={(fixed_x, fixed_y) =>
            updateConfig({ fixed_x, fixed_y })
          }
          onStartPick={handleStartPick}
          t={t}
        />
      </main>

      <SettingsModal
        open={settingsOpen}
        config={config}
        hotkeyError={hotkeyError}
        onClose={() => setSettingsOpen(false)}
        onUpdate={updateConfig}
        t={t}
      />
    </div>
  );
}

export default App;
