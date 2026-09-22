import { test, expect, openChart } from "./helpers";

test("星垣资源与宋体中文在排盘和指南中正常加载", async ({ page }) => {
  const fontRequests: string[] = [];
  page.on("request", (request) => {
    if (/\.woff2?(\?|$)/.test(request.url())) fontRequests.push(request.url());
  });
  const skyResponse = page.waitForResponse(/\/images\/cosmic-void\.webp$/);
  await openChart(page);
  const sky = await skyResponse;
  expect(sky.ok()).toBe(true);
  expect(sky.headers()["content-type"]).toContain("image/webp");
  await page.evaluate(() => document.fonts.ready);

  for (const selector of [
    "body",
    ".cast-button",
    ".field-label",
    ".field-shell input",
    ".select-shell select",
    ".secondary-button",
    ".detail-row dd small",
  ]) {
    await expect(page.locator(selector).first()).toHaveCSS(
      "font-family",
      /^"Noto Serif SC Variable"/,
    );
  }
  expect(
    await page.evaluate(() =>
      document.fonts.check(
        '450 16px "Noto Serif SC Variable"',
        "奇门遁甲公历日期",
      ),
    ),
  ).toBe(true);
  await expect(page.locator(".celestial-backdrop")).toHaveAttribute(
    "aria-hidden",
    "true",
  );
  await expect(page.locator(".celestial-orbits")).toHaveAttribute(
    "aria-hidden",
    "true",
  );

  await page.getByRole("link", { name: "使用指南", exact: true }).click();
  await expect(
    page.getByRole("heading", { name: "使用指南", exact: true }),
  ).toBeVisible();
  await page.evaluate(() => document.fonts.ready);
  await expect(page.locator(".guide-surface p").first()).toHaveCSS(
    "font-family",
    /^"Noto Serif SC Variable"/,
  );
  await expect(page.locator(".celestial-nebula")).toHaveCSS(
    "background-image",
    /cosmic-void\.webp/,
  );
  expect(fontRequests.length).toBeGreaterThan(0);
  expect(
    fontRequests.every(
      (url) => new URL(url).origin === new URL(page.url()).origin,
    ),
  ).toBe(true);
  await page.getByRole("link", { name: "返回排盘", exact: true }).click();
  await expect(page.locator(".palace-grid button")).toHaveCount(9);
});

test("操作按钮默认及悬停状态均有不透底的可读表面", async ({ page }) => {
  await openChart(page);
  const button = page.getByRole("button", { name: "导出 JSON", exact: true });

  async function assertReadableSurface() {
    await expect(async () => {
      const style = await button.evaluate((element) => {
        const css = getComputedStyle(element);
        return { color: css.color, background: css.backgroundColor };
      });
      const channels = (color: string) => color.match(/[\d.]+/g)!.map(Number);
      const luminance = (rgb: number[]) =>
        rgb.slice(0, 3).reduce((sum, channel, index) => {
          const c = channel / 255;
          const linear =
            c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
          return sum + linear * [0.2126, 0.7152, 0.0722][index];
        }, 0);
      const foreground = channels(style.color);
      const background = channels(style.background);
      expect(background[3] ?? 1).toBe(1);
      const light = luminance(foreground);
      const dark = luminance(background);
      expect(
        (Math.max(light, dark) + 0.05) / (Math.min(light, dark) + 0.05),
      ).toBeGreaterThanOrEqual(4.5);
    }).toPass();
  }

  await assertReadableSurface();
  await button.hover();
  await assertReadableSurface();
  await button.focus();
  await expect(button).toBeFocused();
  await expect(button).toHaveCSS("outline-style", "solid");
});
