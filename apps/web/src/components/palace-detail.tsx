"use client";

import { useState, type ReactNode } from "react";
import { ChevronDown, Compass, X } from "lucide-react";
import type { Chart, PalaceNumber } from "@spensercai/qimen-wasm";
import {
  BRANCH_LABELS,
  DEITY_LABELS,
  DIRECTION_LABELS,
  DOOR_LABELS,
  ELEMENT_LABELS,
  GROWTH_LABELS,
  PALACE_LABELS,
  STAR_LABELS,
  STEM_LABELS,
  STRENGTH_LABELS,
  cycleLabel,
} from "@/lib/qimen/labels";
import { civilLabel, placementKey, placementLabel } from "./chart-format";

function DetailRow({
  label,
  children,
}: {
  label: string;
  children: ReactNode;
}) {
  return (
    <div className="detail-row">
      <dt>{label}</dt>
      <dd>{children}</dd>
    </div>
  );
}

function AnnotationSection({
  title,
  rule,
  children,
  open = false,
}: {
  title: string;
  rule: string;
  children: ReactNode;
  open?: boolean;
}) {
  return (
    <details className="annotation-section" open={open}>
      <summary>
        {title}
        <ChevronDown size={14} aria-hidden="true" />
      </summary>
      <div className="annotation-body">
        <p className="annotation-rule">{rule}</p>
        {children}
      </div>
    </details>
  );
}

