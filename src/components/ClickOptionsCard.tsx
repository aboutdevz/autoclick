import { Mouse, Repeat } from "lucide-react";
import type { ClickType, MouseButton, RepeatMode } from "../types";
import type { TranslationKey } from "../i18n/translations";

interface ClickOptionsCardProps {
  button: MouseButton;
  clickType: ClickType;
  repeatMode: RepeatMode;
  repeatCount: number;
  disabled: boolean;
  onChangeButton: (b: MouseButton) => void;
  onChangeClickType: (ct: ClickType) => void;
  onChangeRepeat: (mode: RepeatMode, count: number) => void;
  t: (key: TranslationKey) => string;
}

export function ClickOptionsCard({
  button,
  clickType,
  repeatMode,
  repeatCount,
  disabled,
  onChangeButton,
  onChangeClickType,
  onChangeRepeat,
  t,
}: ClickOptionsCardProps) {
  const buttons: { value: MouseButton; label: string }[] = [
    { value: "left", label: t("left") },
    { value: "right", label: t("right") },
    { value: "middle", label: t("middle") },
  ];

  const types: { value: ClickType; label: string }[] = [
    { value: "single", label: t("single") },
    { value: "double", label: t("double") },
    { value: "hold", label: t("hold") },
  ];

  return (
    <section className="rounded-2xl p-4 bg-white dark:bg-slate-900/90 border border-slate-200/80 dark:border-white/[0.07] shadow-sm space-y-3.5">
      <div className="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
        <Mouse className="w-3.5 h-3.5 text-indigo-500" />
        <span>{t("clickOptions")}</span>
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div>
          <div className="text-[11px] font-medium text-slate-500 dark:text-slate-400 mb-1.5">
            {t("mouseButton")}
          </div>
          <div className="grid grid-cols-3 p-1 rounded-xl bg-slate-100 dark:bg-slate-950/70 border border-slate-200/60 dark:border-white/[0.04]">
            {buttons.map((b) => (
              <button
                key={b.value}
                disabled={disabled}
                onClick={() => onChangeButton(b.value)}
                className={`py-1.5 rounded-lg text-xs font-medium transition-all cursor-pointer disabled:opacity-50 ${
                  button === b.value
                    ? "bg-white dark:bg-slate-800 text-slate-900 dark:text-white shadow-sm"
                    : "text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200"
                }`}
              >
                {b.label}
              </button>
            ))}
          </div>
        </div>

        <div>
          <div className="text-[11px] font-medium text-slate-500 dark:text-slate-400 mb-1.5">
            {t("clickType")}
          </div>
          <div className="grid grid-cols-3 p-1 rounded-xl bg-slate-100 dark:bg-slate-950/70 border border-slate-200/60 dark:border-white/[0.04]">
            {types.map((ct) => (
              <button
                key={ct.value}
                disabled={disabled}
                onClick={() => onChangeClickType(ct.value)}
                className={`py-1.5 rounded-lg text-xs font-medium transition-all cursor-pointer disabled:opacity-50 ${
                  clickType === ct.value
                    ? "bg-white dark:bg-slate-800 text-slate-900 dark:text-white shadow-sm"
                    : "text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200"
                }`}
              >
                {ct.label}
              </button>
            ))}
          </div>
        </div>
      </div>

      <div className="pt-1 border-t border-slate-100 dark:border-white/[0.05] flex items-center justify-between gap-3">
        <div className="flex items-center gap-2 text-xs font-medium text-slate-600 dark:text-slate-300">
          <Repeat className="w-3.5 h-3.5 text-slate-400" />
          <span>{t("repeat")}</span>
        </div>

        <div className="flex items-center gap-2">
          <div className="flex p-0.5 rounded-lg bg-slate-100 dark:bg-slate-950/70 border border-slate-200/60 dark:border-white/[0.04]">
            <button
              disabled={disabled}
              onClick={() => onChangeRepeat("infinite", repeatCount)}
              className={`px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer disabled:opacity-50 ${
                repeatMode === "infinite"
                  ? "bg-white dark:bg-slate-800 text-slate-900 dark:text-white shadow-sm"
                  : "text-slate-500 dark:text-slate-400"
              }`}
            >
              {t("infinite")}
            </button>
            <button
              disabled={disabled}
              onClick={() => onChangeRepeat("count", repeatCount)}
              className={`px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer disabled:opacity-50 ${
                repeatMode === "count"
                  ? "bg-white dark:bg-slate-800 text-slate-900 dark:text-white shadow-sm"
                  : "text-slate-500 dark:text-slate-400"
              }`}
            >
              {t("fixedCount")}
            </button>
          </div>

          {repeatMode === "count" && (
            <input
              type="number"
              min={1}
              max={9999999}
              disabled={disabled}
              value={repeatCount}
              onChange={(e) =>
                onChangeRepeat("count", Math.max(1, Number(e.target.value) || 1))
              }
              className="w-20 text-center py-1 px-2 rounded-lg bg-slate-50 dark:bg-slate-950/70 border border-slate-200/70 dark:border-white/[0.08] text-xs font-bold tabular-nums text-slate-900 dark:text-white focus:outline-none focus:border-indigo-500"
            />
          )}
        </div>
      </div>
    </section>
  );
}
