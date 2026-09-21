import { describe, expect, it } from "vitest";
import {
  loadPreferences,
  savePreferences,
} from "../../src/lib/qimen/preferences";
import {
  DEFAULT_EXTENSIONS,
  validateRequest,
} from "../../src/lib/qimen/request";
import { restoreRequest, serializeRequest } from "../../src/lib/qimen/sharing";

const request = {
  year: 2026,
  month: 9,
  day: 18,
  hour: 18,
  minute: 15,
  utc_offset_minutes: 480,
  extensions: DEFAULT_EXTENSIONS,
};

describe("share links", () => {
  it("preserves the full calculation contract including extension rules", () => {
    const shared = new URL(
      `https://qimen.example/${serializeRequest(request)}`,
    );
    expect(shared.search).toBe("");
    expect(shared.hash).toMatch(/^#q=/);
    expect(restoreRequest(shared.hash)).toEqual(validateRequest(request));
  });

  it("distinguishes a missing share from an invalid share", () => {
    expect(restoreRequest("")).toBeNull();
    expect(restoreRequest("#usage")).toBeNull();
    expect(restoreRequest("#unrelated=value")).toBeNull();
    for (const hash of [
      "#q=",
      "#q=%",
      "#q=null",
      "#q=42",
      "#q=[]",
      "#q={}",
      "#q={}&q={}",
      `#q=${"x".repeat(4097)}`,
    ]) {
      expect(restoreRequest(hash)).toMatchObject({ ok: false });
    }
  });

  it("does not restore calculation input from an HTTP query string", () => {
    expect(
      restoreRequest(`?${new URLSearchParams({ q: JSON.stringify(request) })}`),
    ).toBeNull();
  });

  it("rejects unknown parameters alongside a shared chart", () => {
    expect(
      restoreRequest(
        `${serializeRequest(request)}&redirect=https%3A%2F%2Finvalid.test`,
      ),
    ).toMatchObject({ ok: false });
    expect(
      restoreRequest(`${serializeRequest(request)}&year=2027`),
    ).toMatchObject({ ok: false });
  });

  it("does not trust a link's numeric types or arbitrary input properties", () => {
    const invalid = {
      ...request,
      year: "2026",
      redirect: "https://invalid.test",
    };
    expect(
      restoreRequest(`#${new URLSearchParams({ q: JSON.stringify(invalid) })}`),
    ).toMatchObject({ ok: false });
  });

  it("refuses to produce an invalid share link", () => {
    expect(() => serializeRequest({ ...request, year: 0 })).toThrow();
  });
});

describe("local preferences", () => {
  it("saves only display/calculation preferences, never private date input", () => {
    let stored = "";
    const storage = {
      getItem: () => stored,
      setItem: (_key: string, value: string) => {
        stored = value;
      },
    };
    const preferences = {
      utcOffset: "+08:00",
      dayBoundary: "zi_start",
      extensions: DEFAULT_EXTENSIONS,
      reduceMotion: true,
    } as const;
    expect(savePreferences({ ...preferences, ...request }, storage)).toBe(true);
    expect(loadPreferences(storage)).toEqual(preferences);
    expect(JSON.parse(stored)).not.toHaveProperty("year");
    expect(JSON.parse(stored)).not.toHaveProperty("day");
  });

  it("tolerates disabled storage, invalid JSON, old schemas and invalid saved rules", () => {
    const unavailable = {
      getItem: () => {
        throw new Error("blocked");
      },
      setItem: () => {
        throw new Error("quota");
      },
    };
    expect(loadPreferences(unavailable)).toBeNull();
    expect(
      savePreferences(
        {
          utcOffset: "+08:00",
          dayBoundary: "zi_start",
          extensions: {},
          reduceMotion: false,
        },
        unavailable,
      ),
    ).toBe(false);
    for (const raw of [
      "{",
      "null",
      "[]",
      '{"version":0}',
      JSON.stringify({
        version: 1,
        utcOffset: "+99:00",
        dayBoundary: "zi_start",
        extensions: {},
        reduceMotion: true,
      }),
    ]) {
      expect(
        loadPreferences({ getItem: () => raw, setItem: () => {} }),
      ).toBeNull();
    }
  });
});
