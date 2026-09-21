import type { ChartRequest, DayBoundary, ExtensionOptions } from "./types";

export interface ChartDraft {
  date: string;
  time: string;
  utcOffset: string;
  dayBoundary: DayBoundary;
  extensions: ExtensionOptions;
}

export type RequestValidation =
  { ok: true; request: ChartRequest } | { ok: false; error: string };

export const DEFAULT_EXTENSIONS = {
  hidden_stems: "duty_door_hour_stem_with_center_fallback",
  strength: "classical_stars_and_five_elements",
  growth_stages: "yang_forward_yin_reverse_fire_earth",
  punishments: "six_instrument_branches",
  tombs: "growth_stage_fire_earth",
  day_horse: "day_branch_three_harmony",
  door_pressure: "door_controls_palace",
} as const satisfies Required<ExtensionOptions>;

const REQUEST_KEYS = new Set([
  "year",
  "month",
  "day",
  "hour",
  "minute",
  "second",
  "utc_offset_minutes",
  "day_boundary",
  "extensions",
]);

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isIntegerIn(
  value: unknown,
  min: number,
  max: number,
): value is number {
  return (
    typeof value === "number" &&
    Number.isInteger(value) &&
    value >= min &&
    value <= max
  );
}

export function daysInMonth(year: number, month: number): number {
  if (month === 2)
    return year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0) ? 29 : 28;
  return [4, 6, 9, 11].includes(month) ? 30 : 31;
}

/** Validate the public input shape before sending untrusted links to the worker. */
export function validateRequest(value: unknown): RequestValidation {
  if (
    !isRecord(value) ||
    Object.keys(value).some((key) => !REQUEST_KEYS.has(key))
  ) {
    return { ok: false, error: "排盘参数格式无效，请检查分享链接。" };
  }
  if (!isIntegerIn(value.year, 1, 9999))
    return { ok: false, error: "年份须为 1 至 9999 年。" };
  if (!isIntegerIn(value.month, 1, 12))
    return { ok: false, error: "月份须为 1 至 12 月。" };
  if (!isIntegerIn(value.day, 1, daysInMonth(value.year, value.month))) {
    return { ok: false, error: "该公历日期不存在，请检查月份、日期与闰年。" };
  }
  if (!isIntegerIn(value.hour, 0, 23))
    return { ok: false, error: "小时须为 0 至 23。" };
  const minute = value.minute === undefined ? 0 : value.minute;
  const second = value.second === undefined ? 0 : value.second;
  if (!isIntegerIn(minute, 0, 59) || !isIntegerIn(second, 0, 59)) {
    return { ok: false, error: "分钟与秒须为 0 至 59。" };
  }
  const offset =
    value.utc_offset_minutes === undefined ? 480 : value.utc_offset_minutes;
  if (!isIntegerIn(offset, -840, 840))
    return { ok: false, error: "时区须在 UTC−14:00 至 UTC+14:00 之间。" };
  const boundary =
    value.day_boundary === undefined ? "zi_start" : value.day_boundary;
  if (boundary !== "zi_start" && boundary !== "midnight")
    return { ok: false, error: "请选择有效的换日规则。" };
  const extensions: ExtensionOptions = {};
  if (value.extensions !== undefined) {
    if (!isRecord(value.extensions))
      return { ok: false, error: "扩展标注配置无效。" };
    for (const [key, rule] of Object.entries(value.extensions)) {
      if (!Object.hasOwn(DEFAULT_EXTENSIONS, key))
        return { ok: false, error: "分享链接包含不支持的扩展标注。" };
      const extensionKey = key as keyof ExtensionOptions;
      if (rule === null) continue;
      if (
        rule !== DEFAULT_EXTENSIONS[extensionKey] &&
        !(key === "tombs" && rule === "traditional_three_wonders")
      ) {
        return { ok: false, error: "分享链接包含不支持的扩展规则。" };
      }
      // The rule/key pair has been checked against the binding's exact contract.
      Object.assign(extensions, { [key]: rule });
    }
  }
  return {
    ok: true,
    request: {
      year: value.year,
      month: value.month,
      day: value.day,
      hour: value.hour,
      minute,
      second,
      utc_offset_minutes: offset,
      day_boundary: boundary,
      ...(Object.keys(extensions).length ? { extensions } : {}),
    },
  };
}

