"use client";

export default function ErrorPage({
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <main id="main-content" className="recovery-page">
      <p className="eyebrow">QIMEN-RS</p>
      <h1>页面暂时无法呈现</h1>
      <p>请重新尝试。排盘时间不会上传到服务器。</p>
      <button className="primary-button" onClick={reset}>
        重新加载
      </button>
    </main>
  );
}
