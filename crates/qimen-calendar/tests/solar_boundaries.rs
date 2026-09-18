//! Exact-boundary regressions against the public solar-term timestamps.

use qimen_calendar::{CalendarRequest, CivilDateTime, calculate};
use tyme4rs::tyme::Tyme;
use tyme4rs::tyme::solar::SolarTime;

fn request_at(time: CivilDateTime, delta_seconds: isize) -> CalendarRequest {
    let shifted = SolarTime::from_ymd_hms(
        time.year as isize,
        time.month as usize,
        time.day as usize,
        time.hour as usize,
        time.minute as usize,
        time.second as usize,
    )
    .next(delta_seconds);
    CalendarRequest {
        year: shifted.get_year() as i32,
        month: shifted.get_month() as u32,
        day: shifted.get_day() as u32,
        hour: shifted.get_hour() as u32,
        minute: shifted.get_minute() as u32,
        second: shifted.get_second() as u32,
        utc_offset_minutes: time.utc_offset_minutes,
        day_boundary: qimen_calendar::DayBoundary::ZiStart,
    }
}

#[test]
fn all_twenty_four_terms_use_the_same_inclusive_second_boundary()
-> Result<(), Box<dyn std::error::Error>> {
    let mut start = calculate(&CalendarRequest::new(2024, 1, 1, 12))?
        .next_solar_term
        .start;
    for _ in 0..24 {
        let before = calculate(&request_at(start, -1))?;
        let at = calculate(&request_at(start, 0))?;
        let after = calculate(&request_at(start, 1))?;
        assert_eq!(at.solar_term.index, (before.solar_term.index + 1) % 24);
        assert_eq!(at.solar_term, after.solar_term);
        assert_eq!(at.solar_term.start, start);
        if at.solar_term.index % 2 == 1 {
            assert_ne!(before.four_pillars.month, at.four_pillars.month);
        } else {
            assert_eq!(before.four_pillars.month, at.four_pillars.month);
        }
        if at.solar_term.index == 3 {
            assert_ne!(before.four_pillars.year, at.four_pillars.year);
        } else {
            assert_eq!(before.four_pillars.year, at.four_pillars.year);
        }
        start = at.next_solar_term.start;
    }
    Ok(())
}

#[test]
fn solar_term_is_localized_across_the_previous_civil_date() -> Result<(), Box<dyn std::error::Error>>
{
    let mut request = CalendarRequest::new(2024, 2, 4, 12);
    request.utc_offset_minutes = -720;
    let result = calculate(&request)?;
    assert_eq!(result.solar_term.name, "立春");
    assert_eq!(result.solar_term.start.day, 3);
    assert_eq!(result.solar_term.start.utc_offset_minutes, -720);
    let before = calculate(&request_at(result.solar_term.start, -1))?;
    let at = calculate(&request_at(result.solar_term.start, 0))?;
    assert_eq!(before.four_pillars.year.name(), "癸卯");
    assert_eq!(at.four_pillars.year.name(), "甲辰");
    Ok(())
}