export function formatOffset(minutes: number): string {
  const absolute = Math.abs(minutes);
  return `${minutes < 0 ? "-" : "+"}${String(Math.floor(absolute / 60)).padStart(2, "0")}:${String(absolute % 60).padStart(2, "0")}`;
}

export function requestToDraft(request: ChartRequest): ChartDraft {
  return {
    date: `${String(request.year).padStart(4, "0")}-${String(request.month).padStart(2, "0")}-${String(request.day).padStart(2, "0")}`,
    time: `${String(request.hour).padStart(2, "0")}:${String(request.minute ?? 0).padStart(2, "0")}:${String(request.second ?? 0).padStart(2, "0")}`,
    utcOffset: formatOffset(request.utc_offset_minutes ?? 480),
    dayBoundary: request.day_boundary ?? "zi_start",
    extensions: { ...request.extensions },
  };
}

export function parseDraft(draft: ChartDraft): RequestValidation {
  const date = /^(\d{4})-(\d{2})-(\d{2})$/.exec(draft.date);
  const time = /^(\d{2}):(\d{2})(?::(\d{2}))?$/.exec(draft.time);
  const offset = /^([+-])(\d{2}):(\d{2})$/.exec(draft.utcOffset);
  if (!date)
    return { ok: false, error: "请输入完整公历日期，格式为 YYYY-MM-DD。" };
  if (!time)
    return { ok: false, error: "请输入完整时间，格式为 HH:mm 或 HH:mm:ss。" };
  if (!offset || Number(offset[3]) > 59)
    return { ok: false, error: "时区格式应为 +08:00 或 -05:00。" };
  return validateRequest({
    year: Number(date[1]),
    month: Number(date[2]),
    day: Number(date[3]),
    hour: Number(time[1]),
    minute: Number(time[2]),
    second: Number(time[3] ?? 0),
    utc_offset_minutes:
      (Number(offset[2]) * 60 + Number(offset[3])) *
      (offset[1] === "-" ? -1 : 1),
    day_boundary: draft.dayBoundary,
    extensions: draft.extensions,
  });
}

/** Move one two-hour period in civil time without the Date constructor's 0–99-year remapping. */
export function stepRequest(
  request: ChartRequest,
  direction: -1 | 1,
): RequestValidation {
  const validated = validateRequest(request);
  if (!validated.ok) return validated;
  const next = { ...validated.request };
  next.hour += direction * 2;
  if (next.hour < 0) {
    next.hour += 24;
    next.day -= 1;
    if (next.day === 0) {
      next.month -= 1;
      if (next.month === 0) {
        next.month = 12;
        next.year -= 1;
      }
      next.day = daysInMonth(next.year, next.month);
    }
  } else if (next.hour > 23) {
    next.hour -= 24;
    next.day += 1;
    if (next.day > daysInMonth(next.year, next.month)) {
      next.day = 1;
      next.month += 1;
      if (next.month === 13) {
        next.month = 1;
        next.year += 1;
      }
    }
  }
  if (next.year < 1 || next.year > 9999)
    return { ok: false, error: "已到达支持的日期边界（公元 1–9999 年）。" };
  return { ok: true, request: next };
}

export function nowRequest(now: Date = new Date(), offset = 480): ChartRequest {
  const local = new Date(now.getTime() + offset * 60_000);
  return {
    year: local.getUTCFullYear(),
    month: local.getUTCMonth() + 1,
    day: local.getUTCDate(),
    hour: local.getUTCHours(),
    minute: local.getUTCMinutes(),
    second: 0,
    utc_offset_minutes: offset,
    day_boundary: "zi_start",
    extensions: {},
  };
}

export function formatCivil(
  request: Pick<
    ChartRequest,
    "year" | "month" | "day" | "hour" | "minute" | "second"
  >,
): string {
  const draft = requestToDraft(request);
  return `${draft.date} ${draft.time}`;
}
