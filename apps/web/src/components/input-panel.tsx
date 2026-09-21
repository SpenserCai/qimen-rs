"use client";

import {
  CalendarDays,
  ChevronDown,
  Clock3,
  Compass,
  Globe2,
  Info,
  SlidersHorizontal,
} from "lucide-react";
import type { ExtensionOptions } from "@spensercai/qimen-wasm";
import type { ChartDraft } from "@/lib/qimen/request";

const TOGGLES = [
  {
    key: "hidden_stems",
    label: "暗干",
    rule: "duty_door_hour_stem_with_center_fallback",
  },
  { key: "strength", label: "旺衰", rule: "classical_stars_and_five_elements" },
  {
    key: "growth_stages",
    label: "十二长生",
    rule: "yang_forward_yin_reverse_fire_earth",
  },
  { key: "punishments", label: "六仪击刑", rule: "six_instrument_branches" },
  { key: "day_horse", label: "日马", rule: "day_branch_three_harmony" },
  { key: "door_pressure", label: "门迫", rule: "door_controls_palace" },
] as const;

interface InputPanelProps {
  draft: ChartDraft | null;
  busy: boolean;
  error: string | null;
  onChange: (draft: ChartDraft) => void;
  onSubmit: () => void;
  onNow: () => void;
  onExample: () => void;
}

