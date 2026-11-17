import { Crosshair, MapPin } from "lucide-react";
import type { CoordPayload, PositionMode } from "../types";
import type { TranslationKey } from "../i18n/translations";

interface PositionCardProps {
  mode: PositionMode;
  x: number;
  y: number;
  picking: boolean;
  liveCoord: CoordPayload | null;
  disabled: boolean;
  onChangeMode: (mode: PositionMode) => void;
  onChangeCoords: (x: number, y: number) => void;
  onStartPick: () => void;
  t: (key: TranslationKey) => string;
}

export function PositionCard({
  mode,
  x,
  y,
  picking,
  liveCoord,
  disabled,
  onChangeMode,
  onChangeCoords,
  onStartPick,
  t,
}: PositionCardProps) {
  return (
    <section className="rounded-2xl p-4 bg-white dark:bg-slate-900/90 border border-slate-200/80 dark:border-white/[0.07] shadow-sm space-y-3">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
          <MapPin className="w-3.5 h-3.5 text-indigo-500" />
          <span>{t("cursorPosition")}</span>
        </div>

        <div className="flex p-0.5 rounded-lg bg-slate-100 dark:bg-slate-950/70 border border-slate-200/60 dark:border-white/[0.04]">
          <button
            disabled={disabled}
            onClick={() => onChangeMode("current")}
            className={`px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer disabled:opacity-50 ${
              mode === "current"
                ? "bg-white dark:bg-slate-800 text-slate-900 dark:text-white shadow-sm"
                : "text-slate-500 dark:text-slate-400"
            }`}
          >
            {t("currentCursor")}
          </button>
          <button
            disabled={disabled}
            onClick={() => onChangeMode("fixed")}
            className={`px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer disabled:opacity-50 ${
              mode === "fixed"
                ? "bg-white dark:bg-slate-800 text-slate-900 dark:text-white shadow-sm"
                : "text-slate-500 dark:text-slate-400"
            }`}
          >
            {t("fixedLocation")}
          </button>
        </div>
      </div>

      {mode === "fixed" && (
        <div className="flex items-center gap-2 pt-1">
          <label className="flex items-center gap-1.5 flex-1 rounded-xl px-3 py-2 bg-slate-50 dark:bg-slate-950/60 border border-slate-200/70 dark:border-white/[0.06] focus-within:border-indigo-500">
            <span className="text-xs font-semibold text-slate-400">X</span>
            <input
              type="number"
              disabled={disabled || picking}
              value={picking && liveCoord ? liveCoord.x : x}
              onChange={(e) => onChangeCoords(Number(e.target.value) || 0, y)}
              className="w-full bg-transparent text-sm font-bold tabular-nums text-slate-900 dark:text-white focus:outline-none"
            />
          </label>

          <label className="flex items-center gap-1.5 flex-1 rounded-xl px-3 py-2 bg-slate-50 dark:bg-slate-950/60 border border-slate-200/70 dark:border-white/[0.06] focus-within:border-indigo-500">
            <span className="text-xs font-semibold text-slate-400">Y</span>
            <input
              type="number"
              disabled={disabled || picking}
              value={picking && liveCoord ? liveCoord.y : y}
              onChange={(e) => onChangeCoords(x, Number(e.target.value) || 0)}
              className="w-full bg-transparent text-sm font-bold tabular-nums text-slate-900 dark:text-white focus:outline-none"
            />
          </label>

          <button
            disabled={disabled}
            onClick={onStartPick}
            className={`px-3 py-2 rounded-xl text-xs font-semibold flex items-center gap-1.5 transition-all cursor-pointer disabled:opacity-50 shrink-0 ${
              picking
                ? "bg-amber-500 text-white animate-pulse"
                : "bg-indigo-500/10 text-indigo-600 dark:text-indigo-400 hover:bg-indigo-500/20 border border-indigo-500/20"
            }`}
          >
            <Crosshair className="w-3.5 h-3.5" />
            <span>{picking ? "..." : t("pickLocation")}</span>
          </button>
        </div>
      )}

      {mode === "fixed" && picking && (
        <div className="text-[11px] font-medium text-amber-600 dark:text-amber-400">
          {t("pickingPrompt")}
        </div>
      )}
    </section>
  );
}
