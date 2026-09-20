//! Proleptic Gregorian calendar, solar terms and Four Pillars for civil times.
//!
//! Year and month pillars change at the calculated solar-term instant. The
//! supplied fixed UTC offset identifies that instant; day and hour pillars use
//! the supplied local civil clock. No longitude or apparent-solar-time correction
//! is applied. Lunar dates are the Chinese calendar label for the local date.
//!
//! Solar terms are calculated by `tyme4rs` (Shou Xing astronomy). Second-level
//! output is computational resolution, not a claim of one-second astronomical
//! accuracy. Public inputs cover proleptic Gregorian years 1–9999, including
//! the dates 1582-10-05 through 1582-10-14. Adjacent solar-term timestamps can
//! fall in years 0 or 10000. Historical dates are not Julian-calendar labels.

mod calculation;
mod civil;
mod cycle;
mod lunar;
mod model;

pub use calculation::calculate;
pub use cycle::{Branch, Cycle, Stem};
pub use model::{
    CalendarError, CalendarRequest, CalendarResult, CivilDateTime, DayBoundary, FourPillars,
    LunarDate, SolarTerm,
};
