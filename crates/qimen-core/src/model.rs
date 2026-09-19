use qimen_calendar::{Branch, CalendarResult, Cycle, Stem};
use serde::{Deserialize, Serialize};

use crate::{ChartRequest, Deity, Direction, Door, Dun, Element, Method, Star, Trigram, Yuan};

/// The supported center-palace convention.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CenterPalaceRule {
    /// Always host palace five in palace two, for both yang and yin dun.
    AlwaysKun,
}

/// The supported placement of the central star.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TianQinRule {
    /// 天禽 and its center earth stem travel with 天芮.
    FollowsTianRui,
}

/// The pillar used for a chart-level branch annotation.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationBasis {
    /// Use the hour pillar.
    Hour,
}

/// Explicit conventions needed when comparing results between schools/software.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conventions {
    /// Center-palace hosting rule.
    pub center_palace: CenterPalaceRule,
    /// Central-star movement rule.
    pub tian_qin: TianQinRule,
    /// Basis for the void markers on individual palaces.
    pub void_basis: AnnotationBasis,
    /// Basis for the horse marker.
    pub horse_basis: AnnotationBasis,
}

/// The hour's ten-position period and its hidden 甲 stem.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Xun {
    /// 甲子, 甲戌, 甲申, 甲午, 甲辰 or 甲寅.
    pub head: Cycle,
    /// 戊, 己, 庚, 辛, 壬 or 癸, respectively.
    pub hidden_stem: Stem,
    /// The two unpaired branches for this period.
    pub void_branches: [Branch; 2],
}

/// The two void branches of each pillar; palace markers use the hour pair.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PillarVoids {
    /// Year-pillar void branches.
    pub year: [Branch; 2],
    /// Month-pillar void branches.
    pub month: [Branch; 2],
    /// Day-pillar void branches.
    pub day: [Branch; 2],
    /// Hour-pillar void branches.
    pub hour: [Branch; 2],
}

/// Duty star and duty door, preserving the original center palace when relevant.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Leaders {
    /// 值符星; this is 天禽 when the period's hidden stem originated in palace five.
    pub star: Star,
    /// 值使门; palace five uses 死门 under the selected hosting convention.
    pub door: Door,
    /// Earth palace containing the period's hidden stem, before hosting.
    pub original_palace: u8,
    /// Current palace of the duty star, after center hosting.
    pub star_palace: u8,
    /// Current palace of the duty door, after center hosting.
    pub door_palace: u8,
    /// Duty-door numeric flight result before center hosting; may be five.
    pub door_raw_palace: u8,
}

/// A travelling horse; its containing field identifies the source pillar.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Horse {
    /// One of 寅, 申, 巳 or 亥.
    pub branch: Branch,
    /// Luo Shu palace number containing the horse branch.
    pub palace: u8,
}

/// A heavenly-plate stem, including its provenance and center-hosting identity.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeavenStem {
    /// One of the three wonders or six instruments (never visible 甲).
    pub stem: Stem,
    /// Earth-plate palace from which the stem was rotated.
    pub source_palace: u8,
    /// True only for the palace-five stem travelling with 天禽.
    pub is_center_hosted: bool,
}

/// All fixed and moving components of one Luo Shu palace.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Palace {
    /// Luo Shu number, one through nine; the output array is in this order.
    pub number: u8,
    /// Geographic direction (the center has its own value).
    pub direction: Direction,
    /// Later Heaven trigram; absent for the center.
    pub trigram: Option<Trigram>,
    /// Fixed palace element.
    pub element: Element,
    /// Native earth-plate stem, including the actual center stem at palace five.
    pub earth_stem: Stem,
    /// Additional center earth stem hosted at 坤二; absent elsewhere.
    pub hosted_earth_stem: Option<Stem>,
    /// Rotated heavenly-plate stems; two with 天芮/天禽, none at the center.
    pub heaven_stems: Vec<HeavenStem>,
    /// Moving stars; two with 天芮/天禽, none at the center.
    pub stars: Vec<Star>,
    /// Moving door; absent at the center.
    pub door: Option<Door>,
    /// Moving deity; absent at the center.
    pub deity: Option<Deity>,
    /// Hour-pillar void branches contained in this palace; may hold both.
    pub void_branches: Vec<Branch>,
    /// Whether the hour-branch travelling horse occupies this palace.
    pub is_horse: bool,
}

/// A complete, reproducible, language-independent Qimen chart.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chart {
    /// Structural output-schema version, independent of the crate release.
    pub schema_version: String,
    /// The exact validated civil input and its selected calendar conventions.
    pub input: ChartRequest,
    /// Four pillars, surrounding solar terms and lunar date.
    pub calendar: CalendarResult,
    /// Calculation school and plate method.
    pub method: Method,
    /// Explicit center-hosting, star and annotation conventions.
    pub conventions: Conventions,
    /// Yang or yin dun.
    pub dun: Dun,
    /// Five-day upper, middle or lower yuan.
    pub yuan: Yuan,
    /// Formation number, one through nine.
    pub ju: u8,
    /// Five-day 甲/己 head (符头) used to select the yuan.
    pub yuan_head: Cycle,
    /// Hour-period head, hidden stem and void branches.
    pub xun: Xun,
    /// Duty star and door with source and destination palaces.
    pub leaders: Leaders,
    /// Hour-branch travelling horse.
    pub horse: Horse,
    /// Void branches for each of the four pillars.
    pub pillar_voids: PillarVoids,
    /// The nine palaces, always in numeric Luo Shu order, one through nine.
    pub palaces: [Palace; 9],
    /// Opt-in annotations and their named calculation rules; absent by default.
    /// These never alter the base calendar or rotating plates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<crate::ChartExtensions>,
}

impl Chart {
    /// Returns the palace with the supplied Luo Shu number, or `None` outside 1–9.
    #[must_use]
    pub fn palace(&self, number: u8) -> Option<&Palace> {
        number
            .checked_sub(1)
            .and_then(|index| self.palaces.get(usize::from(index)))
    }
}
