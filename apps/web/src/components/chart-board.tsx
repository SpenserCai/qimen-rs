"use client";

import { m } from "motion/react";
import type { Chart, Palace, PalaceNumber } from "@spensercai/qimen-wasm";
import { CelestialOrbits } from "./celestial-orbits";
import {
  BRANCH_LABELS,
  DEITY_LABELS,
  DOOR_LABELS,
  PALACE_LABELS,
  PALACE_GRID_ORDER,
  STAR_LABELS,
  STEM_LABELS,
} from "@/lib/qimen/labels";

const ORDER: readonly PalaceNumber[] = PALACE_GRID_ORDER;
const RING_SYMBOLS = [
  "午",
  "未",
  "坤",
  "申",
  "酉",
  "戌",
  "乾",
  "亥",
  "子",
  "丑",
  "艮",
  "寅",
  "卯",
  "辰",
  "巽",
  "巳",
];

function CompassRings({
  casting,
  rotation,
  reducedMotion,
}: {
  casting: boolean;
  rotation: number;
  reducedMotion: boolean;
}) {
  return (
    <m.div
      className={`compass-rings ${casting ? "is-casting" : ""}`}
      aria-hidden="true"
      animate={{ rotate: reducedMotion ? 0 : rotation * 360 }}
      transition={{
        duration: reducedMotion ? 0 : 0.9,
        ease: [0.25, 0.05, 0.2, 1],
      }}
    >
      <svg viewBox="0 0 640 640" fill="none">
        <circle cx="320" cy="320" r="306" className="ring-primary" />
        <circle cx="320" cy="320" r="300" />
        <circle cx="320" cy="320" r="275" />
        <circle cx="320" cy="320" r="267" className="ring-primary" />
        <circle cx="320" cy="320" r="244" />
        <circle cx="320" cy="320" r="220" strokeDasharray="2 7" />
        {Array.from({ length: 96 }, (_, i) => (
          <path
            key={i}
            d={`M320 20V${i % 4 === 0 ? 34 : 27}`}
            transform={`rotate(${i * 3.75} 320 320)`}
          />
        ))}
        {Array.from({ length: 16 }, (_, i) => (
          <path
            key={i}
            d="M320 53V74M320 83V100"
            transform={`rotate(${i * 22.5} 320 320)`}
          />
        ))}
        {RING_SYMBOLS.map((symbol, i) => (
          <text
            key={symbol}
            x="320"
            y="54"
            textAnchor="middle"
            transform={`rotate(${i * 22.5} 320 320)`}
          >
            {symbol}
          </text>
        ))}
        <path d="M320 76L564 320L320 564L76 320Z" opacity=".35" />
        <path d="M148 148H492V492H148Z" opacity=".3" />
        {[0, 90, 180, 270].map((angle) => (
          <g key={angle} transform={`rotate(${angle} 320 320)`}>
            <path d="M312 10L320 2L328 10L320 18Z" className="ring-jewel" />
            <circle cx="320" cy="75" r="3" className="ring-jewel" />
          </g>
        ))}
      </svg>
    </m.div>
  );
}

