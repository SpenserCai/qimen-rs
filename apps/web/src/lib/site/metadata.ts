import type { Metadata } from "next";

/** One canonical origin for public pages, social cards and the sitemap. */
export const SITE_URL = "https://qimen-rs.vercel.app";
export const REPOSITORY_URL = "https://github.com/SpenserCai/qimen-rs";
export const SITE_DESCRIPTION =
  "开源奇门遁甲在线排盘，支持八字与时家拆补转盘、九宫详情和可选注记。无需登录，计算在浏览器内完成；提供 Rust、Python、Node.js、WebAssembly 与 MCP 接口。";
export const PUBLIC_PAGES = [
  { path: "/", title: "奇门遁甲在线排盘" },
  { path: "/guide", title: "使用指南" },
  { path: "/guide/conventions", title: "时间与排盘约定" },
  { path: "/guide/extensions", title: "扩展注记与参数" },
  { path: "/developers", title: "开发者接入" },
  { path: "/about", title: "关于 qimen-rs" },
] as const;

export function pageMetadata(
  path: string,
  title: string,
  description: string,
): Metadata {
  const url = new URL(path, SITE_URL).href;
  const fullTitle = `${title} · qimen-rs`;
  return {
    title: { absolute: fullTitle },
    description,
    alternates: { canonical: url },
    openGraph: {
      type: "website",
      locale: "zh_CN",
      siteName: "qimen-rs",
      title: fullTitle,
      description,
      url,
      images: [
        {
          url: `${SITE_URL}/opengraph-image`,
          width: 1200,
          height: 630,
          alt: "qimen-rs · 开源奇门遁甲排盘",
        },
      ],
    },
    twitter: {
      card: "summary_large_image",
      title: fullTitle,
      description,
      images: [`${SITE_URL}/opengraph-image`],
    },
  };
}
