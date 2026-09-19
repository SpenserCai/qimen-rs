use serde::{Deserialize, Serialize};

use super::{
    DayHorseRule, DoorPressureRule, GrowthRule, HiddenStemRule, PunishmentRule, StrengthRule,
    TombRule,
};
use crate::{Branch, Cycle, Door, Element, Horse, Star, Stem};

/// Optional annotations. Absent fields were not requested, rather than negative.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChartExtensions {
    /// Nine-palace 暗干 with its construction convention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_stems: Option<HiddenStems>,
    /// Star, door and stem strengths against palace and solar month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<Strengths>,
    /// Stem growth stages at each constituent branch of a palace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_stages: Option<GrowthStages>,
    /// Six-instrument punishment annotations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punishments: Option<Punishments>,
    /// Stem tomb annotations with an explicit convention and applicability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tombs: Option<Tombs>,
    /// Day-pillar horse, preserving the base chart's separate hour horse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_horse: Option<DayHorse>,
    /// Door-over-palace element control; excludes the reverse relationship.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_pressure: Option<DoorPressures>,
}

/// A plate carrying a stem annotation.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StemPlate {
    /// Rotating heavenly plate.
    Heaven,
    /// Fixed earthly plate, including the explicitly identified hosted center.
    Earth,
    /// Optional nine-palace 暗干 plate; included only when requested.
    Hidden,
}

/// Identity of a stem occurrence; hosted stems must not be silently collapsed.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StemPlacement {
    /// Destination Luo Shu palace, one through nine.
    pub palace: u8,
    /// Heavenly, earthly or optional hidden plate.
    pub plate: StemPlate,
    /// Heavenly stem.
    pub stem: Stem,
    /// Source earth palace; absent for constructed hidden-plate stems.
    pub source_palace: Option<u8>,
    /// Whether this is an additional hosted occurrence of the center stem.
    pub is_center_hosted: bool,
}

/// A constructed hidden stem in one numeric palace.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenStem {
    /// Luo Shu palace number.
    pub palace: u8,
    /// Hidden-plate stem.
    pub stem: Stem,
}

/// Nine-palace hidden-stem calculation and its actual starting point.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenStems {
    /// Selected construction convention.
    pub rule: HiddenStemRule,
    /// Hour stem, with 甲 replaced by its period's hidden instrument.
    pub effective_hour_stem: Stem,
    /// Actual initial numeric palace, after the repetition fallback.
    pub start_palace: u8,
    /// Whether repetition with the duty door's earth stem moved the start to five.
    pub used_center_fallback: bool,
    /// Exactly nine records, in numeric palace order.
    pub palaces: [HiddenStem; 9],
}

macro_rules! states {
    ($(#[$meta:meta])* $name:ident { $($(#[$variant_meta:meta])* $variant:ident => $label:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name { $($(#[$variant_meta])* $variant),+ }

        impl $name {
            /// Traditional Chinese label.
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self { $(Self::$variant => $label),+ }
            }
        }
    };
}

states! {
    /// Nine-star strength; 废 is distinct from the ordinary element's 死.
    StarStrengthState {
        /// The star generates the environment: 旺.
        Wang => "旺",
        /// The star and environment have the same element: 相.
        Xiang => "相",
        /// The star controls the environment: 休.
        Xiu => "休",
        /// The environment controls the star: 囚.
        Qiu => "囚",
        /// The environment generates the star: 废.
        Fei => "废",
    }
}

states! {
    /// Ordinary five-element strength, used for doors and stems.
    ElementStrengthState {
        /// Same element as the environment: 旺.
        Wang => "旺",
        /// Generated by the environment: 相.
        Xiang => "相",
        /// Generates the environment: 休.
        Xiu => "休",
        /// Controls the environment: 囚.
        Qiu => "囚",
        /// Controlled by the environment: 死.
        Si => "死",
    }
}

/// Strength of a moving star under both explicitly recorded environments.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarStrength {
    /// Star identity, including a separate 天禽 entry.
    pub star: Star,
    /// Star's intrinsic element.
    pub element: Element,
    /// Relationship to the destination palace's element.
    pub at_palace: StarStrengthState,
    /// Relationship to the solar-month branch's element.
    pub at_month: StarStrengthState,
}

/// Strength of a moving door.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoorStrength {
    /// Door identity.
    pub door: Door,
    /// Door's intrinsic element.
    pub element: Element,
    /// Relationship to the destination palace's element.
    pub at_palace: ElementStrengthState,
    /// Relationship to the solar-month branch's element.
    pub at_month: ElementStrengthState,
}

/// Strength of one stem occurrence.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StemStrength {
    /// Stem and plate provenance.
    pub placement: StemPlacement,
    /// Stem's intrinsic element.
    pub element: Element,
    /// Relationship to the destination palace's element.
    pub at_palace: ElementStrengthState,
    /// Relationship to the solar-month branch's element.
    pub at_month: ElementStrengthState,
}

