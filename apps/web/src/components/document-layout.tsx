import Link from "next/link";
import type { ReactNode } from "react";
import { PUBLIC_PAGES } from "@/lib/site/metadata";

export function DocumentLayout({
  title,
  introduction,
  children,
}: {
  title: string;
  introduction: string;
  children: ReactNode;
}) {
  return (
    <div className="guide-page min-h-dvh px-5 py-8 text-foreground sm:px-8 sm:py-12">
      <div className="guide-surface mx-auto max-w-3xl px-6 py-5 sm:px-10 sm:py-8">
        <nav
          aria-label="文档导航"
          className="flex flex-wrap gap-x-5 gap-y-3 text-sm text-gold"
        >
          {PUBLIC_PAGES.map(({ path, title: label }) => (
            <Link
              key={path}
              href={path}
              className="inline-flex min-h-11 items-center underline underline-offset-4"
            >
              {path === "/" ? "返回排盘" : label}
            </Link>
          ))}
        </nav>
        <main id="main-content" className="pt-8 pb-12">
          <h1 className="font-serif text-3xl leading-tight text-gold-light sm:text-4xl">
            {title}
          </h1>
          <p className="mt-5 leading-8 text-muted">{introduction}</p>
          <div className="document-content mt-8 space-y-8 leading-8">
            {children}
          </div>
        </main>
        <footer className="border-t border-gold-line pt-6 text-xs text-muted">
          qimen-rs · 奇门遁甲
        </footer>
      </div>
    </div>
  );
}