function PalaceAnnotations({
  chart,
  selected,
}: {
  chart: Chart;
  selected: PalaceNumber;
}) {
  const extensions = chart.extensions;
  if (!extensions || Object.keys(extensions).length === 0)
    return (
      <div className="empty-annotations">
        <Compass size={26} aria-hidden="true" />
        <p>此局未开启扩展注记</p>
        <span>在起局设置中启用后，重新排盘即可查看。</span>
      </div>
    );
  const hidden = extensions.hidden_stems?.palaces.find(
    (item) => item.palace === selected,
  );
  const strength = extensions.strength?.palaces.find(
    (item) => item.palace === selected,
  );
  const growth = extensions.growth_stages?.stems.filter(
    (item) => item.placement.palace === selected,
  );
  const punishments = extensions.punishments?.stems.filter(
    (item) => item.placement.palace === selected,
  );
  const tombs = extensions.tombs?.stems.filter(
    (item) => item.placement.palace === selected,
  );
  const pressure = extensions.door_pressure?.doors.find(
    (item) => item.palace === selected,
  );
  return (
    <div className="annotations">
      {hidden && extensions.hidden_stems ? (
        <AnnotationSection
          title={`暗干 · ${STEM_LABELS[hidden.stem]}`}
          rule="值使门起时干；遇重干从中宫起排。"
          open
        >
          <dl>
            <DetailRow label="起排宫">
              {PALACE_LABELS[extensions.hidden_stems.start_palace]}
            </DetailRow>
            <DetailRow label="有效时干">
              {STEM_LABELS[extensions.hidden_stems.effective_hour_stem]}
            </DetailRow>
            <DetailRow label="中宫起排">
              {extensions.hidden_stems.used_center_fallback ? "是" : "否"}
            </DetailRow>
          </dl>
        </AnnotationSection>
      ) : null}
      {strength && extensions.strength ? (
        <AnnotationSection
          title="旺衰"
          rule={`九星用古典旺衰，门干用五行生克；月令为${BRANCH_LABELS[extensions.strength.month_branch]}月（${ELEMENT_LABELS[extensions.strength.month_element]}）。`}
          open
        >
          <div className="strength-table">
            <div className="strength-table-header">
              <span>对象</span>
              <span>宫内</span>
              <span>月令</span>
            </div>
            {strength.stars.map((item) => (
              <div key={item.star}>
                <span>
                  {STAR_LABELS[item.star]} · {ELEMENT_LABELS[item.element]}
                </span>
                <b>{STRENGTH_LABELS[item.at_palace]}</b>
                <b>{STRENGTH_LABELS[item.at_month]}</b>
              </div>
            ))}
            {strength.door ? (
              <div>
                <span>
                  {DOOR_LABELS[strength.door.door]} ·{" "}
                  {ELEMENT_LABELS[strength.door.element]}
                </span>
                <b>{STRENGTH_LABELS[strength.door.at_palace]}</b>
                <b>{STRENGTH_LABELS[strength.door.at_month]}</b>
              </div>
            ) : null}
            {strength.stems.map((item) => (
              <div key={placementKey(item.placement)}>
                <span>{placementLabel(item.placement)}</span>
                <b>{STRENGTH_LABELS[item.at_palace]}</b>
                <b>{STRENGTH_LABELS[item.at_month]}</b>
              </div>
            ))}
          </div>
        </AnnotationSection>
      ) : null}
      {growth ? (
        <AnnotationSection
          title="十二长生"
          rule="阳干顺行、阴干逆行；火土同宫，角宫双支分别计算。"
        >
          <dl>
            {growth.map((item) => (
              <DetailRow
                key={placementKey(item.placement)}
                label={placementLabel(item.placement)}
              >
                {item.branches.length
                  ? item.branches
                      .map(
                        (branch) =>
                          `${BRANCH_LABELS[branch.branch]}·${GROWTH_LABELS[branch.stage]}`,
                      )
                      .join(" / ")
                  : "中宫无地支"}
              </DetailRow>
            ))}
          </dl>
        </AnnotationSection>
      ) : null}
      {punishments ? (
        <AnnotationSection
          title="六仪击刑"
          rule="按六仪所遁甲支与落宫地支判断。"
        >
          <dl>
            {punishments.map((item) => (
              <DetailRow
                key={placementKey(item.placement)}
                label={placementLabel(item.placement)}
              >
                <span className={item.is_punished ? "cinnabar-text" : ""}>
                  {item.is_punished
                    ? "击刑"
                    : item.hidden_jia
                      ? "未击刑"
                      : "不适用"}
                </span>
                {item.hidden_jia && item.punished_branch ? (
                  <small>
                    {cycleLabel(item.hidden_jia)} · 刑
                    {BRANCH_LABELS[item.punished_branch]}
                  </small>
                ) : null}
              </DetailRow>
            ))}
          </dl>
        </AnnotationSection>
      ) : null}
      {tombs && extensions.tombs ? (
        <AnnotationSection
          title="入墓"
          rule={
            extensions.tombs.rule === "traditional_three_wonders"
              ? "古典三奇入墓，仅对乙、丙、丁判定；其他干不适用。"
              : "按阴阳顺逆十二长生之墓判定，火土同宫。"
          }
        >
          <dl>
            {tombs.map((item) => (
              <DetailRow
                key={placementKey(item.placement)}
                label={placementLabel(item.placement)}
              >
                <span className={item.is_in_tomb ? "cinnabar-text" : ""}>
                  {item.is_in_tomb === null
                    ? "不适用"
                    : item.is_in_tomb
                      ? "入墓"
                      : "未入墓"}
                </span>
                {item.tomb_branch ? (
                  <small>墓支 · {BRANCH_LABELS[item.tomb_branch]}</small>
                ) : null}
              </DetailRow>
            ))}
          </dl>
        </AnnotationSection>
      ) : null}
      {extensions.day_horse ? (
        <AnnotationSection title="日马" rule="按日支三合局取驿马。">
          <dl>
            <DetailRow label="日柱">
              {cycleLabel(extensions.day_horse.pillar)}
            </DetailRow>
            <DetailRow label="日马">
              {BRANCH_LABELS[extensions.day_horse.horse.branch]} ·{" "}
              {PALACE_LABELS[extensions.day_horse.horse.palace]}
            </DetailRow>
            <DetailRow label="本宫">
              {extensions.day_horse.horse.palace === selected
                ? "日马临宫"
                : "日马未临"}
            </DetailRow>
          </dl>
        </AnnotationSection>
      ) : null}
      {extensions.door_pressure ? (
        <AnnotationSection title="门迫" rule="八门五行克落宫五行时为门迫。">
          <dl>
            <DetailRow label="本宫">
              {pressure
                ? pressure.is_pressed
                  ? "门迫"
                  : "无门迫"
                : "中宫无门，不适用"}
            </DetailRow>
            {pressure ? (
              <DetailRow label="五行">
                {DOOR_LABELS[pressure.door]} ·{" "}
                {ELEMENT_LABELS[pressure.door_element]} → 宫 ·{" "}
                {ELEMENT_LABELS[pressure.palace_element]}
              </DetailRow>
            ) : null}
          </dl>
        </AnnotationSection>
      ) : null}
    </div>
  );
}

