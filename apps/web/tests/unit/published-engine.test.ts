import { readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import init, { calculate } from "@spensercai/qimen-wasm";
import { beforeAll, describe, expect, it } from "vitest";
import { DEFAULT_EXTENSIONS } from "../../src/lib/qimen/request";
import { cycleLabel } from "../../src/lib/qimen/labels";

beforeAll(async () => {
  const require = createRequire(import.meta.url);
  const packageDirectory = dirname(
    require.resolve("@spensercai/qimen-wasm/package.json"),
  );
  const bytes = new Uint8Array(
    await readFile(join(packageDirectory, "qimen_wasm_bg.wasm")),
  );
  await init({ module_or_path: bytes });
});

describe("the installed published WASM engine", () => {
  it.each([0, 15])(
    "matches the independently recorded Beijing reference at 18:%i",
    (minute) => {
      // Independent reference transcription: 2026-09-18, 时家拆补转盘,
      // UTC+08:00, 阴遁九局, 丙午 丁酉 乙未 乙酉.
      const chart = calculate({
        year: 2026,
        month: 9,
        day: 18,
        hour: 18,
        minute,
        utc_offset_minutes: 480,
        extensions: DEFAULT_EXTENSIONS,
      });
      expect(
        Object.values(chart.calendar.four_pillars).map(cycleLabel),
      ).toEqual(["丙午", "丁酉", "乙未", "乙酉"]);
      expect(chart).toMatchObject({
        dun: "yin",
        ju: 9,
        leaders: { star: "tian_zhu", door: "jing" },
        horse: { branch: "hai", palace: 6 },
      });
      expect(
        chart.palaces.map((palace) => [
          palace.number,
          palace.earth_stem,
          palace.door,
          palace.deity,
        ]),
      ).toEqual([
        [1, "yi", "kai", "zhi_fu"],
        [2, "bing", "scene", "liu_he"],
        [3, "ding", "sheng", "jiu_di"],
        [4, "gui", "shang", "xuan_wu"],
        [5, "ren", null, null],
        [6, "xin", "jing", "teng_she"],
        [7, "geng", "si", "tai_yin"],
        [8, "ji", "xiu", "jiu_tian"],
        [9, "wu", "du", "bai_hu"],
      ]);
      expect(chart.extensions?.day_horse?.horse).toEqual({
        branch: "si",
        palace: 4,
      });
      expect(
        chart.extensions?.hidden_stems?.palaces.map((palace) => palace.stem),
      ).toEqual([
        "ren",
        "xin",
        "geng",
        "ji",
        "wu",
        "yi",
        "bing",
        "ding",
        "gui",
      ]);
    },
  );

  it.each([
    [1, 1, 1, 15],
    [4, 2, 29, 29],
    [24, 1, 28, 42],
    [1582, 10, 5, 0],
    [1582, 10, 14, 9],
    [9999, 12, 31, 53],
  ])(
    "keeps Gregorian day-cycle continuity at %i-%i-%i",
    (year, month, day, dayCycle) => {
      // Day indices independently obtained by Gregorian ordinal distance from
      // 2000-01-07 (甲子), not by calling another qimen-rs binding.
      const chart = calculate({
        year,
        month,
        day,
        hour: 12,
        utc_offset_minutes: 480,
      });
      expect(chart.calendar.four_pillars.day.index).toBe(dayCycle);
      expect(chart.palaces).toHaveLength(9);
      expect(chart.extensions).toBeUndefined();
    },
  );
});
