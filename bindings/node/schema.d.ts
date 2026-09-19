/** Canonical qimen-core schema 1.1; field names are shared across all bindings. */
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
export type HiddenStemRule = 'duty_door_hour_stem_with_center_fallback';
export type StrengthRule = 'classical_stars_and_five_elements';
export type GrowthRule = 'yang_forward_yin_reverse_fire_earth';
export type PunishmentRule = 'six_instrument_branches';
export type TombRule = 'growth_stage_fire_earth' | 'traditional_three_wonders';
export type DayHorseRule = 'day_branch_three_harmony';
export type DoorPressureRule = 'door_controls_palace';

/** Omitted or null rules are disabled. Selected conventions are echoed in the result. */
export interface ExtensionOptions {
  hidden_stems?: HiddenStemRule | null;
  strength?: StrengthRule | null;
  growth_stages?: GrowthRule | null;
  punishments?: PunishmentRule | null;
  tombs?: TombRule | null;
  day_horse?: DayHorseRule | null;
  door_pressure?: DoorPressureRule | null;
}

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
  /** Optional annotations; all are disabled by default. */
  extensions?: ExtensionOptions;
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

export type StemPlate = 'heaven' | 'earth' | 'hidden';
export type StarStrengthState = 'wang' | 'xiang' | 'xiu' | 'qiu' | 'fei';
export type ElementStrengthState = 'wang' | 'xiang' | 'xiu' | 'qiu' | 'si';
export type GrowthStage = 'chang_sheng' | 'mu_yu' | 'guan_dai' | 'lin_guan' | 'di_wang' | 'shuai' | 'bing' | 'si' | 'mu' | 'jue' | 'tai' | 'yang';

/** A single stem occurrence, retaining plate and hosted-center provenance. */
export interface StemPlacement {
  palace: PalaceNumber;
  plate: StemPlate;
  stem: Stem;
  source_palace: PalaceNumber | null;
  is_center_hosted: boolean;
}

export interface HiddenStem {
  palace: PalaceNumber;
  stem: Stem;
}

export interface HiddenStems {
  rule: HiddenStemRule;
  effective_hour_stem: Stem;
  start_palace: PalaceNumber;
  used_center_fallback: boolean;
  palaces: [HiddenStem, HiddenStem, HiddenStem, HiddenStem, HiddenStem, HiddenStem, HiddenStem, HiddenStem, HiddenStem];
}

export interface StarStrength {
  star: Star;
  element: Element;
  at_palace: StarStrengthState;
  at_month: StarStrengthState;
}

export interface DoorStrength {
  door: Door;
  element: Element;
  at_palace: ElementStrengthState;
  at_month: ElementStrengthState;
}

export interface StemStrength {
  placement: StemPlacement;
  element: Element;
  at_palace: ElementStrengthState;
  at_month: ElementStrengthState;
}

export interface PalaceStrength {
  palace: PalaceNumber;
  element: Element;
  stars: StarStrength[];
  door: DoorStrength | null;
  stems: StemStrength[];
}

export interface Strengths {
  rule: StrengthRule;
  month_branch: Branch;
  month_element: Element;
  palaces: PalaceStrength[];
}

export interface BranchGrowth {
  branch: Branch;
  stage: GrowthStage;
}

export interface StemGrowth {
  placement: StemPlacement;
  /** Separate entries for both branches of a corner palace; empty at center. */
  branches: BranchGrowth[];
}

export interface GrowthStages {
  rule: GrowthRule;
  stems: StemGrowth[];
}

export interface StemPunishment {
  placement: StemPlacement;
  hidden_jia: Cycle | null;
  punished_branch: Branch | null;
  is_punished: boolean;
}

export interface Punishments {
  rule: PunishmentRule;
  stems: StemPunishment[];
}

export interface StemTomb {
  placement: StemPlacement;
  /** Null when the selected convention does not apply to this stem. */
  tomb_branch: Branch | null;
  /** Null means not applicable, distinct from false. */
  is_in_tomb: boolean | null;
}

export interface Tombs {
  rule: TombRule;
  stems: StemTomb[];
}

export interface DayHorse {
  rule: DayHorseRule;
  pillar: Cycle;
  horse: Horse;
}

export interface DoorPressure {
  palace: PalaceNumber;
  door: Door;
  door_element: Element;
  palace_element: Element;
  is_pressed: boolean;
}

export interface DoorPressures {
  rule: DoorPressureRule;
  doors: DoorPressure[];
}

/** Only requested annotations are present; each result records its rule. */
export interface ChartExtensions {
  hidden_stems?: HiddenStems;
  strength?: Strengths;
  growth_stages?: GrowthStages;
  punishments?: Punishments;
  tombs?: Tombs;
  day_horse?: DayHorse;
  door_pressure?: DoorPressures;
}

export interface Chart {
  schema_version: string;
  input: Required<Omit<ChartRequest, 'extensions'>>;
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
  /** Absent when no annotations are enabled. */
  extensions?: ChartExtensions;
}