export function PalaceDetail({
  chart,
  selected,
  onClose,
}: {
  chart: Chart | null;
  selected: PalaceNumber | null;
  onClose: () => void;
}) {
  const [tab, setTab] = useState<"base" | "annotations">("base");
  const palace = chart && selected ? chart.palaces[selected - 1] : null;
  if (!chart || !palace || !selected)
    return (
      <aside className="detail-panel panel detail-empty" aria-label="宫位详情">
        <Compass size={40} aria-hidden="true" />
        <h2>观宫察象</h2>
        <p>
          选择盘中任意一宫
          <br />
          查看星、门、神与干支注记
        </p>
        <span>天 地 人 神 · 各 归 其 位</span>
      </aside>
    );
  const hidden = chart.extensions?.hidden_stems?.palaces.find(
    (item) => item.palace === selected,
  );
  return (
    <aside className="detail-panel panel" aria-label="宫位详情">
      <div className="detail-header">
        <div>
          <span className="section-eyebrow">PALACE INSIGHT</span>
          <h2>{PALACE_LABELS[selected]}</h2>
        </div>
        <button
          type="button"
          className="icon-button close-detail"
          aria-label="关闭宫位详情"
          onClick={onClose}
        >
          <X size={17} aria-hidden="true" />
        </button>
      </div>
      <div className="palace-coordinate">
        <span>
          {DIRECTION_LABELS[palace.direction]}
          <span aria-hidden="true"> · </span>
          {ELEMENT_LABELS[palace.element]}
        </span>
        <span>{selected === 5 ? "中宫" : `第${selected}宫`}</span>
      </div>
      <div className="detail-door">
        <strong>{palace.door ? DOOR_LABELS[palace.door] : "中宫"}</strong>
        <div>
          {selected === chart.leaders.door_palace ? (
            <span className="seal-badge">值使</span>
          ) : null}
          {selected === chart.leaders.star_palace ? (
            <span className="seal-badge jade-seal">值符</span>
          ) : null}
        </div>
      </div>
      <div className="detail-tabs" aria-label="详情类别">
        <button
          type="button"
          aria-pressed={tab === "base"}
          className={tab === "base" ? "active" : ""}
          onClick={() => setTab("base")}
        >
          宫位信息
        </button>
        <button
          type="button"
          aria-pressed={tab === "annotations"}
          className={tab === "annotations" ? "active" : ""}
          onClick={() => setTab("annotations")}
        >
          扩展注记<span>{Object.keys(chart.extensions ?? {}).length}</span>
        </button>
      </div>
      <div className="detail-scroll">
        {tab === "base" ? (
          <>
            <dl className="base-detail">
              <DetailRow label="九星">
                {palace.stars.map((star) => STAR_LABELS[star]).join(" · ") ||
                  "天禽本位"}
              </DetailRow>
              <DetailRow label="八神">
                {palace.deity ? DEITY_LABELS[palace.deity] : "—"}
              </DetailRow>
              <DetailRow label="天盘">
                {palace.heaven_stems.length
                  ? palace.heaven_stems.map((item) => (
                      <span
                        key={`${item.stem}-${item.source_palace}`}
                        className="stem-detail-item"
                      >
                        {STEM_LABELS[item.stem]}
                        {item.is_center_hosted ? <small>寄干</small> : null}
                        <small>源自{PALACE_LABELS[item.source_palace]}</small>
                      </span>
                    ))
                  : "—"}
              </DetailRow>
              <DetailRow label="地盘">
                {STEM_LABELS[palace.earth_stem]}
                {palace.hosted_earth_stem ? (
                  <span> · 寄{STEM_LABELS[palace.hosted_earth_stem]}</span>
                ) : null}
              </DetailRow>
              {hidden ? (
                <DetailRow label="暗干">{STEM_LABELS[hidden.stem]}</DetailRow>
              ) : null}
              <DetailRow label="时空">
                {palace.void_branches.length
                  ? palace.void_branches
                      .map((branch) => BRANCH_LABELS[branch])
                      .join("、")
                  : "无"}
              </DetailRow>
              <DetailRow label="时马">
                {palace.is_horse
                  ? `${BRANCH_LABELS[chart.horse.branch]} · 临宫`
                  : "未临宫"}
              </DetailRow>
              {chart.extensions?.day_horse ? (
                <DetailRow label="日马">
                  {chart.extensions.day_horse.horse.palace === selected
                    ? `${BRANCH_LABELS[chart.extensions.day_horse.horse.branch]} · 临宫`
                    : "未临宫"}
                </DetailRow>
              ) : null}
            </dl>
            <div className="conventions-block">
              <h3>流派约定</h3>
              <p>
                中五寄坤二<span>天禽随天芮</span>
              </p>
              <p>
                空亡取时旬<span>驿马取时支</span>
              </p>
            </div>
            <details className="term-detail">
              <summary>
                节气与旬首
                <ChevronDown size={14} aria-hidden="true" />
              </summary>
              <dl>
                <DetailRow label="当前节气">
                  {chart.calendar.solar_term.name}
                  <small>
                    {civilLabel(chart.calendar.solar_term.start, true)}
                  </small>
                </DetailRow>
                <DetailRow label="下个节气">
                  {chart.calendar.next_solar_term.name}
                  <small>
                    {civilLabel(chart.calendar.next_solar_term.start, true)}
                  </small>
                </DetailRow>
                <DetailRow label="旬首">
                  {cycleLabel(chart.xun.head)} · 遁
                  {STEM_LABELS[chart.xun.hidden_stem]}
                </DetailRow>
                <DetailRow label="符头">
                  {cycleLabel(chart.yuan_head)}
                </DetailRow>
                <DetailRow label="值符本宫">
                  {PALACE_LABELS[chart.leaders.original_palace]}
                </DetailRow>
                <DetailRow label="值使原落宫">
                  {PALACE_LABELS[chart.leaders.door_raw_palace]}
                </DetailRow>
              </dl>
            </details>
          </>
        ) : (
          <PalaceAnnotations chart={chart} selected={selected} />
        )}
      </div>
      <div className="detail-footer">
        <span aria-hidden="true">✦</span> 星 门 神 干 · 合 参 九 宫
      </div>
    </aside>
  );
}
