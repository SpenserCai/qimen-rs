import Link from "next/link";

export default function NotFound() {
  return (
    <main id="main-content" className="recovery-page">
      <p className="eyebrow">QIMEN-RS · 404</p>
      <h1>此处尚无星图</h1>
      <p>这个页面不存在，回到排盘继续探索。</p>
      <Link className="primary-button" href="/">
        返回排盘
      </Link>
    </main>
  );
}
