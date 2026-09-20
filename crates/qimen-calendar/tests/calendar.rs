//! Public calendar contract and boundary regressions.

use qimen_calendar::{CalendarError, CalendarRequest, DayBoundary, calculate};

#[test]
fn lunar_new_year_is_not_the_pillar_year_boundary() -> Result<(), CalendarError> {
    let before = calculate(&CalendarRequest::new(2024, 2, 4, 16))?;
    let after = calculate(&CalendarRequest::new(2024, 2, 4, 17))?;
    assert_eq!(before.four_pillars.year.name(), "癸卯");
    assert_eq!(before.four_pillars.month.name(), "乙丑");
    assert_eq!(after.four_pillars.year.name(), "甲辰");
    assert_eq!(after.four_pillars.month.name(), "丙寅");
    assert_eq!(after.lunar_date.year, 2023);
    let new_year = calculate(&CalendarRequest::new(2024, 2, 10, 12))?;
    assert_eq!(
        (
            new_year.lunar_date.year,
            new_year.lunar_date.month,
            new_year.lunar_date.day
        ),
        (2024, 1, 1)
    );
    assert_eq!(new_year.four_pillars.year, after.four_pillars.year);
    Ok(())
}

#[test]
fn late_zi_day_conventions_preserve_a_continuous_hour_pillar() -> Result<(), CalendarError> {
    let mut request = CalendarRequest::new(2000, 1, 7, 22);
    let before = calculate(&request)?;
    assert_eq!(before.four_pillars.day.name(), "甲子");
    assert_eq!(before.four_pillars.hour.name(), "乙亥");
    request.hour = 23;
    let zi_start = calculate(&request)?;
    assert_eq!(zi_start.four_pillars.day.name(), "乙丑");
    assert_eq!(zi_start.four_pillars.hour.name(), "丙子");
    request.day_boundary = DayBoundary::Midnight;
    let midnight = calculate(&request)?;
    assert_eq!(midnight.four_pillars.day.name(), "甲子");
    assert_eq!(midnight.four_pillars.hour, zi_start.four_pillars.hour);
    request.day = 8;
    request.hour = 0;
    let next_midnight = calculate(&request)?;
    assert_eq!(next_midnight.four_pillars.day, zi_start.four_pillars.day);
    assert_eq!(next_midnight.four_pillars.hour, zi_start.four_pillars.hour);
    Ok(())
}

#[test]
fn negative_day_offsets_wrap_across_the_jia_zi_anchor() -> Result<(), CalendarError> {
    let before = calculate(&CalendarRequest::new(2000, 1, 6, 22))?;
    let after = calculate(&CalendarRequest::new(2000, 1, 6, 23))?;
    assert_eq!(before.four_pillars.day.name(), "癸亥");
    assert_eq!(after.four_pillars.day.name(), "甲子");
    assert_eq!(after.four_pillars.hour.name(), "甲子");
    Ok(())
}

#[test]
fn instant_based_pillars_do_not_replace_the_local_civil_clock() -> Result<(), CalendarError> {
    let beijing = calculate(&CalendarRequest::new(2024, 2, 4, 17))?;
    let mut utc_request = CalendarRequest::new(2024, 2, 4, 9);
    utc_request.utc_offset_minutes = 0;
    let utc = calculate(&utc_request)?;
    assert_eq!(utc.four_pillars.year, beijing.four_pillars.year);
    assert_eq!(utc.four_pillars.month, beijing.four_pillars.month);
    assert_eq!(utc.solar_term.index, beijing.solar_term.index);
    assert_ne!(utc.four_pillars.hour, beijing.four_pillars.hour);
    assert_eq!(utc.solar_term.start.hour + 8, beijing.solar_term.start.hour);
    assert_eq!(utc.solar_term.start.utc_offset_minutes, 0);
    Ok(())
}

#[test]
fn lunar_leap_month_is_explicit() -> Result<(), CalendarError> {
    let result = calculate(&CalendarRequest::new(2023, 3, 22, 12))?;
    assert_eq!(
        (
            result.lunar_date.year,
            result.lunar_date.month,
            result.lunar_date.day
        ),
        (2023, 2, 1)
    );
    assert!(result.lunar_date.is_leap_month);
    Ok(())
}

#[test]
fn hko_2026_lunar_calendar_fixtures() -> Result<(), CalendarError> {
    // Independent source: Hong Kong Observatory Gregorian-Lunar Calendar 2026.
    // https://www.hko.gov.hk/en/gts/time/calendar/pdf/files/2026e.pdf
    for (month, day, lunar_month, lunar_day) in [(2, 17, 1, 1), (9, 25, 8, 15)] {
        let result = calculate(&CalendarRequest::new(2026, month, day, 12))?;
        assert_eq!(
            (
                result.lunar_date.year,
                result.lunar_date.month,
                result.lunar_date.day
            ),
            (2026, lunar_month, lunar_day)
        );
        assert!(!result.lunar_date.is_leap_month);
    }
    Ok(())
}

#[test]
fn all_supported_input_edges_allow_internal_timezone_crossovers() -> Result<(), CalendarError> {
    for (year, month, day, hour) in [(1, 1, 1, 0), (9999, 12, 31, 23)] {
        for offset in [-840, 840] {
            let mut request = CalendarRequest::new(year, month, day, hour);
            request.utc_offset_minutes = offset;
            calculate(&request)?;
        }
    }
    Ok(())
}

#[test]
fn invalid_inputs_are_errors() {
    for (year, month, day, hour) in [
        (0, 1, 1, 0),
        (10000, 1, 1, 0),
        (1900, 2, 29, 0),
        (2100, 2, 29, 0),
        (2024, 0, 1, 0),
        (2024, 13, 1, 0),
        (2024, 1, 0, 0),
        (2024, 4, 31, 0),
        (2024, 1, 1, 24),
        (2024, u32::MAX, 1, 0),
        (2024, 1, u32::MAX, 0),
    ] {
        assert!(calculate(&CalendarRequest::new(year, month, day, hour)).is_err());
    }
    let mut request = CalendarRequest::new(2024, 1, 1, 0);
    request.minute = 60;
    assert!(calculate(&request).is_err());
    request.minute = 0;
    request.second = 60;
    assert!(calculate(&request).is_err());
    request.second = 0;
    request.utc_offset_minutes = 841;
    assert_eq!(
        calculate(&request),
        Err(CalendarError::InvalidUtcOffset(841))
    );
    assert!(calculate(&CalendarRequest::new(2000, 2, 29, 0)).is_ok());
}

#[test]
fn json_defaults_and_unknown_field_rejection() -> Result<(), serde_json::Error> {
    let request: CalendarRequest =
        serde_json::from_str(r#"{"year":2024,"month":2,"day":10,"hour":12}"#)?;
    assert_eq!(request, CalendarRequest::new(2024, 2, 10, 12));
    assert!(
        serde_json::from_str::<CalendarRequest>(
            r#"{"year":2024,"month":2,"day":10,"hour":12,"day_boundary":{"zi_start":null}}"#
        )
        .is_err()
    );
    assert!(
        serde_json::from_str::<CalendarRequest>(
            r#"{"year":2024,"month":2,"day":10,"hour":12,"offset":0}"#
        )
        .is_err()
    );
    Ok(())
}
