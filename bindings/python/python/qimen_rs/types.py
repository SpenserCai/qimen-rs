"""Static Python types for the canonical qimen-core schema 1.1.

JSON arrays remain lists at runtime; no independent conversion or chart logic
is introduced by these definitions. Keep these and Node's schema.d.ts aligned
with the Rust model whenever the canonical schema changes.
"""

from typing import Literal, TypedDict

DayBoundary = Literal["zi_start", "midnight"]
Stem = Literal["jia", "yi", "bing", "ding", "wu", "ji", "geng", "xin", "ren", "gui"]
Branch = Literal["zi", "chou", "yin", "mao", "chen", "si", "wu", "wei", "shen", "you", "xu", "hai"]
PalaceNumber = Literal[1, 2, 3, 4, 5, 6, 7, 8, 9]
Direction = Literal["north", "southwest", "east", "southeast", "center", "northwest", "west", "northeast", "south"]
Trigram = Literal["kan", "kun", "zhen", "xun", "qian", "dui", "gen", "li"]
Element = Literal["wood", "fire", "earth", "metal", "water"]
Star = Literal["tian_peng", "tian_rui", "tian_chong", "tian_fu", "tian_qin", "tian_xin", "tian_zhu", "tian_ren", "tian_ying"]
Door = Literal["xiu", "si", "shang", "du", "kai", "jing", "sheng", "scene"]
Deity = Literal["zhi_fu", "teng_she", "tai_yin", "liu_he", "bai_hu", "xuan_wu", "jiu_di", "jiu_tian"]
HiddenStemRule = Literal["duty_door_hour_stem_with_center_fallback"]
StrengthRule = Literal["classical_stars_and_five_elements"]
GrowthRule = Literal["yang_forward_yin_reverse_fire_earth"]
PunishmentRule = Literal["six_instrument_branches"]
TombRule = Literal["growth_stage_fire_earth", "traditional_three_wonders"]
DayHorseRule = Literal["day_branch_three_harmony"]
DoorPressureRule = Literal["door_controls_palace"]


class ExtensionOptions(TypedDict, total=False):
    """Explicit annotation conventions; omitted or null rules are disabled."""

    hidden_stems: HiddenStemRule | None
    strength: StrengthRule | None
    growth_stages: GrowthRule | None
    punishments: PunishmentRule | None
    tombs: TombRule | None
    day_horse: DayHorseRule | None
    door_pressure: DoorPressureRule | None


class _RequiredRequest(TypedDict):
    year: int
    month: int
    day: int
    hour: int


class ChartRequest(_RequiredRequest, total=False):
    """Civil time; optional field defaults belong to the Rust engine."""

    minute: int
    second: int
    utc_offset_minutes: int
    day_boundary: DayBoundary
    extensions: ExtensionOptions


class ValidatedInput(_RequiredRequest):
    minute: int
    second: int
    utc_offset_minutes: int
    day_boundary: DayBoundary


class CivilDateTime(_RequiredRequest):
    minute: int
    second: int
    utc_offset_minutes: int


class Cycle(TypedDict):
    index: int
    stem: Stem
    branch: Branch


class FourPillars(TypedDict):
    year: Cycle
    month: Cycle
    day: Cycle
    hour: Cycle


class SolarTerm(TypedDict):
    index: int
    name: str
    start: CivilDateTime


class LunarDate(TypedDict):
    year: int
    month: int
    day: int
    is_leap_month: bool
    name: str


class CalendarResult(TypedDict):
    four_pillars: FourPillars
    solar_term: SolarTerm
    next_solar_term: SolarTerm
    lunar_date: LunarDate


class Conventions(TypedDict):
    center_palace: Literal["always_kun"]
    tian_qin: Literal["follows_tian_rui"]
    void_basis: Literal["hour"]
    horse_basis: Literal["hour"]


class Xun(TypedDict):
    head: Cycle
    hidden_stem: Stem
    void_branches: list[Branch]


class PillarVoids(TypedDict):
    year: list[Branch]
    month: list[Branch]
    day: list[Branch]
    hour: list[Branch]


class Leaders(TypedDict):
    star: Star
    door: Door
    original_palace: PalaceNumber
    star_palace: PalaceNumber
    door_palace: PalaceNumber
    door_raw_palace: PalaceNumber


class Horse(TypedDict):
    branch: Branch
    palace: PalaceNumber


