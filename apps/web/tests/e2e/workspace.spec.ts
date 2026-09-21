import {
  test,
  expect,
  REFERENCE_REQUEST,
  exportedChart,
  openChart,
  ready,
  sharedPath,
  expectNoOverflow,
} from "./helpers";

test("真实 WASM 的参考盘、九宫顺序、注记和 JSON 导出一致", async ({ page }) => {
  await openChart(page);
  const summary = page.getByRole("region", { name: "八字与局式" });
  await expect(summary).toContainText("丙午");
  await expect(summary).toContainText("丁酉");
  await expect(summary).toContainText("乙未");
  await expect(summary).toContainText("乙酉");
  await expect(summary).toContainText("阴遁九局");
  await expect(summary).toContainText("值符 天柱");
  await expect(summary).toContainText("值使 惊门");
  expect(
    await page
      .locator(".palace-grid button")
      .evaluateAll((buttons) => buttons.map((button) => button.id)),
  ).toEqual([
    "palace-4",
    "palace-9",
    "palace-2",
    "palace-3",
    "palace-5",
    "palace-7",
    "palace-8",
    "palace-1",
    "palace-6",
  ]);
  await expect(page.locator("#palace-1")).toContainText("天柱");
  await expect(page.locator("#palace-1")).toContainText("开门");
  await expect(page.locator("#palace-6")).toHaveAccessibleName(
    "乾六宫，天芮、天禽，惊门，查看详情",
  );
  await expect(page.locator("#palace-6")).toContainText("惊门");
  await expect(page.locator("#palace-4")).toContainText("日马");

  const at18 = await exportedChart(page);
  // Independently transcribed reference values, in Luo Shu 1–9 order.
  expect(at18.calendar.four_pillars.day).toEqual({
    index: 31,
    stem: "yi",
    branch: "wei",
  });
  expect(at18.leaders).toMatchObject({
    star: "tian_zhu",
    star_palace: 1,
    door: "jing",
    door_palace: 6,
  });
  expect(at18.palaces.map((palace) => palace.earth_stem)).toEqual([
    "yi",
    "bing",
    "ding",
    "gui",
    "ren",
    "xin",
    "geng",
    "ji",
    "wu",
  ]);
  expect(at18.palaces.map((palace) => palace.stars)).toEqual([
    ["tian_zhu"],
    ["tian_fu"],
    ["tian_peng"],
    ["tian_ren"],
    [],
    ["tian_rui", "tian_qin"],
    ["tian_ying"],
    ["tian_xin"],
    ["tian_chong"],
  ]);
  expect(
    at18.palaces.map((palace) => palace.heaven_stems.map((stem) => stem.stem)),
  ).toEqual([
    ["geng"],
    ["gui"],
    ["yi"],
    ["ji"],
    [],
    ["bing", "ren"],
    ["wu"],
    ["xin"],
    ["ding"],
  ]);
  expect(at18.palaces.map((palace) => palace.door)).toEqual([
    "kai",
    "scene",
    "sheng",
    "shang",
    null,
    "jing",
    "si",
    "xiu",
    "du",
  ]);
  expect(at18.palaces.map((palace) => palace.deity)).toEqual([
    "zhi_fu",
    "liu_he",
    "jiu_di",
    "xuan_wu",
    null,
    "teng_she",
    "tai_yin",
    "jiu_tian",
    "bai_hu",
  ]);
  expect(
    at18.extensions?.hidden_stems?.palaces.map((palace) => palace.stem),
  ).toEqual(["ren", "xin", "geng", "ji", "wu", "yi", "bing", "ding", "gui"]);
  expect(at18.extensions?.day_horse?.horse).toEqual({
    branch: "si",
    palace: 4,
  });
  expect(at18.horse).toEqual({ branch: "hai", palace: 6 });
  const detail = page.getByRole("complementary", { name: "宫位详情" });
  await detail.getByRole("button", { name: /^扩展注记/ }).click();
  await detail.getByText("十二长生", { exact: true }).click();
  await expect(
    detail.getByText("戌·墓 / 亥·绝", { exact: true }),
  ).toBeVisible();

  await page.getByLabel("当地时间", { exact: true }).fill("18:15:00");
  await page.getByRole("button", { name: "开始排盘", exact: true }).click();
  await ready(page);
  await expect(summary).toContainText("2026.09.18 18:15");
  const at1815 = await exportedChart(page);
  expect(at1815).toEqual({ ...at18, input: { ...at18.input, minute: 15 } });
});

