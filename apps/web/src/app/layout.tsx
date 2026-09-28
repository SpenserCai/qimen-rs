import type { Metadata, Viewport } from "next";
import { SITE_URL, SITE_DESCRIPTION } from "@/lib/site/metadata";
import { CelestialBackdrop } from "@/components/celestial-backdrop";
import "@fontsource/ma-shan-zheng/400.css";
import "lxgw-wenkai-webfont/lxgwwenkai-regular.css";
import "@fontsource-variable/noto-serif-sc";
import "./globals.css";

export const metadata: Metadata = {
  metadataBase: new URL(SITE_URL),
  title: { default: "奇门遁甲在线排盘 · qimen-rs", template: "%s · qimen-rs" },
  description: SITE_DESCRIPTION,
  applicationName: "qimen-rs",
  icons: { icon: "/icon.svg" },
  robots: { index: process.env.VERCEL_ENV !== "preview", follow: true },
};

export const viewport: Viewport = {
  themeColor: "#050d19",
  colorScheme: "dark",
};

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="zh-CN">
      <body>
        <CelestialBackdrop />
        <a className="skip-link" href="#main-content">
          跳转到主要内容
        </a>
        {children}
      </body>
    </html>
  );
}
