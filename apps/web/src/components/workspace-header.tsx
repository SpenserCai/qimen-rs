"use client";

import Link from "next/link";
import { BookOpen, Code2, Sparkles, VolumeX } from "lucide-react";

export function WorkspaceHeader({
  quiet,
  onQuietChange,
}: {
  quiet: boolean;
  onQuietChange: () => void;
}) {
  return (
    <header className="workspace-header">
      <Link href="/" className="brand" aria-label="qimen-rs 奇门遁甲首页">
        <span className="brand-seal" aria-hidden="true">
          <span>奇</span>
        </span>
        <span className="brand-wordmark">
          qimen<span>·</span>rs<small>奇门遁甲</small>
        </span>
      </Link>
      <nav className="header-nav" aria-label="主导航">
        <span className="nav-current" aria-current="page">
          奇门排盘
        </span>
        <Link href="/guide">
          <BookOpen size={15} aria-hidden="true" />
          <span>使用指南</span>
        </Link>
      </nav>
      <div className="header-actions">
        <button
          type="button"
          className={`icon-button quiet-button ${quiet ? "is-active" : ""}`}
          onClick={onQuietChange}
          aria-pressed={quiet}
          aria-label={
            quiet ? "关闭静心模式，恢复动效" : "开启静心模式，减弱动效"
          }
          title="静心模式"
        >
          {quiet ? (
            <VolumeX size={18} aria-hidden="true" />
          ) : (
            <Sparkles size={18} aria-hidden="true" />
          )}
          <span>静心</span>
        </button>
        <a
          className="icon-button"
          href="https://github.com/SpenserCai/qimen-rs"
          target="_blank"
          rel="noopener noreferrer"
          aria-label="在新窗口打开 GitHub 项目"
        >
          <Code2 size={20} aria-hidden="true" />
        </a>
      </div>
    </header>
  );
}
