use serde::{Deserialize, Serialize};

/// The optional annotations to calculate after constructing a base chart.
///
/// Every field defaults to disabled. Each selected rule is copied into the
/// output, so a caller can distinguish school conventions from chart facts.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ExtensionOptions {
    /// 暗干: fly the hour stem from the duty-door palace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_stems: Option<HiddenStemRule>,
    /// 星、门、干旺衰, evaluated against both the palace and solar month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<StrengthRule>,
    /// 十二长生 for every stem and every branch in its palace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_stages: Option<GrowthRule>,
    /// 六仪击刑, preserving heavenly/earthly and hosted-stem identities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punishments: Option<PunishmentRule>,
    /// 天干入墓 under the explicitly selected convention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tombs: Option<TombRule>,
    /// Day-pillar travelling horse, independent of the base chart's hour horse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_horse: Option<DayHorseRule>,
    /// 门迫: a door's element controls its destination palace's element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door_pressure: Option<DoorPressureRule>,
}

impl ExtensionOptions {
    /// Whether all optional annotations are disabled.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.hidden_stems.is_none()
            && self.strength.is_none()
            && self.growth_stages.is_none()
            && self.punishments.is_none()
            && self.tombs.is_none()
            && self.day_horse.is_none()
            && self.door_pressure.is_none()
    }

    /// Enables every implemented annotation with the documented conventions.
    ///
    /// This is an explicit convenience preset, not a claim that these are the
    /// only traditional rules. In particular, tombs follow the twelve stages;
    /// classical 三奇入墓 may use a different location for 乙.
    #[must_use]
    pub const fn all() -> Self {
        Self {
            hidden_stems: Some(HiddenStemRule::DutyDoorHourStemWithCenterFallback),
            strength: Some(StrengthRule::ClassicalStarsAndFiveElements),
            growth_stages: Some(GrowthRule::YangForwardYinReverseFireEarth),
            punishments: Some(PunishmentRule::SixInstrumentBranches),
            tombs: Some(TombRule::GrowthStageFireEarth),
            day_horse: Some(DayHorseRule::DayBranchThreeHarmony),
            door_pressure: Some(DoorPressureRule::DoorControlsPalace),
        }
    }
}

/// Supported 暗干 construction conventions.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HiddenStemRule {
    /// Put the hour stem at the duty door, then fly the nine instruments in
    /// numeric palace order (yang forward, yin backward). If it repeats that
    /// palace's earth stem, start at palace five. 甲 uses its hidden instrument.
    DutyDoorHourStemWithCenterFallback,
}

/// Supported element-strength conventions.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StrengthRule {
    /// Stars use 生我为废、我生为旺、同我为相、我克为休、克我为囚.
    /// Doors/stems use ordinary 旺相休囚死. Both evaluate the palace element
    /// and the element of the solar-month pillar's branch (not lunar month).
    ClassicalStarsAndFiveElements,
}

/// Supported twelve-stage growth conventions.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrowthRule {
    /// Yang stems progress forward, yin stems backward; 戊 follows 丙 and
    /// 己 follows 丁. Every branch in a double-branch palace remains separate.
    YangForwardYinReverseFireEarth,
}

/// Supported 六仪击刑 conventions.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PunishmentRule {
    /// 戊/己/庚/辛/壬/癸 at 卯/未/寅/午/辰/巳 respectively.
    /// Heavenly-plate instruments are the traditional primary application;
    /// earthly and hidden plates are separately identified geometric annotations.
    SixInstrumentBranches,
}

/// Supported stem-tomb conventions; these must not be conflated.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TombRule {
    /// Use 墓 in the yang-forward/yin-reverse twelve stages, with earth
    /// following fire. In this convention 乙's tomb is 戌, not 未.
    GrowthStageFireEarth,
    /// Classical 三奇入墓 only: 乙→未、丙→戌、丁→丑. Other stems
    /// are explicitly not applicable; this does not supply a six-instrument rule.
    TraditionalThreeWonders,
}

/// Supported day-horse conventions.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DayHorseRule {
    /// 申子辰→寅、寅午戌→申、亥卯未→巳、巳酉丑→亥, using the
    /// calculated day pillar and therefore the request's day-boundary rule.
    DayBranchThreeHarmony,
}

/// Supported door-pressure conventions.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoorPressureRule {
    /// Door element controls palace element; the reverse is not 门迫.
    DoorControlsPalace,
}