test("编辑、关闭注记、分享恢复和导出均绑定已显示的盘", async ({
  page,
  context,
  browserName,
}) => {
  if (browserName === "chromium") {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  } else {
    // Safari also needs to support a denied Clipboard API through the manual-copy UI.
    await page.addInitScript(() => {
      Object.defineProperty(navigator, "clipboard", {
        value: {
          writeText: () =>
            Promise.reject(
              new DOMException("Clipboard access denied", "NotAllowedError"),
            ),
        },
      });
    });
  }
  await openChart(page);
  const original = await exportedChart(page);
  await page.getByLabel("公历日期", { exact: true }).fill("2026-09-19");
  await expect(
    page.getByText("起局参数已修改 · 点击「开始排盘」更新结果", {
      exact: true,
    }),
  ).toBeVisible();
  expect(await exportedChart(page)).toEqual(original);
  await page.getByRole("button", { name: "分享此局", exact: true }).click();
  const copied = browserName === "chromium";
  await expect(
    page.getByText(
      copied
        ? "分享链接已复制，链接包含此局时间和规则。"
        : "浏览器未允许自动复制，请在下方手动复制。",
      { exact: true },
    ),
  ).toBeVisible();
  const share = copied
    ? await page.evaluate(() => navigator.clipboard.readText())
    : await page.getByLabel("选中并复制以下内容", { exact: true }).inputValue();
  const shareUrl = new URL(share);
  expect(shareUrl.search).toBe("");
  const shared = JSON.parse(
    new URLSearchParams(shareUrl.hash.slice(1)).get("q")!,
  );
  expect(shared).toEqual(REFERENCE_REQUEST);
  // Force a document request so we can verify that chart inputs stay in the fragment.
  await page.goto("/guide");
  const navigation = page.waitForRequest(
    (request) =>
      request.isNavigationRequest() && request.resourceType() === "document",
  );
  await page.goto(share);
  expect((await navigation).url()).not.toContain("q=");
  await ready(page);
  expect(await exportedChart(page)).toEqual(original);
  await expect(page.getByLabel("公历日期", { exact: true })).toHaveValue(
    "2026-09-18",
  );

  await page.getByRole("checkbox", { name: "日马", exact: true }).uncheck();
  await page.getByRole("button", { name: "开始排盘", exact: true }).click();
  await ready(page);
  const withoutDayHorse = await exportedChart(page);
  expect(withoutDayHorse.extensions?.day_horse).toBeUndefined();
  expect(withoutDayHorse.palaces).toEqual(original.palaces);
  await expect(page.locator("#palace-4")).not.toContainText("日马");
});

test("日期边界、格里高利历连续性和低年份跨日步进", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  for (const [year, month, day, dayIndex] of [
    [1, 1, 1, 15],
    [1582, 10, 5, 0],
    [9999, 12, 31, 53],
  ]) {
    await openChart(page, { year, month, day, hour: 12 });
    const chart = await exportedChart(page);
    expect(chart.input).toMatchObject({ year, month, day, hour: 12 });
    expect(chart.calendar.four_pillars.day.index).toBe(dayIndex);
    expect(chart.palaces).toHaveLength(9);
  }

  await openChart(page, { year: 4, month: 2, day: 29, hour: 23, minute: 30 });
  await page.getByRole("button", { name: "下一时辰", exact: true }).click();
  await ready(page);
  await expect(page.getByLabel("公历日期", { exact: true })).toHaveValue(
    "0004-03-01",
  );
  await expect(page.getByLabel("当地时间", { exact: true })).toHaveValue(
    "01:30:00",
  );
  expect((await exportedChart(page)).input).toMatchObject({
    year: 4,
    month: 3,
    day: 1,
    hour: 1,
    minute: 30,
  });
  await page.getByRole("button", { name: "上一时辰", exact: true }).click();
  await ready(page);
  await expect(page.getByLabel("公历日期", { exact: true })).toHaveValue(
    "0004-02-29",
  );
  await openChart(page, { year: 1, month: 1, day: 1, hour: 0 });
  await page.getByRole("button", { name: "上一时辰", exact: true }).click();
  await expect(
    page.getByRole("complementary", { name: "起局参数" }).getByRole("alert"),
  ).toContainText("已到达支持的日期边界");
  expect((await exportedChart(page)).input).toMatchObject({
    year: 1,
    month: 1,
    day: 1,
    hour: 0,
  });
});

test("无效日期、时区及损坏分享链接提供反馈，不产生伪结果", async ({ page }) => {
  await page.goto(sharedPath({ year: 2026, month: 2, day: 30, hour: 18 }));
  await expect(
    page.getByRole("complementary", { name: "起局参数" }).getByRole("alert"),
  ).toContainText("该公历日期不存在");
  await expect(page.locator(".palace-grid button")).toHaveCount(0);
  await expect(
    page.getByRole("button", { name: "导出 JSON", exact: true }),
  ).toBeDisabled();
  await page.goto("/#q=%7Bbroken");
  await expect(
    page.getByRole("complementary", { name: "起局参数" }).getByRole("alert"),
  ).toContainText("分享链接损坏");
  await openChart(page);
  const original = await exportedChart(page);
  await page.getByLabel("时区偏移", { exact: true }).fill("+14:01");
  await page.getByRole("button", { name: "开始排盘", exact: true }).click();
  await expect(
    page.getByRole("complementary", { name: "起局参数" }).getByRole("alert"),
  ).toContainText("时区须在");
  expect(await exportedChart(page)).toEqual(original);
  await page.getByLabel("时区偏移", { exact: true }).fill("-05:00");
  await page.getByLabel("公历日期", { exact: true }).fill("");
  await page.getByRole("button", { name: "此刻", exact: true }).click();
  await expect(page.getByLabel("时区偏移", { exact: true })).toHaveValue(
    "-05:00",
  );
  await expect(page.getByLabel("公历日期", { exact: true })).not.toHaveValue(
    "",
  );
});

