import { test as base, expect, type Page } from "@playwright/test";
import type { Chart, ChartRequest } from "@spensercai/qimen-wasm";

/** All tests calculate in the browser using the actual published WASM package. */
export const test = base.extend({
  page: async ({ page }, providePage, testInfo) => {
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(error.message));
    page.on("console", (message) => {
      if (message.type() !== "error") return;
      const expectedNetworkFailure = testInfo.annotations.some(
        ({ type }) => type === "expected-network-failure",
      );
      if (
        expectedNetworkFailure &&
        message.text().startsWith("Failed to load resource:") &&
        message.location().url.includes("/wasm/") &&
        message.location().url.endsWith("/qimen_wasm_bg.wasm")
      )
        return;
      errors.push(message.text());
    });
    await providePage(page);
    expect(
      errors,
      "Uncaught browser errors and unexpected console errors",
    ).toEqual([]);
  },
});

export { expect };

export const REFERENCE_REQUEST: ChartRequest = {
  year: 2026,
  month: 9,
  day: 18,
  hour: 18,
  minute: 0,
  second: 0,
  utc_offset_minutes: 480,
  day_boundary: "zi_start",
  extensions: {
    hidden_stems: "duty_door_hour_stem_with_center_fallback",
    strength: "classical_stars_and_five_elements",
    growth_stages: "yang_forward_yin_reverse_fire_earth",
    punishments: "six_instrument_branches",
    tombs: "growth_stage_fire_earth",
    day_horse: "day_branch_three_harmony",
    door_pressure: "door_controls_palace",
  },
};

export function sharedPath(request: ChartRequest): string {
  return `/#${new URLSearchParams({ q: JSON.stringify(request) })}`;
}

export async function ready(page: Page): Promise<void> {
  await expect(
    page.getByRole("button", { name: "开始排盘", exact: true }),
  ).toBeEnabled();
  await expect(page.locator(".chart-stage")).toHaveAttribute(
    "aria-busy",
    "false",
  );
  await expect(page.locator(".palace-grid button")).toHaveCount(9);
}

export async function openChart(
  page: Page,
  request: ChartRequest = REFERENCE_REQUEST,
): Promise<void> {
  await page.goto(sharedPath(request));
  const date = `${String(request.year).padStart(4, "0")}-${String(request.month).padStart(2, "0")}-${String(request.day).padStart(2, "0")}`;
  await expect(page.getByLabel("公历日期", { exact: true })).toHaveValue(date);
  await ready(page);
}

export async function exportedChart(page: Page): Promise<Chart> {
  const downloadPromise = page.waitForEvent("download");
  await page.getByRole("button", { name: "导出 JSON", exact: true }).click();
  const download = await downloadPromise;
  expect(download.suggestedFilename()).toMatch(/\.json$/);
  const stream = await download.createReadStream();
  expect(stream).not.toBeNull();
  const chunks: Buffer[] = [];
  for await (const chunk of stream!) chunks.push(Buffer.from(chunk));
  return JSON.parse(Buffer.concat(chunks).toString("utf8")) as Chart;
}

export async function expectNoOverflow(page: Page): Promise<void> {
  const dimensions = await page.evaluate(() => ({
    viewport: document.documentElement.clientWidth,
    document: document.documentElement.scrollWidth,
    body: document.body.scrollWidth,
  }));
  expect(dimensions.document).toBeLessThanOrEqual(dimensions.viewport);
  expect(dimensions.body).toBeLessThanOrEqual(dimensions.viewport);
  const palaces = await page
    .locator(".palace-grid button")
    .evaluateAll((buttons) =>
      buttons.map((button) => ({
        id: button.id,
        left: button.getBoundingClientRect().left,
        right: button.getBoundingClientRect().right,
        content: button.scrollWidth,
        width: button.clientWidth,
      })),
    );
  for (const palace of palaces) {
    expect(
      palace.left,
      `${palace.id} must remain on screen`,
    ).toBeGreaterThanOrEqual(0);
    expect(
      palace.right,
      `${palace.id} must remain on screen`,
    ).toBeLessThanOrEqual(dimensions.viewport);
    expect(
      palace.content,
      `${palace.id} content must not be clipped`,
    ).toBeLessThanOrEqual(palace.width + 1);
  }
}
