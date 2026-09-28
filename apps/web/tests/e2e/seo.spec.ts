import { test, expect } from "@playwright/test";

const origin = "https://qimen-rs.vercel.app";
const routes = [
  ["/", "奇门遁甲在线排盘"],
  ["/guide", "使用指南"],
  ["/guide/conventions", "时间与排盘约定"],
  ["/guide/extensions", "扩展注记与参数"],
  ["/developers", "开发者接入"],
  ["/about", "关于 qimen-rs"],
];

test("SEO 页面无需 JavaScript 即可读取正文与独立规范链接", async ({
  browser,
  baseURL,
}) => {
  const context = await browser.newContext({
    javaScriptEnabled: false,
    viewport: { width: 390, height: 844 },
  });
  const page = await context.newPage();
  try {
    for (const [path, title] of routes) {
      const response = await page.goto(new URL(path, baseURL).href);
      expect(response?.status()).toBe(200);
      await expect(page).toHaveTitle(`${title} · qimen-rs`);
      await expect(page.locator('link[rel="canonical"]')).toHaveAttribute(
        "href",
        `${origin}${path}`,
      );
      await expect(page.locator('meta[property="og:url"]')).toHaveAttribute(
        "content",
        `${origin}${path}`,
      );
      await expect(page.locator('meta[property="og:title"]')).toHaveAttribute(
        "content",
        `${title} · qimen-rs`,
      );
      await expect(page.locator('meta[name="twitter:card"]')).toHaveAttribute(
        "content",
        "summary_large_image",
      );
      await expect(page.locator('meta[name="description"]')).toHaveAttribute(
        "content",
        /奇门|公历|排盘|暗干/,
      );
      await expect(page.locator("h1")).toHaveCount(1);
      if (path !== "/") {
        await expect(page.locator("main")).toContainText(/排盘|计算/);
        expect(
          await page.evaluate(
            () => document.documentElement.scrollWidth <= innerWidth,
          ),
        ).toBe(true);
      }
    }
    await page.goto(new URL("/?utm_source=regression", baseURL).href);
    await expect(page.locator('link[rel="canonical"]')).toHaveAttribute(
      "href",
      `${origin}/`,
    );
    const graph = JSON.parse(
      await page.locator('script[type="application/ld+json"]').innerText(),
    )["@graph"];
    expect(graph.map((item: { "@type": string }) => item["@type"])).toEqual([
      "WebSite",
      "WebApplication",
    ]);
    expect(graph[1].isAccessibleForFree).toBe(true);
    expect(graph[1].aggregateRating).toBeUndefined();
  } finally {
    await context.close();
  }
});

test("SEO sitemap、robots、分享卡片与 404 正确返回", async ({ request }) => {
  const sitemap = await request.get("/sitemap.xml");
  expect(sitemap.status()).toBe(200);
  const urls = [...(await sitemap.text()).matchAll(/<loc>(.*?)<\/loc>/g)].map(
    (match) => match[1],
  );
  expect(urls.sort()).toEqual(
    routes.map(([path]) => `${origin}${path}`).sort(),
  );
  const robots = await request.get("/robots.txt");
  expect(robots.status()).toBe(200);
  expect(await robots.text()).toContain("Allow: /");
  expect(await robots.text()).toContain(`Sitemap: ${origin}/sitemap.xml`);
  const card = await request.get("/opengraph-image");
  expect(card.status()).toBe(200);
  expect(card.headers()["content-type"]).toContain("image/png");
  const image = await card.body();
  expect(image.subarray(1, 4).toString()).toBe("PNG");
  expect(image.readUInt32BE(16)).toBe(1200);
  expect(image.readUInt32BE(20)).toBe(630);
  const missing = await request.get("/page-that-does-not-exist");
  expect(missing.status()).toBe(404);
});