export function InputPanel({
  draft,
  busy,
  error,
  onChange,
  onSubmit,
  onNow,
  onExample,
}: InputPanelProps) {
  const update = (patch: Partial<ChartDraft>) => {
    if (draft) onChange({ ...draft, ...patch });
  };
  const extension = (key: keyof ExtensionOptions, value: string | null) => {
    if (draft) update({ extensions: { ...draft.extensions, [key]: value } });
  };

  return (
    <aside className="input-panel panel" aria-label="起局参数">
      <details className="input-disclosure" open>
        <summary className="panel-heading input-heading">
          <span>
            <span className="section-eyebrow">SETTING THE MOMENT</span>
            <span className="heading-text">起局时间</span>
          </span>
          <ChevronDown
            className="disclosure-chevron"
            size={18}
            aria-hidden="true"
          />
        </summary>
        <form
          className="input-form"
          onSubmit={(event) => {
            event.preventDefault();
            onSubmit();
          }}
        >
          <fieldset disabled={!draft || busy} className="form-fields">
            <legend className="sr-only">公历日期与时间</legend>
            <label className="field-label" htmlFor="qimen-date">
              公历日期
            </label>
            <div className="field-shell">
              <CalendarDays size={17} aria-hidden="true" />
              <input
                id="qimen-date"
                name="date"
                type="date"
                min="0001-01-01"
                max="9999-12-31"
                required
                value={draft?.date ?? ""}
                onChange={(event) => update({ date: event.target.value })}
              />
            </div>
            <label className="field-label" htmlFor="qimen-time">
              当地时间
            </label>
            <div className="time-row">
              <div className="field-shell">
                <Clock3 size={17} aria-hidden="true" />
                <input
                  id="qimen-time"
                  name="time"
                  type="time"
                  step="1"
                  required
                  value={draft?.time ?? ""}
                  onChange={(event) => update({ time: event.target.value })}
                />
              </div>
              <button
                type="button"
                className="secondary-button now-button"
                onClick={onNow}
              >
                此刻
              </button>
            </div>
            <label className="field-label" htmlFor="qimen-offset">
              时区偏移
            </label>
            <div className="field-shell">
              <Globe2 size={17} aria-hidden="true" />
              <span className="offset-prefix">UTC</span>
              <input
                id="qimen-offset"
                name="utcOffset"
                type="text"
                inputMode="text"
                placeholder="+08:00"
                required
                pattern="([+]|-)[0-9]{2}:[0-9]{2}"
                maxLength={6}
                value={draft?.utcOffset ?? "+08:00"}
                onChange={(event) => update({ utcOffset: event.target.value })}
                aria-describedby="offset-help"
              />
            </div>
            <p id="offset-help" className="field-hint">
              北京时间为 +08:00 · 范围 ±14:00
            </p>
            <div className="method-chip">
              <Compass size={17} aria-hidden="true" />
              <span>时家拆补转盘</span>
              <span className="tiny-tag">定式</span>
            </div>
            <fieldset className="boundary-field">
              <legend className="field-label">换日规则</legend>
              <div className="segmented-control">
                <label
                  className={
                    draft?.dayBoundary === "zi_start" ? "selected" : ""
                  }
                >
                  <input
                    type="radio"
                    name="day-boundary"
                    value="zi_start"
                    checked={draft?.dayBoundary === "zi_start"}
                    onChange={() => update({ dayBoundary: "zi_start" })}
                  />
                  <span>子初 · 23时</span>
                </label>
                <label
                  className={
                    draft?.dayBoundary === "midnight" ? "selected" : ""
                  }
                >
                  <input
                    type="radio"
                    name="day-boundary"
                    value="midnight"
                    checked={draft?.dayBoundary === "midnight"}
                    onChange={() => update({ dayBoundary: "midnight" })}
                  />
                  <span>午夜 · 0时</span>
                </label>
              </div>
            </fieldset>
          </fieldset>
          <button
            className="cast-button"
            type="submit"
            disabled={!draft || busy}
          >
            <Compass size={23} aria-hidden="true" />
            <span>{busy ? "天盘运转中" : "开始排盘"}</span>
          </button>
          {error ? (
            <p className="form-error" role="alert">
              {error}
            </p>
          ) : null}
          <fieldset className="extension-fields" disabled={!draft || busy}>
            <legend className="subsection-title">
              <SlidersHorizontal size={15} aria-hidden="true" />
              扩展注记
            </legend>
            <div className="extension-batch">
              <span>按需启用 · 规则可追溯</span>
              <button
                type="button"
                className="text-button"
                onClick={() =>
                  update({
                    extensions: Object.keys(draft?.extensions ?? {}).some(
                      (key) => draft?.extensions[key as keyof ExtensionOptions],
                    )
                      ? {}
                      : {
                          hidden_stems:
                            "duty_door_hour_stem_with_center_fallback",
                          strength: "classical_stars_and_five_elements",
                          growth_stages: "yang_forward_yin_reverse_fire_earth",
                          punishments: "six_instrument_branches",
                          tombs: "growth_stage_fire_earth",
                          day_horse: "day_branch_three_harmony",
                          door_pressure: "door_controls_palace",
                        },
                  })
                }
              >
                {Object.keys(draft?.extensions ?? {}).some(
                  (key) => draft?.extensions[key as keyof ExtensionOptions],
                )
                  ? "全部关闭"
                  : "全部开启"}
              </button>
            </div>
            <div className="extension-toggles">
              {TOGGLES.map(({ key, label, rule }) => (
                <label className="toggle-label" key={key}>
                  <span>{label}</span>
                  <input
                    type="checkbox"
                    checked={Boolean(draft?.extensions?.[key])}
                    onChange={(event) =>
                      extension(key, event.target.checked ? rule : null)
                    }
                  />
                  <span className="switch-track" aria-hidden="true" />
                </label>
              ))}
            </div>
            <label className="field-label tomb-label" htmlFor="tomb-rule">
              入墓规则
            </label>
            <div className="select-shell">
              <select
                id="tomb-rule"
                value={draft?.extensions?.tombs ?? ""}
                onChange={(event) =>
                  extension("tombs", event.target.value || null)
                }
              >
                <option value="">不显示入墓</option>
                <option value="traditional_three_wonders">古典三奇入墓</option>
                <option value="growth_stage_fire_earth">
                  十二长生墓 · 火土同宫
                </option>
              </select>
              <ChevronDown size={15} aria-hidden="true" />
            </div>
            <p className="field-hint">修改注记后，点击「开始排盘」生效。</p>
          </fieldset>
          <div className="input-footnote">
            <Info size={14} aria-hidden="true" />
            <span>按民用时间起局，不自动校正真太阳时。</span>
          </div>
          <button
            type="button"
            className="text-button example-button"
            onClick={onExample}
            disabled={!draft || busy}
          >
            试算示例 · 2026.09.18 18:00<span aria-hidden="true">↗</span>
          </button>
        </form>
      </details>
    </aside>
  );
}
