import { describe, expect, it } from "vitest";
import {
  DEFAULT_EXTENSIONS,
  formatCivil,
  formatOffset,
  nowRequest,
  parseDraft,
  requestToDraft,
  stepRequest,
  validateRequest,
} from "../../src/lib/qimen/request";
import type { ChartRequest } from "../../src/lib/qimen/types";

const reference: ChartRequest = {
  year: 2026,
  month: 9,
  day: 18,
  hour: 18,
  minute: 15,
  utc_offset_minutes: 480,
};

describe("civil input", () => {
  it.each([
    [1, 1, 1, true],
    [4, 2, 29, true],
    [100, 2, 29, false],
    [400, 2, 29, true],
    [1582, 10, 5, true],
    [1582, 10, 14, true],
    [1900, 2, 29, false],
    [2000, 2, 29, true],
    [2100, 2, 29, false],
    [9999, 12, 31, true],
    [0, 1, 1, false],
    [10000, 1, 1, false],
  ])(
    "validates Gregorian %i-%i-%i independently of browser historical cutovers",
    (year, month, day, ok) => {
      expect(validateRequest({ year, month, day, hour: 12 }).ok).toBe(ok);
    },
  );

  it.each([
    { hour: 24 },
    { minute: -1 },
    { second: 60 },
    { year: 2026.1 },
    { year: "2026" },
    { hour: Number.NaN },
    { utc_offset_minutes: 841 },
    { utc_offset_minutes: -841 },
    { day_boundary: "solar" },
    { unsupported: true },
    { extensions: { growth_stages: "wrong" } },
    { extensions: { __unknown: true } },
    { extensions: null },
    { minute: null },
  ])("rejects invalid API input %j", (replacement) => {
    expect(validateRequest({ ...reference, ...replacement }).ok).toBe(false);
  });

  it("normalizes defaults without enabling unrequested annotations", () => {
    expect(validateRequest({ year: 1, month: 1, day: 1, hour: 0 })).toEqual({
      ok: true,
      request: {
        year: 1,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        utc_offset_minutes: 480,
        day_boundary: "zi_start",
      },
    });
    expect(
      validateRequest({
        ...reference,
        extensions: { tombs: "traditional_three_wonders", strength: null },
      }),
    ).toMatchObject({
      ok: true,
      request: { extensions: { tombs: "traditional_three_wonders" } },
    });
  });

  it("round-trips a low year and sub-minute precision without Date remapping", () => {
    const request = {
      ...reference,
      year: 9,
      second: 43,
      utc_offset_minutes: -345,
      extensions: DEFAULT_EXTENSIONS,
    };
    const draft = requestToDraft(request);
    expect(draft).toMatchObject({
      date: "0009-09-18",
      time: "18:15:43",
      utcOffset: "-05:45",
    });
    expect(parseDraft(draft)).toEqual(validateRequest(request));
    expect(formatCivil(request)).toBe("0009-09-18 18:15:43");
  });

  it.each([
    "2026-2-03",
    "2026-02-30",
    "0000-01-01",
    "10000-01-01",
    "not-a-date",
  ])("rejects incomplete or invalid date %s", (date) => {
    expect(parseDraft({ ...requestToDraft(reference), date }).ok).toBe(false);
  });

  it.each(["+08:60", "+14:01", "-14:01", "8:00", "+0800", "UTC+08:00"])(
    "rejects invalid fixed offset %s",
    (utcOffset) => {
      expect(parseDraft({ ...requestToDraft(reference), utcOffset }).ok).toBe(
        false,
      );
    },
  );

  it("accepts second-free time, both offset endpoints and quarter-hour offsets", () => {
    for (const utcOffset of ["+14:00", "-14:00", "+05:45", "+00:00"]) {
      expect(
        parseDraft({ ...requestToDraft(reference), time: "18:00", utcOffset })
          .ok,
      ).toBe(true);
    }
    expect(formatOffset(-30)).toBe("-00:30");
  });

  it("derives the explicit Beijing civil time from an instant", () => {
    expect(nowRequest(new Date("2026-09-18T10:15:43Z"))).toMatchObject({
      year: 2026,
      month: 9,
      day: 18,
      hour: 18,
      minute: 15,
      second: 0,
      utc_offset_minutes: 480,
    });
  });
});

describe("two-hour stepping", () => {
  it.each([
    [
      { year: 4, month: 2, day: 28, hour: 23 },
      1,
      { year: 4, month: 2, day: 29, hour: 1 },
    ],
    [
      { year: 1900, month: 2, day: 28, hour: 23 },
      1,
      { year: 1900, month: 3, day: 1, hour: 1 },
    ],
    [
      { year: 1582, month: 10, day: 4, hour: 23 },
      1,
      { year: 1582, month: 10, day: 5, hour: 1 },
    ],
    [
      { year: 100, month: 1, day: 1, hour: 0 },
      -1,
      { year: 99, month: 12, day: 31, hour: 22 },
    ],
    [
      { year: 2000, month: 3, day: 1, hour: 0 },
      -1,
      { year: 2000, month: 2, day: 29, hour: 22 },
    ],
  ] as const)("moves %j by %i periods", (request, direction, expected) => {
    expect(
      stepRequest(
        {
          ...request,
          minute: 37,
          second: 2,
          utc_offset_minutes: -330,
          extensions: DEFAULT_EXTENSIONS,
        },
        direction,
      ),
    ).toMatchObject({
      ok: true,
      request: {
        ...expected,
        minute: 37,
        second: 2,
        utc_offset_minutes: -330,
        extensions: DEFAULT_EXTENSIONS,
      },
    });
  });

  it("stops at the public calendar endpoints", () => {
    expect(stepRequest({ year: 1, month: 1, day: 1, hour: 0 }, -1).ok).toBe(
      false,
    );
    expect(
      stepRequest({ year: 9999, month: 12, day: 31, hour: 23 }, 1).ok,
    ).toBe(false);
  });
});
