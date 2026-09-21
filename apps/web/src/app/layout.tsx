import type { Metadata, Viewport } from "next";
import "@fontsource-variable/noto-serif-sc";
import "./globals.css";

export const metadata: Metadata = {
  title: { default: "qimen-rs · 奇门遁甲", template: "%s · qimen-rs" },
  description:
    "八字与时家拆补转盘。输入公历时间，即刻呈现九宫、星门神与可选注记。计算在浏览器内完成。",
  applicationName: "qimen-rs",
  icons: { icon: "/icon.svg" },
};

export const viewport: Viewport = {
  themeColor: "#071518",
  colorScheme: "dark",
};

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="zh-CN">
      <body>
        <a className="skip-link" href="#main-content">
          跳转到主要内容
        </a>
        {children}
      </body>
    </html>
  );
}
