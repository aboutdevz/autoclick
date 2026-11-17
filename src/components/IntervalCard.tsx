import { Clock, Sparkles } from "lucide-react";
import type { TranslationKey } from "../i18n/translations";

interface IntervalCardProps {
  intervalMs: number;
  jitterEnabled: boolean;
  jitterMs: number;
  disabled: boolean;
  onChangeInterval: (ms: number) => void;
  onChangeJitter: (enabled: boolean, ms: number) => void;
  t: (key: TranslationKey) => string;
}

const PRESETS = [
  { label: "10ms", ms: 10 },
  { label: "50ms", ms: 50 },
  { label: "100ms", ms: 100 },
  { label: "500ms", ms: 500 },
  { label: "1s", ms: 1000 },
];

export function IntervalCard({
  intervalMs,
  jitterEnabled,
  jitterMs,
  disabled,
  onChangeInterval,
  onChangeJitter,
  t,
}: IntervalCardProps) {
  const safeMs = Math.max(1, intervalMs);
  const hours = Math.floor(safeMs / 3_600_000);
  const mins = Math.floor((safeMs % 3_600_000) / 60_000);
  const secs = Math.floor((safeMs % 60_000) / 1000);
  const ms = safeMs % 1000;

  const updatePart = (h: number, m: number, s: number, milli: number) => {
    const total = Math.max(
      1,
      Math.max(0, h) * 3_600_000 +
        Math.max(0, m) * 60_000 +
        Math.max(0, s) * 1000 +
        Math.max(0, milli)
    );
    onChangeInterval(total);
  };

  const minRange = Math.max(1, safeMs - jitterMs);
  const maxRange = safeMs + jitterMs;

  return (
    <section className="rounded-2xl p-4 bg-white dark:bg-slate-900/90 border border-slate-200/80 dark:border-white/[0.07] shadow-sm space-y-3">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
          <Clock className="w-3.5 h-3.5 text-indigo-500" />
          <span>{t("clickInterval")}</span>
        </div>

        <div className="flex items-center gap-1">
          {PRESETS.map((p) => (
            <button
              key={p.ms}
              disabled={disabled}
              onClick={() => onChangeInterval(p.ms)}
              className={`px-2 py-0.5 rounded-md text-[11px] font-medium transition-colors cursor-pointer disabled:opacity-50 ${
                safeMs === p.ms
                  ? "bg-indigo-500 text-white"
                  : "bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700"
              }`}
            >
              {p.label}
            </button>
          ))}
        </div>
      </div>

      <div className="grid grid-cols-4 gap-2">
        {[
          { label: t("hours"), val: hours, max: 99, onChange: (v: number) => updatePart(v, mins, secs, ms) },
          { label: t("mins"), val: mins, max: 59, onChange: (v: number) => updatePart(hours, v, secs, ms) },
          { label: t("secs"), val: secs, max: 59, onChange: (v: number) => updatePart(hours, mins, v, ms) },
          { label: t("ms"), val: ms, max: 999, onChange: (v: number) => updatePart(hours, mins, secs, v) },
        ].map((item) => (
          <label
            key={item.label}
            className="flex flex-col items-center rounded-xl px-2 py-2 bg-slate-50 dark:bg-slate-950/60 border border-slate-200/70 dark:border-white/[0.06] focus-within:border-indigo-500 transition-colors"
          >
            <input
              type="number"
              min={0}
              max={item.max}
              disabled={disabled}
              value={item.val}
              onChange={(e) => item.onChange(Number(e.target.value) || 0)}
              className="w-full text-center bg-transparent text-base font-bold tabular-nums text-slate-900 dark:text-white focus:outline-none disabled:opacity-50"
            />
            <span className="text-[10px] font-medium text-slate-400 dark:text-slate-500 mt-0.5">
              {item.label}
            </span>
          </label>
        ))}
      </div>

      <div className="pt-1 border-t border-slate-100 dark:border-white/[0.05]">
        <div className="flex items-center justify-between">
          <button
            type="button"
            disabled={disabled}
            onClick={() => onChangeJitter(!jitterEnabled, jitterMs)}
            className="flex items-center gap-2 text-xs font-medium text-slate-700 dark:text-slate-300 cursor-pointer disabled:opacity-50"
          >
            <Sparkles className={`w-3.5 h-3.5 ${jitterEnabled ? "text-amber-500" : "text-slate-400"}`} />
            <span>{t("randomJitter")}</span>
          </button>

          <div className="flex items-center gap-2">
            {jitterEnabled && (
              <div className="flex items-center gap-1 bg-slate-100 dark:bg-slate-950/70 px-2 py-0.5 rounded-lg border border-slate-200/70 dark:border-white/[0.06]">
                <span className="text-[11px] text-slate-400">±</span>
                <input
                  type="number"
                  min={0}
                  max={5000}
                  disabled={disabled}
                  value={jitterMs}
                  onChange={(e) =>
                    onChangeJitter(true, Math.max(0, Number(e.target.value) || 0))
                  }
                  className="w-12 text-center bg-transparent text-xs font-semibold tabular-nums text-slate-900 dark:text-white focus:outline-none"
                />
                <span className="text-[11px] text-slate-400">ms</span>
              </div>
            )}

            <button
              type="button"
              role="switch"
              aria-checked={jitterEnabled}
              disabled={disabled}
              onClick={() => onChangeJitter(!jitterEnabled, jitterMs)}
              className={`w-9 h-5 rounded-full transition-colors p-0.5 cursor-pointer disabled:opacity-50 ${
                jitterEnabled ? "bg-indigo-500" : "bg-slate-300 dark:bg-slate-700"
              }`}
            >
              <div
                className={`w-4 h-4 rounded-full bg-white shadow transition-transform ${
                  jitterEnabled ? "translate-x-4" : "translate-x-0"
                }`}
              />
            </button>
          </div>
        </div>

        {jitterEnabled && (
          <div className="mt-1.5 text-[11px] text-slate-500 dark:text-slate-400">
            {t("jitterRange")}:{" "}
            <span className="font-semibold text-slate-700 dark:text-slate-200 tabular-nums">
              {minRange}–{maxRange} ms
            </span>
          </div>
        )}
      </div>
    </section>
  );
}
