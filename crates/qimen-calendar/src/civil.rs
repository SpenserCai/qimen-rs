//! Integer proleptic-Gregorian dates, independent of the astronomy provider's
//! Julian/Gregorian cutover and its bounded civil-date constructors.
//!
//! Integer Gregorian/Julian-day conversion follows the standard March-based
//! Gregorian cycle decomposition; see <https://aa.usno.navy.mil/faq/JD_formula>.

use crate::{CalendarError, CalendarRequest, CivilDateTime};

pub(crate) const SECONDS_PER_DAY: i64 = 86_400;

/// The integral Julian day number of a Gregorian civil date (midnight to
/// midnight here, unlike the astronomical Julian date's noon origin).
pub(crate) fn day_number(year: i32, month: u32, day: u32) -> i64 {
    let a = (14 - i64::from(month)) / 12;
    let y = i64::from(year) + 4800 - a;
    let m = i64::from(month) + 12 * a - 3;
    i64::from(day) + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

pub(crate) fn validate(request: &CalendarRequest) -> Result<i64, CalendarError> {
    if !(1..=9999).contains(&request.year) {
        return Err(CalendarError::UnsupportedYear(request.year));
    }
    if !(-840..=840).contains(&request.utc_offset_minutes) {
        return Err(CalendarError::InvalidUtcOffset(request.utc_offset_minutes));
    }
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
    Ok(
        day_number(request.year, request.month, request.day) * SECONDS_PER_DAY
            + i64::from(request.hour * 3600 + request.minute * 60 + request.second),
    )
}

/// Converts a provider Julian date directly to its rounded civil second. No
/// provider civil-date constructor is used, including near years 0 and 10000.
pub(crate) fn julian_seconds(julian_day: f64) -> i64 {
    ((julian_day + 0.5) * SECONDS_PER_DAY as f64).round() as i64
}

pub(crate) fn from_seconds(seconds: i64, offset: i32) -> CivilDateTime {
    let jdn = seconds.div_euclid(SECONDS_PER_DAY);
    let time = seconds.rem_euclid(SECONDS_PER_DAY) as u32;
    let a = jdn + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - 146097 * b / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - 1461 * d / 4;
    let m = (5 * e + 2) / 153;
    CivilDateTime {
        year: (100 * b + d - 4800 + m / 10) as i32,
        month: (m + 3 - 12 * (m / 10)) as u32,
        day: (e - (153 * m + 2) / 5 + 1) as u32,
        hour: time / 3600,
        minute: time % 3600 / 60,
        second: time % 60,
        utc_offset_minutes: offset,
    }
}

#[cfg(test)]
#[path = "../tests/unit/civil.rs"]
mod tests;
