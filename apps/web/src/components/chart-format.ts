import type { CivilDateTime, StemPlacement } from "@spensercai/qimen-wasm";
import { STEM_LABELS } from "@/lib/qimen/labels";

export const NUMBER_LABELS = [
  "",
  "一",
  "二",
  "三",
  "四",
  "五",
  "六",
  "七",
  "八",
  "九",
];
import { PLATE_LABELS } from "@/lib/qimen/labels";

export function civilLabel(value: CivilDateTime, seconds = false): string {
  const pad = (number: number) => String(number).padStart(2, "0");
  const date = `${String(value.year).padStart(4, "0")}.${pad(value.month)}.${pad(value.day)}`;
  return `${date} ${pad(value.hour)}:${pad(value.minute)}${seconds ? `:${pad(value.second)}` : ""}`;
}

export function offsetLabel(minutes: number): string {
  const absolute = Math.abs(minutes);
  return `UTC${minutes < 0 ? "−" : "+"}${String(Math.floor(absolute / 60)).padStart(2, "0")}:${String(absolute % 60).padStart(2, "0")}`;
}

export function placementLabel(placement: StemPlacement): string {
  return `${PLATE_LABELS[placement.plate]} ${STEM_LABELS[placement.stem]}${placement.is_center_hosted ? " · 寄干" : ""}`;
}

export function placementKey(placement: StemPlacement): string {
  return `${placement.plate}-${placement.stem}-${placement.source_palace ?? "none"}-${placement.is_center_hosted}`;
}
