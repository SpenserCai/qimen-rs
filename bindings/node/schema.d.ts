/** Canonical qimen-core schema 1.0; field names are shared across all bindings. */
export type DayBoundary = 'zi_start' | 'midnight';
export type Stem = 'jia' | 'yi' | 'bing' | 'ding' | 'wu' | 'ji' | 'geng' | 'xin' | 'ren' | 'gui';
export type Branch = 'zi' | 'chou' | 'yin' | 'mao' | 'chen' | 'si' | 'wu' | 'wei' | 'shen' | 'you' | 'xu' | 'hai';
export type PalaceNumber = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9;
export type Direction = 'north' | 'southwest' | 'east' | 'southeast' | 'center' | 'northwest' | 'west' | 'northeast' | 'south';
export type Trigram = 'kan' | 'kun' | 'zhen' | 'xun' | 'qian' | 'dui' | 'gen' | 'li';
export type Element = 'wood' | 'fire' | 'earth' | 'metal' | 'water';
export type Star = 'tian_peng' | 'tian_rui' | 'tian_chong' | 'tian_fu' | 'tian_qin' | 'tian_xin' | 'tian_zhu' | 'tian_ren' | 'tian_ying';
/** jing = 惊门; scene = 景门. */
export type Door = 'xiu' | 'si' | 'shang' | 'du' | 'kai' | 'jing' | 'sheng' | 'scene';
export type Deity = 'zhi_fu' | 'teng_she' | 'tai_yin' | 'liu_he' | 'bai_hu' | 'xuan_wu' | 'jiu_di' | 'jiu_tian';

export interface ChartRequest {
  /** Gregorian year, 1900–2100 inclusive. */
  year: number;
  month: number;
  day: number;
  hour: number;
  /** Defaults to zero. */
  minute?: number;
  /** Defaults to zero; leap seconds are not accepted. */
  second?: number;
  /** Fixed offset east of UTC in minutes, -840 through 840; default 480. */
  utc_offset_minutes?: number;
  /** Defaults to zi_start (23:00). */
  day_boundary?: DayBoundary;
}

export interface CivilDateTime {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  utc_offset_minutes: number;
}

export interface Cycle {
  /** Zero-based sexagenary index, 甲子 = 0, 癸亥 = 59. */
  index: number;
  stem: Stem;
  branch: Branch;
}

export interface FourPillars {
  year: Cycle;
  month: Cycle;
  day: Cycle;
  hour: Cycle;
}

export interface SolarTerm {
  /** 冬至 = 0, 小寒 = 1, …, 大雪 = 23. */
  index: number;
  name: string;
  start: CivilDateTime;
}

export interface LunarDate {
  year: number;
  month: number;
  day: number;
  is_leap_month: boolean;
  name: string;
}

export interface CalendarResult {
  four_pillars: FourPillars;
  solar_term: SolarTerm;
  next_solar_term: SolarTerm;
  lunar_date: LunarDate;
}

export interface Conventions {
  center_palace: 'always_kun';
  tian_qin: 'follows_tian_rui';
  void_basis: 'hour';
  horse_basis: 'hour';
}

export interface Xun {
  head: Cycle;
  hidden_stem: Stem;
  void_branches: [Branch, Branch];
}

export interface PillarVoids {
  year: [Branch, Branch];
  month: [Branch, Branch];
  day: [Branch, Branch];
  hour: [Branch, Branch];
}

export interface Leaders {
  star: Star;
  door: Door;
  original_palace: PalaceNumber;
  star_palace: PalaceNumber;
  door_palace: PalaceNumber;
  door_raw_palace: PalaceNumber;
}

export interface Horse {
  branch: Branch;
  palace: PalaceNumber;
}

export interface HeavenStem {
  stem: Stem;
  source_palace: PalaceNumber;
  is_center_hosted: boolean;
}

export interface Palace {
  number: PalaceNumber;
  direction: Direction;
  trigram: Trigram | null;
  element: Element;
  earth_stem: Stem;
  hosted_earth_stem: Stem | null;
  heaven_stems: HeavenStem[];
  stars: Star[];
  door: Door | null;
  deity: Deity | null;
  void_branches: Branch[];
  is_horse: boolean;
}

export interface Chart {
  schema_version: string;
  input: Required<ChartRequest>;
  calendar: CalendarResult;
  method: 'shi_jia_chai_bu_zhuan_pan';
  conventions: Conventions;
  dun: 'yang' | 'yin';
  yuan: 'upper' | 'middle' | 'lower';
  ju: PalaceNumber;
  yuan_head: Cycle;
  xun: Xun;
  leaders: Leaders;
  horse: Horse;
  pillar_voids: PillarVoids;
  /** Exactly nine palaces, in Luo Shu numeric order, 1 through 9. */
  palaces: [Palace, Palace, Palace, Palace, Palace, Palace, Palace, Palace, Palace];
}
