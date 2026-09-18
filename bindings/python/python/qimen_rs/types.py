"""Static Python types for the canonical qimen-core schema 1.0.

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


class Chart(TypedDict):
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
