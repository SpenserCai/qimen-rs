import { test, expect, openChart, expectNoOverflow } from "./helpers";

for (const viewport of [
  { width: 1051, height: 700 },
  { width: 1280, height: 720 },
  { width: 1366, height: 768 },
  { width: 1440, height: 900 },
  { width: 1920, height: 1080 },
]) {
  test(`桌面 ${viewport.width}×${viewport.height} 完整盘面与操作一屏展示`, async ({
    page,
  }, testInfo) => {
    await page.setViewportSize(viewport);
    await openChart(page);
    await page.evaluate(() => document.fonts.ready);
    await expectNoOverflow(page);

    const geometry = await page.evaluate(() => {
      const selectors = [
        ".input-panel",
        ".chart-workspace",
        ".detail-panel",
        ".workspace-footer",
        ".example-button",
        ".chart-actions",
        ".palace-grid",
      ];
      return {
        height: document.documentElement.scrollHeight,
        panels: selectors.map((selector) => {
          const element = document.querySelector<HTMLElement>(selector)!;
          const rect = element.getBoundingClientRect();
          return {
            selector,
            top: rect.top,
            bottom: rect.bottom,
            left: rect.left,
            right: rect.right,
            content: element.scrollHeight,
            height: element.clientHeight,
          };
        }),
        cells: [...document.querySelectorAll<HTMLElement>(".palace-cell")].map(
          (element) => ({
            id: element.id,
            content: element.scrollHeight,
            height: element.clientHeight,
          }),
        ),
      };
    });
    expect(geometry.height, JSON.stringify(geometry)).toBeLessThanOrEqual(
      viewport.height,
    );
    for (const panel of geometry.panels) {
      expect(panel.top, panel.selector).toBeGreaterThanOrEqual(0);
      expect(panel.bottom, panel.selector).toBeLessThanOrEqual(viewport.height);
    }
    const [input, chart, detail] = geometry.panels;
    expect(input.right).toBeLessThan(chart.left);
    expect(chart.right).toBeLessThan(detail.left);
    expect(
      input.content,
      "All input controls fit without an inner scrollbar",
    ).toBeLessThanOrEqual(input.height + 1);
    for (const cell of geometry.cells) {
      expect(cell.content, `${cell.id} vertical content`).toBeLessThanOrEqual(
        cell.height + 1,
      );
    }

    await page.locator("#palace-6").click();
    await page.getByRole("button", { name: /^扩展注记/ }).click();
    while (await page.locator(".annotation-section:not([open])").count()) {
      await page
        .locator(".annotation-section:not([open]) > summary")
        .first()
        .click();
    }
    await expect(page.locator(".annotation-section[open]")).toHaveCount(7);
    await page.locator(".annotation-section").last().scrollIntoViewIfNeeded();
    expect(
      await page.evaluate(() => document.documentElement.scrollHeight),
    ).toBeLessThanOrEqual(viewport.height);
    expect(await page.evaluate(() => window.scrollY)).toBe(0);
    await expect(
      page.getByRole("button", { name: "开始排盘", exact: true }),
    ).toBeInViewport();
    await expect(
      page.getByRole("button", { name: "导出 JSON", exact: true }),
    ).toBeInViewport();
    await page.getByRole("button", { name: "宫位信息", exact: true }).click();
    await expect(
      page.locator(".base-detail .detail-row").first(),
    ).toBeInViewport();
    await page.screenshot({
      path: testInfo.outputPath("desktop.png"),
      fullPage: true,
    });
  });
}

test("较矮窗口保留自然滚动，输入与导出仍可访问", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 600 });
  await openChart(page);
  await expectNoOverflow(page);
  await page
    .getByRole("button", { name: "导出 JSON", exact: true })
    .scrollIntoViewIfNeeded();
  await expect(
    page.getByRole("button", { name: "导出 JSON", exact: true }),
  ).toBeInViewport();
  await page
    .getByRole("button", { name: "开始排盘", exact: true })
    .scrollIntoViewIfNeeded();
  await expect(
    page.getByRole("button", { name: "开始排盘", exact: true }),
  ).toBeInViewport();
});
