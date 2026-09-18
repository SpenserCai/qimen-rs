use tyme4rs::tyme::solar::{SolarDay, SolarTerm as TymeSolarTerm, SolarTime};
use tyme4rs::tyme::{Culture, Tyme};

use crate::{
    CalendarError, CalendarRequest, CalendarResult, CivilDateTime, Cycle, DayBoundary, FourPillars,
    LunarDate, SolarTerm,
};

/// Calculates the Four Pillars and the surrounding solar terms.
///
/// Year and month boundaries are compared as absolute instants. Day and hour
/// pillars follow the input's local civil clock, without longitude correction.
/// The interval for a term includes its calculated start instant.
///
/// # Errors
///
/// Returns [`CalendarError`] for dates outside 1900–2100, invalid Gregorian
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
    let local = validate(request)?;
    // The upstream astronomy API expresses term instants as UTC+08:00 civil time.
    // Normalize only the instant used for term/year/month comparisons, leaving
    // the original local day/hour untouched.
    let beijing = local.next(((480 - request.utc_offset_minutes) * 60) as isize);
    let term = beijing.get_term();
    let pillars = pillars(local, beijing, request.day_boundary);
    let lunar = local.get_solar_day().get_lunar_day();
    let lunar_month = lunar.get_lunar_month();

    Ok(CalendarResult {
        four_pillars: pillars,
        solar_term: convert_term(&term, request.utc_offset_minutes),
        next_solar_term: convert_term(&term.next(1), request.utc_offset_minutes),
        lunar_date: LunarDate {
            year: lunar.get_year() as i32,
            month: lunar_month.get_month() as u32,
            day: lunar.get_day() as u32,
            is_leap_month: lunar_month.is_leap(),
            name: lunar.to_string(),
        },
    })
}

fn validate(request: &CalendarRequest) -> Result<SolarTime, CalendarError> {
    if !(1900..=2100).contains(&request.year) {
        return Err(CalendarError::UnsupportedYear(request.year));
    }
    if !(-840..=840).contains(&request.utc_offset_minutes) {
        return Err(CalendarError::InvalidUtcOffset(request.utc_offset_minutes));
    }
    // Validate before entering the dependency: some upstream `new` paths call
    // infallible constructors internally before validating the month.
    for (field, value, minimum, maximum) in [
        ("month", request.month, 1, 12),
        ("hour", request.hour, 0, 23),
        ("minute", request.minute, 0, 59),
        ("second", request.second, 0, 59),
    ] {
        if !(minimum..=maximum).contains(&value) {
            return Err(CalendarError::InvalidDateTime(format!(
                "{field} must be between {minimum} and {maximum}, got {value}"
            )));
        }
    }
    let leap = request.year % 4 == 0 && (request.year % 100 != 0 || request.year % 400 == 0);
    let mut days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][(request.month - 1) as usize];
    if request.month == 2 && leap {
        days += 1;
    }
    if !(1..=days).contains(&request.day) {
        return Err(CalendarError::InvalidDateTime(format!(
            "day must be between 1 and {days} in {}-{:02}, got {}",
            request.year, request.month, request.day
        )));
    }
    SolarTime::new(
        request.year as isize,
        request.month as usize,
        request.day as usize,
        request.hour as usize,
        request.minute as usize,
        request.second as usize,
    )
    .map_err(CalendarError::InvalidDateTime)
}

fn pillars(local: SolarTime, beijing: SolarTime, day_boundary: DayBoundary) -> FourPillars {
    // Year/month are delegated together so all term boundaries share the exact
    // same astronomical provider. Local day/hour do not inherit Beijing's clock.
    let astronomical = beijing.get_sixty_cycle_hour();
    // 2000-01-07 is the Jia-Zi anchor also used by tyme4rs::SixtyCycleDay.
    let day_offset = local
        .get_solar_day()
        .subtract(SolarDay::from_ymd(2000, 1, 7));
    let local_day = day_offset.rem_euclid(60) as u8;
    let next_day = u8::from(local.get_hour() == 23);
    let day = Cycle::from_index(
        local_day
            + if day_boundary == DayBoundary::ZiStart {
                next_day
            } else {
                0
            },
    );
    // Five-rat rule: Jia/Ji days begin with Jia-Zi, Yi/Geng with Bing-Zi, etc.
    // Midnight follows the common late-Zi convention: keep the displayed day
    // pillar but use the following day's Zi-hour stem across the complete hour.
    let hour_day = Cycle::from_index(local_day + next_day);
    let branch = local.get_hour().div_ceil(2) as u8 % 12;
    let hour = Cycle::from_index((hour_day.stem.index() % 5) * 12 + branch);

    FourPillars {
        year: Cycle::from_index(astronomical.get_year().get_index() as u8),
        month: Cycle::from_index(astronomical.get_month().get_index() as u8),
        day,
        hour,
    }
}

fn convert_term(term: &TymeSolarTerm, offset: i32) -> SolarTerm {
    let local = term
        .get_julian_day()
        .get_solar_time()
        .next(((offset - 480) * 60) as isize);
    SolarTerm {
        index: term.get_index() as u8,
        name: term.get_name(),
        start: CivilDateTime {
            year: local.get_year() as i32,
            month: local.get_month() as u32,
            day: local.get_day() as u32,
            hour: local.get_hour() as u32,
            minute: local.get_minute() as u32,
            second: local.get_second() as u32,
            utc_offset_minutes: offset,
        },
    }
}
