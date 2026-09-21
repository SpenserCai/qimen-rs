import type { Chart } from "@spensercai/qimen-wasm";
import {
  BRANCH_LABELS,
  DOOR_LABELS,
  STAR_LABELS,
  YUAN_LABELS,
  cycleLabel,
} from "@/lib/qimen/labels";
import { civilLabel, NUMBER_LABELS, offsetLabel } from "./chart-format";

export function ChartSummary({ chart }: { chart: Chart }) {
  return (
    <section className="chart-summary" aria-label="八字与局式">
      <div className="chart-date">
        <time>{civilLabel(chart.input, true)}</time>
        <span>{chart.calendar.lunar_date.name}</span>
      </div>
      <div className="pillars panel">
        {(["year", "month", "day", "hour"] as const).map((key, index) => (
          <div className="pillar" key={key}>
            <span>{["年柱", "月柱", "日柱", "时柱"][index]}</span>
            <strong className={key === "day" ? "day-pillar" : ""}>
              {cycleLabel(chart.calendar.four_pillars[key])}
            </strong>
          </div>
        ))}
      </div>
      <div className="chart-facts">
        <strong>
          {chart.dun === "yang" ? "阳遁" : "阴遁"}
          {NUMBER_LABELS[chart.ju]}局
        </strong>
        <span>{YUAN_LABELS[chart.yuan]}</span>
        <span>{cycleLabel(chart.xun.head)}旬</span>
        <span>
          值符 <b>{STAR_LABELS[chart.leaders.star]}</b>
        </span>
        <span>
          值使 <b>{DOOR_LABELS[chart.leaders.door]}</b>
        </span>
      </div>
      <div className="chart-context">
        <span>
          {chart.calendar.solar_term.name} <span aria-hidden="true">·</span>{" "}
          {offsetLabel(chart.input.utc_offset_minutes)}
        </span>
        <span>
          {chart.input.day_boundary === "zi_start" ? "子初换日" : "午夜换日"}
          <span className="context-divider">/</span>
          {BRANCH_LABELS[chart.calendar.four_pillars.hour.branch]}时
        </span>
      </div>
    </section>
  );
}