function PalaceCell({
  palace,
  chart,
  selected,
  onSelect,
}: {
  palace: Palace;
  chart: Chart;
  selected: boolean;
  onSelect: () => void;
}) {
  const hidden = chart.extensions?.hidden_stems?.palaces.find(
    (item) => item.palace === palace.number,
  );
  const dayHorse = chart.extensions?.day_horse?.horse.palace === palace.number;
  const isLeader = palace.number === chart.leaders.star_palace;
  return (
    <button
      type="button"
      id={`palace-${palace.number}`}
      className={`palace-cell ${selected ? "selected-palace" : ""} ${palace.number === 5 ? "center-palace" : ""}`}
      aria-pressed={selected}
      aria-label={`${PALACE_LABELS[palace.number]}，${palace.stars.map((star) => STAR_LABELS[star]).join("、") || "中宫"}，${palace.door ? DOOR_LABELS[palace.door] : "无门"}，查看详情`}
      onClick={onSelect}
      onKeyDown={(event) => {
        const index = ORDER.indexOf(palace.number);
        const next = {
          ArrowRight: index + 1,
          ArrowLeft: index - 1,
          ArrowDown: index + 3,
          ArrowUp: index - 3,
          Home: 0,
          End: 8,
        }[event.key];
        if (next !== undefined) {
          event.preventDefault();
          const target = ORDER[(next + 9) % 9];
          document.getElementById(`palace-${target}`)?.focus();
        }
      }}
    >
      <span className="palace-top">
        <span>{PALACE_LABELS[palace.number]}</span>
        <span className="palace-markers">
          {hidden ? (
            <span className="hidden-stem" title="暗干">
              {STEM_LABELS[hidden.stem]}
            </span>
          ) : null}
          {palace.void_branches.length > 0 ? (
            <span
              title={`时空：${palace.void_branches.map((branch) => BRANCH_LABELS[branch]).join("")}`}
            >
              ○
            </span>
          ) : null}
          {palace.is_horse ? (
            <span className="horse-marker" title="时马">
              马
            </span>
          ) : null}
          {dayHorse ? (
            <span className="day-horse-marker" title="日马">
              日马
            </span>
          ) : null}
        </span>
      </span>
      {palace.number === 5 ? (
        <span className="center-content">
          <span className="center-emblem" aria-hidden="true">
            五
          </span>
          <strong>{STEM_LABELS[palace.earth_stem]}</strong>
          <span>中五寄坤二</span>
        </span>
      ) : (
        <>
          <span
            className={`palace-deity ${palace.deity === "zhi_fu" ? "jade-text" : ""}`}
          >
            {palace.deity ? DEITY_LABELS[palace.deity] : "—"}
          </span>
          <span className="palace-main">
            <span className="palace-star-door">
              <strong className={`palace-stars ${isLeader ? "jade-text" : ""}`}>
                {palace.stars
                  .filter((star) => star !== "tian_qin")
                  .map((star) => STAR_LABELS[star])
                  .join("·")}
                {palace.stars.includes("tian_qin") ? (
                  <small className="hosted-star" title="天禽寄宫">
                    ·禽
                  </small>
                ) : null}
              </strong>
              <strong
                className={`palace-door ${palace.number === chart.leaders.door_palace ? "duty-door" : ""}`}
              >
                {palace.door ? DOOR_LABELS[palace.door] : "—"}
              </strong>
            </span>
            <span className="palace-stems">
              <span>
                <small>天</small>
                {palace.heaven_stems.map((item) => (
                  <span
                    key={`${item.stem}-${item.source_palace}`}
                    className={item.is_center_hosted ? "hosted-stem" : ""}
                    title={item.is_center_hosted ? "天盘寄干" : "天盘干"}
                  >
                    {STEM_LABELS[item.stem]}
                    {item.is_center_hosted ? <sup>寄</sup> : null}
                  </span>
                ))}
              </span>
              <span>
                <small>地</small>
                {STEM_LABELS[palace.earth_stem]}
                {palace.hosted_earth_stem ? (
                  <span className="hosted-stem">
                    {STEM_LABELS[palace.hosted_earth_stem]}
                    <sup>寄</sup>
                  </span>
                ) : null}
              </span>
            </span>
          </span>
        </>
      )}
      <span className="palace-corner" aria-hidden="true" />
    </button>
  );
}

export function ChartBoard({
  chart,
  selected,
  onSelect,
  casting,
  rotation,
  reducedMotion,
}: {
  chart: Chart | null;
  selected: PalaceNumber | null;
  onSelect: (palace: PalaceNumber) => void;
  casting: boolean;
  rotation: number;
  reducedMotion: boolean;
}) {
  return (
    <div
      className={`chart-stage ${casting ? "casting" : ""}`}
      aria-busy={casting}
    >
      <CelestialOrbits />
      <div className="chart-face">
        <CompassRings
          casting={casting}
          rotation={rotation}
          reducedMotion={reducedMotion}
        />
        <span className="chart-axis axis-south" aria-hidden="true">
          南
        </span>
        <span className="chart-axis axis-north" aria-hidden="true">
          北
        </span>
        <span className="chart-axis axis-east" aria-hidden="true">
          东
        </span>
        <span className="chart-axis axis-west" aria-hidden="true">
          西
        </span>
        <div
          className="palace-grid"
          role="group"
          aria-label="九宫排盘，南在上、东在左，可使用方向键移动焦点"
        >
          {chart
            ? ORDER.map((number) => (
                <PalaceCell
                  key={number}
                  palace={chart.palaces[number - 1]}
                  chart={chart}
                  selected={selected === number}
                  onSelect={() => onSelect(number)}
                />
              ))
            : ORDER.map((number) => (
                <div className="palace-cell skeleton-palace" key={number}>
                  <span>{PALACE_LABELS[number]}</span>
                  <span className="skeleton-line" />
                  <span className="skeleton-line short" />
                </div>
              ))}
        </div>
      </div>
      {casting ? (
        <div className="casting-indicator" role="status">
          <span className="casting-dot" />
          推演天地 · 正在起局
        </div>
      ) : null}
    </div>
  );
}
