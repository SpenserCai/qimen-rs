use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::Cycle;

/// Civil-clock convention for the day pillar.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum DayBoundary {
    /// Change the day pillar at 23:00, at the start of 子时.
    #[default]
    ZiStart,
    /// Change the day pillar at 00:00. The 23:00 子 hour still uses the following
    /// day's stem for its hour pillar, preserving one pillar across 子时.
    Midnight,
}

impl<'de> Deserialize<'de> for DayBoundary {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // The public schema uses a string enum, not Serde's alternative
        // single-key map representation for externally tagged unit variants.
        match String::deserialize(deserializer)?.as_str() {
            "zi_start" => Ok(Self::ZiStart),
            "midnight" => Ok(Self::Midnight),
            value => Err(serde::de::Error::unknown_variant(
                value,
                &["zi_start", "midnight"],
            )),
        }
    }
}

const fn default_offset() -> i32 {
    480
}

/// Gregorian input and explicit calculation conventions.
///
/// Missing JSON `minute` and `second` default to zero, `utc_offset_minutes` to
/// 480 (UTC+08:00), and `day_boundary` to `zi_start`. Unknown fields are rejected
/// so a misspelled convention cannot silently change the result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CalendarRequest {
    /// Gregorian year, inclusive range 1900–2100.
    pub year: i32,
    /// Gregorian month, 1–12.
    pub month: u32,
    /// Gregorian day, validated against the month and year.
    pub day: u32,
    /// Local civil hour, 0–23.
    pub hour: u32,
    /// Local civil minute, 0–59.
    #[serde(default)]
    pub minute: u32,
    /// Local civil second, 0–59; leap seconds are not accepted.
    #[serde(default)]
    pub second: u32,
    /// Fixed offset east of UTC, in minutes, from −840 through +840.
    /// Supply the offset actually applicable at the requested date, including DST.
    #[serde(default = "default_offset")]
    pub utc_offset_minutes: i32,
    /// Day-pillar rollover convention.
    #[serde(default)]
    pub day_boundary: DayBoundary,
}

impl CalendarRequest {
    /// Creates a request at the given hour, in UTC+08:00 with 子初 day rollover.
    /// Validation is performed by [`crate::calculate`].
    #[must_use]
    pub const fn new(year: i32, month: u32, day: u32, hour: u32) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute: 0,
            second: 0,
            utc_offset_minutes: 480,
            day_boundary: DayBoundary::ZiStart,
        }
    }
}

/// An unambiguous second-resolution civil timestamp with a fixed UTC offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CivilDateTime {
    /// Gregorian year.
    pub year: i32,
    /// Gregorian month, 1–12.
    pub month: u32,
    /// Gregorian day, 1–31.
    pub day: u32,
    /// Local hour, 0–23.
    pub hour: u32,
    /// Local minute, 0–59.
    pub minute: u32,
    /// Local second, 0–59.
    pub second: u32,
    /// Minutes east of UTC.
    pub utc_offset_minutes: i32,
}

/// The four sexagenary pillars, with exact term boundaries for year and month.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FourPillars {
    /// Year pillar, changing at 立春.
    pub year: Cycle,
    /// Month pillar, changing at the twelve 节 instants.
    pub month: Cycle,
    /// Day pillar under the selected local-clock rollover convention.
    pub day: Cycle,
    /// Hour pillar; 子时 begins at 23:00.
    pub hour: Cycle,
}

/// One solar-term boundary in the request's fixed UTC offset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SolarTerm {
    /// Traditional index: 冬至 = 0, 小寒 = 1, …, 大雪 = 23.
    pub index: u8,
    /// Chinese name.
    pub name: String,
    /// Calculated start instant, at second resolution.
    pub start: CivilDateTime,
}

/// Chinese lunar calendar label for the supplied local Gregorian date.
///
/// This is the conventional Chinese calendar date conversion, not an
/// independent astronomical lunar calendar recalculated for the input timezone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct LunarDate {
    /// Lunar calendar year.
    pub year: i32,
    /// Lunar month number, 1–12.
    pub month: u32,
    /// Lunar day number, 1–30.
    pub day: u32,
    /// Whether the month is an intercalary month.
    pub is_leap_month: bool,
    /// Traditional Chinese display label.
    pub name: String,
}

/// Calendar data required for a reproducible Qimen chart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CalendarResult {
    /// Computed year, month, day and hour pillars.
    pub four_pillars: FourPillars,
    /// Most recent solar term; equality belongs to this new term.
    pub solar_term: SolarTerm,
    /// The following solar term, useful for boundary verification.
    pub next_solar_term: SolarTerm,
    /// Chinese lunar calendar label for the input civil date.
    pub lunar_date: LunarDate,
}

/// A rejected input or calendar conversion failure.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CalendarError {
    /// The public, tested Gregorian year range was exceeded.
    #[error("year must be between 1900 and 2100 inclusive, got {0}")]
    UnsupportedYear(i32),
    /// The supplied civil offset exceeds the accepted range.
    #[error("utc_offset_minutes must be between -840 and 840 inclusive, got {0}")]
    InvalidUtcOffset(i32),
    /// An invalid Gregorian date or civil time was supplied.
    #[error("invalid Gregorian date or time: {0}")]
    InvalidDateTime(String),
}
