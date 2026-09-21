import type {
  Branch,
  Cycle,
  Deity,
  Direction,
  Door,
  Element,
  ElementStrengthState,
  GrowthStage,
  PalaceNumber,
  Star,
  StarStrengthState,
  Stem,
  StemPlate,
  Trigram,
} from "./types";

export const STEM_LABELS = {
  jia: "甲",
  yi: "乙",
  bing: "丙",
  ding: "丁",
  wu: "戊",
  ji: "己",
  geng: "庚",
  xin: "辛",
  ren: "壬",
  gui: "癸",
} as const satisfies Record<Stem, string>;
export const BRANCH_LABELS = {
  zi: "子",
  chou: "丑",
  yin: "寅",
  mao: "卯",
  chen: "辰",
  si: "巳",
  wu: "午",
  wei: "未",
  shen: "申",
  you: "酉",
  xu: "戌",
  hai: "亥",
} as const satisfies Record<Branch, string>;
export const STAR_LABELS = {
  tian_peng: "天蓬",
  tian_rui: "天芮",
  tian_chong: "天冲",
  tian_fu: "天辅",
  tian_qin: "天禽",
  tian_xin: "天心",
  tian_zhu: "天柱",
  tian_ren: "天任",
  tian_ying: "天英",
} as const satisfies Record<Star, string>;
export const DOOR_LABELS = {
  xiu: "休门",
  si: "死门",
  shang: "伤门",
  du: "杜门",
  kai: "开门",
  jing: "惊门",
  sheng: "生门",
  scene: "景门",
} as const satisfies Record<Door, string>;
export const DEITY_LABELS = {
  zhi_fu: "值符",
  teng_she: "腾蛇",
  tai_yin: "太阴",
  liu_he: "六合",
  bai_hu: "白虎",
  xuan_wu: "玄武",
  jiu_di: "九地",
  jiu_tian: "九天",
} as const satisfies Record<Deity, string>;
export const PALACE_LABELS = {
  1: "坎一宫",
  2: "坤二宫",
  3: "震三宫",
  4: "巽四宫",
  5: "中五宫",
  6: "乾六宫",
  7: "兑七宫",
  8: "艮八宫",
  9: "离九宫",
} as const satisfies Record<PalaceNumber, string>;
export const DIRECTION_LABELS = {
  north: "正北",
  southwest: "西南",
  east: "正东",
  southeast: "东南",
  center: "中宫",
  northwest: "西北",
  west: "正西",
  northeast: "东北",
  south: "正南",
} as const satisfies Record<Direction, string>;
export const TRIGRAM_LABELS = {
  kan: "坎",
  kun: "坤",
  zhen: "震",
  xun: "巽",
  qian: "乾",
  dui: "兑",
  gen: "艮",
  li: "离",
} as const satisfies Record<Trigram, string>;
export const ELEMENT_LABELS = {
  wood: "木",
  fire: "火",
  earth: "土",
  metal: "金",
  water: "水",
} as const satisfies Record<Element, string>;
export const GROWTH_LABELS = {
  chang_sheng: "长生",
  mu_yu: "沐浴",
  guan_dai: "冠带",
  lin_guan: "临官",
  di_wang: "帝旺",
  shuai: "衰",
  bing: "病",
  si: "死",
  mu: "墓",
  jue: "绝",
  tai: "胎",
  yang: "养",
} as const satisfies Record<GrowthStage, string>;
export const STRENGTH_LABELS = {
  wang: "旺",
  xiang: "相",
  xiu: "休",
  qiu: "囚",
  fei: "废",
  si: "死",
} as const satisfies Record<StarStrengthState | ElementStrengthState, string>;
export const PLATE_LABELS = {
  heaven: "天盘",
  earth: "地盘",
  hidden: "暗干",
} as const satisfies Record<StemPlate, string>;
export const YUAN_LABELS = {
  upper: "上元",
  middle: "中元",
  lower: "下元",
} as const;
export const DUN_LABELS = { yang: "阳遁", yin: "阴遁" } as const;

/** South at the top, matching the traditional Luo Shu chart orientation. */
export const PALACE_GRID_ORDER = [
  4, 9, 2, 3, 5, 7, 8, 1, 6,
] as const satisfies readonly PalaceNumber[];

export function cycleLabel(cycle: Cycle): string {
  return STEM_LABELS[cycle.stem] + BRANCH_LABELS[cycle.branch];
}
