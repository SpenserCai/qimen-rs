"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { LazyMotion, domAnimation } from "motion/react";
import {
  ArrowLeft,
  ArrowRight,
  Check,
  Copy,
  Download,
  Link2,
  ShieldCheck,
  X,
} from "lucide-react";
import type { Chart, ChartRequest, PalaceNumber } from "@spensercai/qimen-wasm";
import { useQimen } from "@/hooks/use-qimen";
import { useReducedMotion } from "@/hooks/use-reduced-motion";
import { BRANCH_LABELS } from "@/lib/qimen/labels";
import { loadPreferences, savePreferences } from "@/lib/qimen/preferences";
import {
  nowRequest,
  parseDraft,
  requestToDraft,
  stepRequest,
  type ChartDraft,
} from "@/lib/qimen/request";
import { restoreRequest, serializeRequest } from "@/lib/qimen/sharing";
import { ChartBoard } from "./chart-board";
import { ChartSummary } from "./chart-summary";
import { InputPanel } from "./input-panel";
import { PalaceDetail } from "./palace-detail";
import { WorkspaceHeader } from "./workspace-header";

export function QimenWorkspace() {
  const { result, request, status, error, calculate, cancel } = useQimen();
  const systemReducedMotion = useReducedMotion();
  const [quiet, setQuiet] = useState(false);
  const [draft, setDraft] = useState<ChartDraft | null>(null);
  const [validationError, setValidationError] = useState<string | null>(null);
  const [selected, setSelected] = useState<PalaceNumber | null>(6);
  const [casting, setCasting] = useState(false);
  const [rotation, setRotation] = useState(0);
  const [previous, setPrevious] = useState<Chart | null>(null);
  const [notice, setNotice] = useState("");
  const [manualCopy, setManualCopy] = useState<string | null>(null);
  const animationTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const animationResolve = useRef<(() => void) | null>(null);
  const runSequence = useRef(0);
  const reducedMotion = quiet || systemReducedMotion;
  const busy = casting || status === "calculating";
  const chart = casting ? previous : result;

  useEffect(() => {
    let active = true;
    queueMicrotask(() => {
      if (!active) return;
      const preferences = loadPreferences();
      const base = nowRequest();
      const configuration = preferences
        ? parseDraft({ ...requestToDraft(base), ...preferences })
        : null;
      const current = nowRequest(
        new Date(),
        configuration?.ok ? configuration.request.utc_offset_minutes : 480,
      );
      const initial: ChartRequest = {
        ...current,
        day_boundary: preferences?.dayBoundary ?? current.day_boundary,
        extensions: preferences?.extensions ?? current.extensions,
      };
      const shared = restoreRequest(window.location.hash);
      const input = shared?.ok ? shared.request : initial;
      setDraft(requestToDraft(input));
      setQuiet(preferences?.reduceMotion ?? false);
      if (shared && !shared.ok) {
        setValidationError(shared.error);
        return;
      }
      void calculate(input);
    });
    return () => {
      active = false;
    };
  }, [calculate]);

  useEffect(() => {
    function restoreSharedChart() {
      const shared = restoreRequest(window.location.hash);
      if (!shared) return;
      runSequence.current += 1;
      cancel();
      if (animationTimer.current) clearTimeout(animationTimer.current);
      animationTimer.current = null;
      animationResolve.current?.();
      animationResolve.current = null;
      setCasting(false);
      setNotice("");
      if (!shared.ok) {
        setValidationError(shared.error);
        return;
      }
      setValidationError(null);
      setDraft(requestToDraft(shared.request));
      void calculate(shared.request);
    }
    window.addEventListener("hashchange", restoreSharedChart);
    return () => window.removeEventListener("hashchange", restoreSharedChart);
  }, [calculate, cancel]);

  useEffect(
    () => () => {
      runSequence.current += 1;
      if (animationTimer.current) clearTimeout(animationTimer.current);
      animationResolve.current?.();
    },
    [],
  );

  useEffect(() => {
    if (!reducedMotion) return;
    if (animationTimer.current) clearTimeout(animationTimer.current);
    animationTimer.current = null;
    animationResolve.current?.();
    animationResolve.current = null;
  }, [reducedMotion]);

  const run = useCallback(
    async (input: ChartRequest) => {
      const sequence = ++runSequence.current;
      setValidationError(null);
      setNotice("");
      setPrevious(result);
      setCasting(!reducedMotion);
      setRotation((value) => value + 1);
      if (window.matchMedia("(max-width: 760px)").matches) {
        document.getElementById("chart-workspace")?.scrollIntoView({
          behavior: reducedMotion ? "instant" : "smooth",
          block: "start",
        });
      }
      const animation = reducedMotion
        ? Promise.resolve()
        : new Promise<void>((resolve) => {
            animationResolve.current = resolve;
            animationTimer.current = setTimeout(() => {
              animationTimer.current = null;
              animationResolve.current = null;
              resolve();
            }, 900);
          });
      await Promise.all([calculate(input), animation]);
      if (sequence === runSequence.current) setCasting(false);
    },
    [calculate, reducedMotion, result],
  );

  function submit() {
    if (!draft || busy) return;
    const parsed = parseDraft(draft);
    if (!parsed.ok) {
      setValidationError(parsed.error);
      return;
    }
    savePreferences({
      utcOffset: draft.utcOffset,
      dayBoundary: draft.dayBoundary,
      extensions: draft.extensions,
      reduceMotion: quiet,
    });
    void run(parsed.request);
  }

  function changeQuiet() {
    const next = !quiet;
    setQuiet(next);
    if (next) {
      if (animationTimer.current) clearTimeout(animationTimer.current);
      animationTimer.current = null;
      animationResolve.current?.();
      animationResolve.current = null;
    }
    if (draft && parseDraft(draft).ok)
      savePreferences({
        utcOffset: draft.utcOffset,
        dayBoundary: draft.dayBoundary,
        extensions: draft.extensions,
        reduceMotion: next,
      });
  }

  function useNow() {
    if (!draft || busy) return;
    const validated = parseDraft({
      ...requestToDraft(nowRequest()),
      utcOffset: draft.utcOffset,
    });
    const current = nowRequest(
      new Date(),
      validated.ok ? validated.request.utc_offset_minutes : 480,
    );
    setDraft({
      ...requestToDraft(current),
      dayBoundary: draft.dayBoundary,
      extensions: draft.extensions,
    });
    setValidationError(null);
  }

  function useExample() {
    if (!draft || busy) return;
    const example = {
      year: 2026,
      month: 9,
      day: 18,
      hour: 18,
      minute: 0,
      second: 0,
      utc_offset_minutes: 480,
      day_boundary: draft.dayBoundary,
      extensions: draft.extensions,
    };
    setDraft(requestToDraft(example));
    void run(example);
  }

  function step(direction: -1 | 1) {
    if (!request || busy) return;
    const next = stepRequest(request, direction);
    if (!next.ok) {
      setValidationError(next.error);
      setNotice(next.error);
      return;
    }
    setDraft(requestToDraft(next.request));
    void run(next.request);
  }

  async function copy(value: string, success: string) {
    try {
      await navigator.clipboard.writeText(value);
      setNotice(success);
      setManualCopy(null);
    } catch {
      setManualCopy(value);
      setNotice("浏览器未允许自动复制，请在下方手动复制。");
    }
  }

  function download() {
    if (!chart || busy) return;
    const url = URL.createObjectURL(
      new Blob([JSON.stringify(chart, null, 2)], {
        type: "application/json;charset=utf-8",
      }),
    );
    const anchor = document.createElement("a");
    anchor.href = url;
    const date = requestToDraft(chart.input).date;
    anchor.download = `qimen-${date}-${String(chart.input.hour).padStart(2, "0")}${String(chart.input.minute).padStart(2, "0")}.json`;
    anchor.click();
    setTimeout(() => URL.revokeObjectURL(url), 1000);
    setNotice("排盘 JSON 已导出。");
  }

  const parsedDraft = draft ? parseDraft(draft) : null;
  const draftChanged = Boolean(
    request &&
    draft &&
    (!parsedDraft?.ok ||
      serializeRequest(parsedDraft.request) !== serializeRequest(request)),
  );
  const currentError = validationError ?? error;

  return (
    <LazyMotion features={domAnimation} strict>
      <div className={`qimen-app ${reducedMotion ? "quiet-mode" : ""}`}>
        <div className="landscape-backdrop" aria-hidden="true" />
        <div className="landscape-vignette" aria-hidden="true" />
        <div className="ambient-mist mist-one" aria-hidden="true" />
        <div className="ambient-mist mist-two" aria-hidden="true" />
        <a href="#chart-workspace" className="skip-link">
          跳到排盘结果
        </a>
        <WorkspaceHeader quiet={quiet} onQuietChange={changeQuiet} />
        <main className="workspace-main" id="main-content">
          <h1 className="sr-only">奇门遁甲在线排盘</h1>
          <div className="workspace-intro">
            <span className="intro-line" />
            <p>
              观天之道<span>·</span>执天之行
            </p>
            <span className="intro-line" />
          </div>
          <div className="workspace-grid">
            <InputPanel
              draft={draft}
              busy={busy}
              error={currentError}
              onChange={(value) => {
                setDraft(value);
                setValidationError(null);
              }}
              onSubmit={submit}
              onNow={useNow}
              onExample={useExample}
            />
            <section
              className="chart-workspace"
              id="chart-workspace"
              tabIndex={-1}
              aria-label="排盘结果"
            >
              {chart ? (
                <ChartSummary chart={chart} />
              ) : (
                <div className="summary-placeholder">
                  <span className="section-eyebrow">天地定位 · 时序成局</span>
                  <h2>一时一局，观照天地</h2>
                  <p>
                    {currentError
                      ? "请调整起局参数后重新排盘"
                      : "正在准备此刻的奇门盘"}
                  </p>
                </div>
              )}
              <div
                className="result-state"
                aria-live="polite"
                aria-atomic="true"
              >
                {busy && chart
                  ? "正在起新局 · 当前显示上次排盘"
                  : currentError && chart
                    ? "未能起新局 · 当前显示上次排盘"
                    : draftChanged
                      ? "起局参数已修改 · 点击「开始排盘」更新结果"
                      : chart
                        ? "排盘完成 · 点选九宫查看详情"
                        : ""}
              </div>
              <ChartBoard
                chart={chart}
                selected={selected}
                onSelect={setSelected}
                casting={busy}
                rotation={rotation}
                reducedMotion={reducedMotion}
              />
              <div className="time-navigation">
                <button
                  type="button"
                  className="secondary-button"
                  onClick={() => step(-1)}
                  disabled={!request || busy}
                >
                  <ArrowLeft size={16} aria-hidden="true" />
                  <span>上一时辰</span>
                </button>
                <span>
                  {chart
                    ? `${BRANCH_LABELS[chart.calendar.four_pillars.hour.branch]}时`
                    : "—"}
                  <small>每次前后两小时</small>
                </span>
                <button
                  type="button"
                  className="secondary-button"
                  onClick={() => step(1)}
                  disabled={!request || busy}
                >
                  <span>下一时辰</span>
                  <ArrowRight size={16} aria-hidden="true" />
                </button>
              </div>
              {chart ? (
                <div className="void-horse-strip">
                  <span>
                    日空
                    <b>
                      {chart.pillar_voids.day
                        .map((branch) => BRANCH_LABELS[branch])
                        .join("")}
                    </b>
                  </span>
                  <span>
                    时空
                    <b>
                      {chart.pillar_voids.hour
                        .map((branch) => BRANCH_LABELS[branch])
                        .join("")}
                    </b>
                  </span>
                  {chart.extensions?.day_horse ? (
                    <span>
                      日马
                      <b>
                        {BRANCH_LABELS[chart.extensions.day_horse.horse.branch]}
                      </b>
                    </span>
                  ) : null}
                  <span>
                    时马<b>{BRANCH_LABELS[chart.horse.branch]}</b>
                  </span>
                </div>
              ) : null}
              <div className="chart-actions">
                <button
                  type="button"
                  className="secondary-button"
                  disabled={!chart || busy}
                  onClick={() => {
                    if (chart)
                      void copy(
                        JSON.stringify(chart, null, 2),
                        "排盘结果已复制。",
                      );
                  }}
                >
                  <Copy size={15} aria-hidden="true" />
                  <span>复制结果</span>
                </button>
                <button
                  type="button"
                  className="secondary-button"
                  disabled={!chart || busy}
                  onClick={download}
                >
                  <Download size={16} aria-hidden="true" />
                  <span>导出 JSON</span>
                </button>
                <button
                  type="button"
                  className="secondary-button"
                  disabled={!request || busy}
                  onClick={() => {
                    if (request)
                      void copy(
                        `${window.location.origin}${window.location.pathname}${serializeRequest(request)}`,
                        "分享链接已复制，链接包含此局时间和规则。",
                      );
                  }}
                >
                  <Link2 size={16} aria-hidden="true" />
                  <span>分享此局</span>
                </button>
              </div>
              <div className="action-notice" role="status" aria-live="polite">
                {notice ? (
                  <>
                    <Check size={14} aria-hidden="true" />
                    <span>{notice}</span>
                  </>
                ) : (
                  <>
                    <ShieldCheck size={14} aria-hidden="true" />
                    <span>排盘在你的设备上完成，时间资料不上传。</span>
                  </>
                )}
              </div>
              {manualCopy ? (
                <div className="manual-copy panel">
                  <label htmlFor="manual-copy-text">选中并复制以下内容</label>
                  <button
                    type="button"
                    className="icon-button"
                    onClick={() => setManualCopy(null)}
                    aria-label="关闭手动复制"
                  >
                    <X size={16} aria-hidden="true" />
                  </button>
                  <textarea
                    id="manual-copy-text"
                    readOnly
                    value={manualCopy}
                    onFocus={(event) => event.target.select()}
                    rows={4}
                  />
                </div>
              ) : null}
            </section>
            <PalaceDetail
              chart={chart}
              selected={selected}
              onClose={() => {
                const previousSelected = selected;
                setSelected(null);
                if (previousSelected)
                  document
                    .getElementById(`palace-${previousSelected}`)
                    ?.focus();
              }}
            />
          </div>
          <footer className="workspace-footer">
            <span>
              天地玄黄<span className="footer-dot">·</span>宇宙洪荒
            </span>
            <span>
              时家拆补转盘<span className="footer-dot">/</span>公元 1—9999 年
            </span>
          </footer>
        </main>
      </div>
    </LazyMotion>
  );
}
