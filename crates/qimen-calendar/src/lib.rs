//! Gregorian calendar, solar terms and Four Pillars for civil times.
//!
//! Year and month pillars change at the calculated solar-term instant. The
//! supplied fixed UTC offset identifies that instant; day and hour pillars use
//! the supplied local civil clock. No longitude or apparent-solar-time correction
//! is applied. Lunar dates are the Chinese calendar label for the local date.
//!
//! Solar terms are calculated by `tyme4rs` (Shou Xing astronomy). Second-level
//! output is computational resolution, not a claim of one-second astronomical
//! accuracy. Public inputs are restricted to Gregorian years 1900–2100.

mod calculation;
mod cycle;
mod model;

pub use calculation::calculate;
pub use cycle::{Branch, Cycle, Stem};
pub use model::{
    CalendarError, CalendarRequest, CalendarResult, CivilDateTime, DayBoundary, FourPillars,
    LunarDate, SolarTerm,
};
