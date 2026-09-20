use tyme4rs::tyme::solar::SolarTerm as TymeSolarTerm;
use tyme4rs::tyme::{Culture, Tyme};

use crate::civil::{SECONDS_PER_DAY, from_seconds, julian_seconds, validate};
use crate::{
    CalendarError, CalendarRequest, CalendarResult, Cycle, DayBoundary, FourPillars, SolarTerm,
};

/// Calculates the Four Pillars and the surrounding solar terms.
///
/// Year and month boundaries are compared as absolute instants. Day and hour
/// pillars follow the input's local civil clock, without longitude correction.
/// The interval for a term includes its calculated start instant.
///
/// # Errors
///
/// Returns [`CalendarError`] for dates outside 1–9999, invalid proleptic-Gregorian
/// dates or times, or UTC offsets outside −14:00 through +14:00.
///
/// # Examples
///
/// ```
/// use qimen_calendar::{CalendarRequest, calculate};
/// let result = calculate(&CalendarRequest::new(2024, 2, 10, 12))?;
/// assert_eq!(result.four_pillars.year.name(), "甲辰");
/// assert_eq!(result.lunar_date.day, 1);
/// # Ok::<(), qimen_calendar::CalendarError>(())
/// ```
pub fn calculate(request: &CalendarRequest) -> Result<CalendarResult, CalendarError> {
    let local_seconds = validate(request)?;
    // Upstream term Julian dates carry the provider's UTC+08:00 civil clock.
    // Compare integral seconds so equality has the same meaning as the public
    // second-resolution boundary. Civil conversions remain Gregorian throughout.
    let beijing_seconds = local_seconds + i64::from(480 - request.utc_offset_minutes) * 60;
    let term = current_term(beijing_seconds);
    let local_day = local_seconds.div_euclid(SECONDS_PER_DAY);

    Ok(CalendarResult {
        four_pillars: pillars(request, local_day, &term),
        solar_term: convert_term(&term, request.utc_offset_minutes),
        next_solar_term: convert_term(&term.next(1), request.utc_offset_minutes),
        lunar_date: crate::lunar::calculate(request.year, local_day)?,
    })
}

fn current_term(beijing_seconds: i64) -> TymeSolarTerm {
    let date = from_seconds(beijing_seconds, 480);
    let mut term = TymeSolarTerm::from_index(date.year as isize, (date.month * 2) as isize);
    while beijing_seconds < julian_seconds(term.get_julian_day().get_day()) {
        term = term.next(-1);
    }
    // Far from the modern epoch a term can drift beyond the initial civil-month
    // estimate. Compare both sides rather than assuming that estimate is a bound.
    loop {
        let next = term.next(1);
        if beijing_seconds < julian_seconds(next.get_julian_day().get_day()) {
            return term;
        }
        term = next;
    }
}

fn pillars(request: &CalendarRequest, local_day: i64, term: &TymeSolarTerm) -> FourPillars {
    // The provider indexes winter solstice as 0 of the following term year.
    // Li Chun (index 3) begins the pillar year; each pair of terms begins a month.
    let pillar_year = term.get_year() as i64 - i64::from(term.get_index() < 3);
    let month_index = (term.get_index() as i64 - 3).rem_euclid(24) / 2;
    let year = Cycle::from_index((pillar_year - 4).rem_euclid(60) as u8);
    // Five-tiger rule: Jia/Ji years begin with Bing-Yin.
    let month = Cycle::from_index((pillar_year * 12 - 46 + month_index).rem_euclid(60) as u8);
    // Gregorian 2000-01-07 has Julian day number 2451551 and is Jia-Zi.
    let day_index = (local_day - 2_451_551).rem_euclid(60) as u8;
    let next_day = u8::from(request.hour == 23);
    let day = Cycle::from_index(
        day_index
            + if request.day_boundary == DayBoundary::ZiStart {
                next_day
            } else {
                0
            },
    );
    // Five-rat rule and late-Zi convention: the hour stem uses the next day
    // at 23:00 even when the displayed day changes at midnight.
    let hour_day = Cycle::from_index(day_index + next_day);
    let branch = request.hour.div_ceil(2) as u8 % 12;
    let hour = Cycle::from_index((hour_day.stem.index() % 5) * 12 + branch);
    FourPillars {
        year,
        month,
        day,
        hour,
    }
}

fn convert_term(term: &TymeSolarTerm, offset: i32) -> SolarTerm {
    let seconds = julian_seconds(term.get_julian_day().get_day()) + i64::from(offset - 480) * 60;
    SolarTerm {
        index: term.get_index() as u8,
        name: term.get_name(),
        start: from_seconds(seconds, offset),
    }
}