class HeavenStem(TypedDict):
    stem: Stem
    source_palace: PalaceNumber
    is_center_hosted: bool


class Palace(TypedDict):
    number: PalaceNumber
    direction: Direction
    trigram: Trigram | None
    element: Element
    earth_stem: Stem
    hosted_earth_stem: Stem | None
    heaven_stems: list[HeavenStem]
    stars: list[Star]
    door: Door | None
    deity: Deity | None
    void_branches: list[Branch]
    is_horse: bool


StemPlate = Literal["heaven", "earth", "hidden"]
StarStrengthState = Literal["wang", "xiang", "xiu", "qiu", "fei"]
ElementStrengthState = Literal["wang", "xiang", "xiu", "qiu", "si"]
GrowthStage = Literal["chang_sheng", "mu_yu", "guan_dai", "lin_guan", "di_wang", "shuai", "bing", "si", "mu", "jue", "tai", "yang"]


class StemPlacement(TypedDict):
    """A stem occurrence, retaining its plate and hosted-center provenance."""

    palace: PalaceNumber
    plate: StemPlate
    stem: Stem
    source_palace: PalaceNumber | None
    is_center_hosted: bool


class HiddenStem(TypedDict):
    palace: PalaceNumber
    stem: Stem


class HiddenStems(TypedDict):
    rule: HiddenStemRule
    effective_hour_stem: Stem
    start_palace: PalaceNumber
    used_center_fallback: bool
    palaces: list[HiddenStem]


class StarStrength(TypedDict):
    star: Star
    element: Element
    at_palace: StarStrengthState
    at_month: StarStrengthState


class DoorStrength(TypedDict):
    door: Door
    element: Element
    at_palace: ElementStrengthState
    at_month: ElementStrengthState


class StemStrength(TypedDict):
    placement: StemPlacement
    element: Element
    at_palace: ElementStrengthState
    at_month: ElementStrengthState


class PalaceStrength(TypedDict):
    palace: PalaceNumber
    element: Element
    stars: list[StarStrength]
    door: DoorStrength | None
    stems: list[StemStrength]


class Strengths(TypedDict):
    rule: StrengthRule
    month_branch: Branch
    month_element: Element
    palaces: list[PalaceStrength]


class BranchGrowth(TypedDict):
    branch: Branch
    stage: GrowthStage


class StemGrowth(TypedDict):
    """Corner-palace branches remain separate; center branches are empty."""

    placement: StemPlacement
    branches: list[BranchGrowth]


class GrowthStages(TypedDict):
    rule: GrowthRule
    stems: list[StemGrowth]


class StemPunishment(TypedDict):
    placement: StemPlacement
    hidden_jia: Cycle | None
    punished_branch: Branch | None
    is_punished: bool


class Punishments(TypedDict):
    rule: PunishmentRule
    stems: list[StemPunishment]


class StemTomb(TypedDict):
    """None means the selected rule does not apply, distinct from False."""

    placement: StemPlacement
    tomb_branch: Branch | None
    is_in_tomb: bool | None


class Tombs(TypedDict):
    rule: TombRule
    stems: list[StemTomb]


class DayHorse(TypedDict):
    rule: DayHorseRule
    pillar: Cycle
    horse: Horse


class DoorPressure(TypedDict):
    palace: PalaceNumber
    door: Door
    door_element: Element
    palace_element: Element
    is_pressed: bool


class DoorPressures(TypedDict):
    rule: DoorPressureRule
    doors: list[DoorPressure]


class ChartExtensions(TypedDict, total=False):
    """Only requested annotations are present; each result records its rule."""

    hidden_stems: HiddenStems
    strength: Strengths
    growth_stages: GrowthStages
    punishments: Punishments
    tombs: Tombs
    day_horse: DayHorse
    door_pressure: DoorPressures


class _OptionalChart(TypedDict, total=False):
    extensions: ChartExtensions


class Chart(_OptionalChart):
    schema_version: str
    input: ValidatedInput
    calendar: CalendarResult
    method: Literal["shi_jia_chai_bu_zhuan_pan"]
    conventions: Conventions
    dun: Literal["yang", "yin"]
    yuan: Literal["upper", "middle", "lower"]
    ju: PalaceNumber
    yuan_head: Cycle
    xun: Xun
    leaders: Leaders
    horse: Horse
    pillar_voids: PillarVoids
    palaces: list[Palace]