test("WASM 下载失败后可直接重试恢复", async ({ page, context }, testInfo) => {
  testInfo.annotations.push({
    type: "expected-network-failure",
    description: "Intentional WASM fetch abort",
  });
  await context.route("**/wasm/**/qimen_wasm_bg.wasm", (route) =>
    route.abort("failed"),
  );
  await page.goto(sharedPath(REFERENCE_REQUEST));
  await expect(
    page.getByRole("complementary", { name: "起局参数" }).getByRole("alert"),
  ).toContainText("计算引擎加载失败");
  await expect(page.locator(".palace-grid button")).toHaveCount(0);
  await context.unroute("**/wasm/**/qimen_wasm_bg.wasm");
  await page.getByRole("button", { name: "开始排盘", exact: true }).click();
  await ready(page);
  expect((await exportedChart(page)).ju).toBe(9);
  await expect(
    page.getByRole("complementary", { name: "起局参数" }).getByRole("alert"),
  ).toHaveCount(0);
});

test("罗盘短暂旋转，减弱动效模式无需等待动画计时器", async ({ page }) => {
  await openChart(page);
  await page.clock.install();
  await page.clock.pauseAt(new Date());
  await page.getByLabel("当地时间", { exact: true }).fill("18:15:00");
  await page.getByRole("button", { name: "开始排盘", exact: true }).click();
  await expect(page.locator(".chart-stage")).toHaveAttribute(
    "aria-busy",
    "true",
  );
  await expect(page.getByRole("region", { name: "八字与局式" })).toContainText(
    "18:00",
  );
  await expect(
    page.getByRole("button", { name: "导出 JSON", exact: true }),
  ).toBeDisabled();
  await page.clock.fastForward(800);
  await expect(page.locator(".chart-stage")).toHaveAttribute(
    "aria-busy",
    "true",
  );
  await page.clock.fastForward(150);
  await ready(page);
  await expect(page.getByRole("region", { name: "八字与局式" })).toContainText(
    "18:15",
  );

  await page.getByLabel("当地时间", { exact: true }).fill("18:30:00");
  await page.getByRole("button", { name: "开始排盘", exact: true }).click();
  await expect(page.locator(".chart-stage")).toHaveAttribute(
    "aria-busy",
    "true",
  );
  await page.emulateMedia({ reducedMotion: "reduce" });
  await expect(page.locator(".qimen-app")).toHaveClass(/quiet-mode/);
  // The browser clock remains paused: a hidden 900 ms delay would never complete.
  await ready(page);
  await expect(page.getByRole("region", { name: "八字与局式" })).toContainText(
    "18:30",
  );
  await page.getByLabel("当地时间", { exact: true }).fill("18:45:00");
  await page.getByRole("button", { name: "开始排盘", exact: true }).click();
  await ready(page);
  await expect(page.getByRole("region", { name: "八字与局式" })).toContainText(
    "18:45",
  );
});

test("手机与桌面完整排盘无横向溢出，九宫支持键盘选择", async ({ page }) => {
  await openChart(page);
  await page.setViewportSize({ width: 390, height: 1000 });
  await page.getByLabel("当地时间", { exact: true }).fill("18:15:00");
  await page.getByRole("button", { name: "开始排盘", exact: true }).click();
  await expect(page.locator(".casting-indicator")).toBeInViewport();
  await expect(page.locator(".chart-stage")).toHaveAttribute(
    "aria-busy",
    "true",
  );
  await ready(page);
  for (const width of [320, 390, 1440]) {
    await page.setViewportSize({ width, height: 1000 });
    await expectNoOverflow(page);
    await expect(page.locator(".palace-grid button")).toHaveCount(9);
    await page.locator("#palace-4").focus();
    await page.keyboard.press("ArrowRight");
    await expect(page.locator("#palace-9")).toBeFocused();
    await page.keyboard.press("End");
    await expect(page.locator("#palace-6")).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(page.locator("#palace-6")).toHaveAttribute(
      "aria-pressed",
      "true",
    );
    await expect(
      page
        .getByRole("complementary", { name: "宫位详情" })
        .getByRole("heading", { name: "乾六宫", exact: true }),
    ).toBeVisible();
    await page
      .getByRole("button", { name: "关闭宫位详情", exact: true })
      .click();
    await expect(page.locator("#palace-6")).toBeFocused();
    await expectNoOverflow(page);
  }
});
