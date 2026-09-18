//! Accurate, typed time-based Chai Bu rotating Qimen Dunjia charts.
//!
//! Calendar calculation and plate construction are separated into independent
//! crates. This crate adds the three wonders/six instruments, nine stars, eight
//! doors, eight deities, duty star/door, void branches and travelling horse.
//! Its output records conventions explicitly so another program can reproduce it.
//!
//! ```
//! let request = qimen_core::ChartRequest::new(2026, 9, 18, 14);
//! let chart = qimen_core::calculate(&request)?;
//! assert_eq!(chart.palaces.len(), 9);
//! assert_eq!(chart.schema_version, "1.0");
//! # Ok::<(), qimen_core::Error>(())
//! ```

mod engine;
mod model;
mod symbols;

pub use model::{
    AnnotationBasis, CenterPalaceRule, Chart, Conventions, HeavenStem, Horse, Leaders, Palace,
    PillarVoids, TianQinRule, Xun,
};
pub use qimen_calendar::{
    Branch, CalendarError, CalendarRequest, CalendarResult, CivilDateTime, Cycle, DayBoundary,
    FourPillars, LunarDate, SolarTerm, Stem,
};
pub use symbols::{Deity, Direction, Door, Dun, Element, Method, Star, Trigram, Yuan};

/// Calendar input for the default time-based Chai Bu rotating chart.
pub type ChartRequest = CalendarRequest;

/// Version of the serialized chart structure.
pub const SCHEMA_VERSION: &str = "1.0";

/// An invalid calendar request, malformed JSON, or serialization failure.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid Gregorian date, time, year range or UTC offset.
    #[error(transparent)]
    Calendar(#[from] CalendarError),
    /// Invalid request JSON or a failed JSON serialization.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

/// Calculates the four pillars and complete nine-palace chart.
///
/// The default method is 时家拆补转盘, always hosting the center at 坤二,
/// with 天禽 following 天芮. Input uses the supplied civil clock and fixed UTC
/// offset; no implicit longitude correction or daylight-saving lookup occurs.
/// Invalid input is returned as an error before plate construction.
pub fn calculate(request: &ChartRequest) -> Result<Chart, Error> {
    let calendar = qimen_calendar::calculate(request)?;
    Ok(engine::build(request, calendar))
}

/// Parses a strict JSON request and returns a compact serialized chart.
///
/// Missing `minute` and `second` default to zero, `utc_offset_minutes` to
/// 480 (UTC+08:00), and `day_boundary` to `"zi_start"`. Unknown request fields
/// are errors. Language bindings use this function to share the exact schema.
pub fn calculate_json(request_json: &str) -> Result<String, Error> {
    let request = serde_json::from_str::<ChartRequest>(request_json)?;
    Ok(serde_json::to_string(&calculate(&request)?)?)
}

/// Produces an indented JSON chart for human-readable command-line output.
pub fn calculate_json_pretty(request_json: &str) -> Result<String, Error> {
    let request = serde_json::from_str::<ChartRequest>(request_json)?;
    Ok(serde_json::to_string_pretty(&calculate(&request)?)?)
}

/// Returns the generated JSON Schema for input, including serde defaults.
#[cfg(feature = "schema")]
#[must_use]
pub fn request_schema() -> schemars::Schema {
    schemars::schema_for!(ChartRequest)
}

/// Returns the generated JSON Schema for the complete versioned chart output.
#[cfg(feature = "schema")]
#[must_use]
pub fn chart_schema() -> schemars::Schema {
    schemars::schema_for!(Chart)
}