/// All strength annotations for a single palace.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PalaceStrength {
    /// Luo Shu palace number.
    pub palace: u8,
    /// Fixed destination-palace element.
    pub element: Element,
    /// Separate annotations for every star, including 天禽.
    pub stars: Vec<StarStrength>,
    /// Door annotation, absent at the center.
    pub door: Option<DoorStrength>,
    /// Every earthly/heavenly stem; hidden stems are included when enabled.
    pub stems: Vec<StemStrength>,
}

/// Strength annotations with the exact seasonal environment used.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Strengths {
    /// Selected rule, distinguishing star strength from ordinary element strength.
    pub rule: StrengthRule,
    /// Branch of the solar-month pillar; never inferred from lunar month number.
    pub month_branch: Branch,
    /// Element of the solar-month branch. 辰戌丑未 are earth; no implicit
    /// eighteen-day seasonal-transition adjustment is applied.
    pub month_element: Element,
    /// All nine palaces in numeric order.
    pub palaces: Vec<PalaceStrength>,
}

states! {
    /// One of the twelve stages of stem growth.
    GrowthStage {
        /// 长生.
        ChangSheng => "长生",
        /// 沐浴.
        MuYu => "沐浴",
        /// 冠带.
        GuanDai => "冠带",
        /// 临官.
        LinGuan => "临官",
        /// 帝旺.
        DiWang => "帝旺",
        /// 衰.
        Shuai => "衰",
        /// 病.
        Bing => "病",
        /// 死.
        Si => "死",
        /// 墓.
        Mu => "墓",
        /// 绝.
        Jue => "绝",
        /// 胎.
        Tai => "胎",
        /// 养.
        Yang => "养",
    }
}

/// One growth stage at one concrete palace branch.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchGrowth {
    /// The actual palace branch; double-branch palaces retain both entries.
    pub branch: Branch,
    /// Stage at that branch under the selected growth convention.
    pub stage: GrowthStage,
}

/// Growth stages for one stem occurrence.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StemGrowth {
    /// Stem and plate provenance.
    pub placement: StemPlacement,
    /// Zero branches at center five; one or two elsewhere.
    pub branches: Vec<BranchGrowth>,
}

/// Twelve-stage results and the polarity/earth convention used.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrowthStages {
    /// Explicit growth convention.
    pub rule: GrowthRule,
    /// Every earthly/heavenly stem; hidden stems are included when enabled.
    pub stems: Vec<StemGrowth>,
}

/// Punishment evaluation for one stem occurrence.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StemPunishment {
    /// Stem and plate provenance.
    pub placement: StemPlacement,
    /// The six instrument's hidden 甲 pillar; absent for the three wonders.
    pub hidden_jia: Option<Cycle>,
    /// Palace branch struck by this instrument; absent when not applicable.
    pub punished_branch: Option<Branch>,
    /// Whether the actual destination palace contains that branch.
    pub is_punished: bool,
}

/// 六仪击刑 evaluations, with plate identities retained.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Punishments {
    /// Selected punishment convention.
    pub rule: PunishmentRule,
    /// Heavenly instruments are primary; other plate results are explicitly typed.
    pub stems: Vec<StemPunishment>,
}

/// Tomb evaluation for one stem occurrence.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StemTomb {
    /// Stem and plate provenance.
    pub placement: StemPlacement,
    /// Branch containing the tomb; `None` when the chosen rule does not cover
    /// this stem (for example a six instrument under the three-wonders rule).
    pub tomb_branch: Option<Branch>,
    /// Whether the palace contains the tomb branch; `None` means not applicable.
    pub is_in_tomb: Option<bool>,
}

/// Tomb evaluations; applicability remains distinct from a negative result.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tombs {
    /// Selected tomb convention.
    pub rule: TombRule,
    /// Every stem with explicit rule applicability.
    pub stems: Vec<StemTomb>,
}

/// Optional day horse, independent of the existing hour-based palace marker.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayHorse {
    /// Selected derivation convention.
    pub rule: DayHorseRule,
    /// Exact day pillar after applying the request's day-boundary convention.
    pub pillar: Cycle,
    /// Day horse branch and its palace.
    pub horse: Horse,
}

/// Door pressure for a single occupied door palace.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoorPressure {
    /// Destination Luo Shu palace number.
    pub palace: u8,
    /// Moving door identity.
    pub door: Door,
    /// Intrinsic door element.
    pub door_element: Element,
    /// Destination palace element.
    pub palace_element: Element,
    /// True only when the door's element controls the palace's element.
    pub is_pressed: bool,
}

/// Door-pressure results for all eight doors.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoorPressures {
    /// Selected control-direction convention.
    pub rule: DoorPressureRule,
    /// Eight door entries in numeric destination-palace order.
    pub doors: Vec<DoorPressure>,
}
