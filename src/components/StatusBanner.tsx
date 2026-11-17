import { Play, Square, Keyboard } from "lucide-react";
import type { EngineStatus, HotkeyMode } from "../types";
import type { TranslationKey } from "../i18n/translations";

interface StatusBannerProps {
  status: EngineStatus;
  hotkey: string;
  hotkeyMode: HotkeyMode;
  onToggle: () => void;
  t: (key: TranslationKey) => string;
}

function formatElapsed(ms: number): string {
  const totalSecs = Math.floor(ms / 1000);
  const mins = Math.floor(totalSecs / 60);
  const secs = totalSecs % 60;
  const tenths = Math.floor((ms % 1000) / 100);
  return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}.${tenths}`;
}

export function StatusBanner({
  status,
  hotkey,
  hotkeyMode,
  onToggle,
  t,
}: StatusBannerProps) {
  const cps =
    status.elapsed_ms > 100
      ? (status.clicks / (status.elapsed_ms / 1000)).toFixed(1)
      : "0.0";

  return (
    <section className="rounded-2xl p-4 bg-white dark:bg-slate-900/90 border border-slate-200/80 dark:border-white/[0.07] shadow-sm">
      <div className="grid grid-cols-3 gap-2 mb-3.5">
        <div className="rounded-xl px-3 py-2 bg-slate-50 dark:bg-slate-950/60 border border-slate-200/60 dark:border-white/[0.04]">
          <div className="text-[11px] font-medium text-slate-500 dark:text-slate-400">
            {t("clicks")}
          </div>
          <div className="text-lg font-bold tabular-nums text-slate-900 dark:text-white">
            {status.clicks.toLocaleString()}
          </div>
        </div>
        <div className="rounded-xl px-3 py-2 bg-slate-50 dark:bg-slate-950/60 border border-slate-200/60 dark:border-white/[0.04]">
          <div className="text-[11px] font-medium text-slate-500 dark:text-slate-400">
            {t("elapsed")}
          </div>
          <div className="text-lg font-bold tabular-nums text-slate-900 dark:text-white">
            {formatElapsed(status.elapsed_ms)}
          </div>
        </div>
        <div className="rounded-xl px-3 py-2 bg-slate-50 dark:bg-slate-950/60 border border-slate-200/60 dark:border-white/[0.04]">
          <div className="text-[11px] font-medium text-slate-500 dark:text-slate-400">
            {t("cps")}
          </div>
          <div className="text-lg font-bold tabular-nums text-indigo-600 dark:text-indigo-400">
            {cps}
          </div>
        </div>
      </div>

      <button
        onClick={onToggle}
        className={`w-full py-3 px-4 rounded-xl font-semibold text-sm flex items-center justify-between transition-all duration-200 cursor-pointer shadow-md ${
          status.running
            ? "bg-gradient-to-r from-rose-500 to-red-600 hover:from-rose-600 hover:to-red-700 text-white shadow-rose-500/25"
            : "bg-gradient-to-r from-indigo-500 to-violet-600 hover:from-indigo-600 hover:to-violet-700 text-white shadow-indigo-500/25"
        }`}
      >
        <span className="flex items-center gap-2.5">
          {status.running ? (
            <Square className="w-4 h-4 fill-current" />
          ) : (
            <Play className="w-4 h-4 fill-current" />
          )}
          <span>{status.running ? t("stopClicking") : t("startClicking")}</span>
        </span>

        {hotkey && (
          <span className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-black/20 text-xs font-medium text-white/90">
            <Keyboard className="w-3.5 h-3.5 opacity-80" />
            <span>
              {hotkeyMode === "hold" ? `${t("holdHotkeyHint")}: ` : ""}
              {hotkey}
            </span>
          </span>
        )}
      </button>
    </section>
  );
}
