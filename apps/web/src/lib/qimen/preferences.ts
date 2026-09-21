import { parseDraft } from "./request";
import type { DayBoundary, ExtensionOptions } from "./types";

export interface Preferences {
  utcOffset: string;
  dayBoundary: DayBoundary;
  extensions: ExtensionOptions;
  reduceMotion: boolean;
}

type PreferenceStorage = Pick<Storage, "getItem" | "setItem">;
const STORAGE_KEY = "qimen.preferences.v1";

/** Dates and chart results are deliberately excluded from persisted preferences. */
export function loadPreferences(
  storage?: PreferenceStorage,
): Preferences | null {
  try {
    const raw = (storage ?? window.localStorage).getItem(STORAGE_KEY);
    if (!raw) return null;
    const value: unknown = JSON.parse(raw);
    if (typeof value !== "object" || value === null || Array.isArray(value))
      return null;
    const candidate = value as Record<string, unknown>;
    if (
      candidate.version !== 1 ||
      typeof candidate.utcOffset !== "string" ||
      (candidate.dayBoundary !== "midnight" &&
        candidate.dayBoundary !== "zi_start") ||
      typeof candidate.reduceMotion !== "boolean"
    )
      return null;
    const validated = parseDraft({
      date: "2000-01-01",
      time: "12:00",
      utcOffset: candidate.utcOffset,
      dayBoundary: candidate.dayBoundary,
      extensions: candidate.extensions as ExtensionOptions,
    });
    if (!validated.ok) return null;
    return {
      utcOffset: candidate.utcOffset,
      dayBoundary: candidate.dayBoundary,
      extensions: validated.request.extensions ?? {},
      reduceMotion: candidate.reduceMotion,
    };
  } catch {
    return null;
  }
}

/** Private mode, a full quota or disabled storage must not prevent calculation. */
export function savePreferences(
  preferences: Preferences,
  storage?: PreferenceStorage,
): boolean {
  try {
    (storage ?? window.localStorage).setItem(
      STORAGE_KEY,
      JSON.stringify({
        version: 1,
        utcOffset: preferences.utcOffset,
        dayBoundary: preferences.dayBoundary,
        extensions: preferences.extensions,
        reduceMotion: preferences.reduceMotion,
      }),
    );
    return true;
  } catch {
    return false;
  }
}
