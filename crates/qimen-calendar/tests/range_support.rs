//! Proleptic-Gregorian and full-range provider regressions.

use qimen_calendar::{
    CalendarError, CalendarRequest, CivilDateTime, Cycle, DayBoundary, calculate,
};
use tyme4rs::tyme::solar::SolarTime;

fn timestamp(time: CivilDateTime) -> (i32, u32, u32, u32, u32, u32) {
    (
        time.year,
        time.month,
        time.day,
        time.hour,
        time.minute,
        time.second,
    )
}

#[test]
fn gregorian_julian_day_anchors_determine_day_pillars() -> Result<(), CalendarError> {
    // USNO's Fliegel/van Flandern Gregorian/JD algorithm is valid before 1582:
    // https://aa.usno.navy.mil/faq/JD_formula
    // These fixed integral noon JDs were also checked with Python datetime's
    // independent Gregorian ordinal (ordinal + 1721425). 2000-01-07 is Jia-Zi.
    for (year, month, day, jdn) in [
        (1, 1, 1, 1_721_426_i64),
        (100, 3, 1, 1_757_644),
        (400, 2, 29, 1_867_216),
        (1500, 3, 1, 2_268_983),
        (1582, 10, 4, 2_299_150),
        (1582, 10, 5, 2_299_151),
        (1582, 10, 14, 2_299_160),
        (1582, 10, 15, 2_299_161),
        (1600, 2, 29, 2_305_507),
        (1877, 8, 11, 2_406_843),
        (1970, 1, 1, 2_440_588),
        (1978, 1, 1, 2_443_510),
        (2400, 2, 29, 2_597_701),
        (9999, 12, 31, 5_373_484),
    ] {
        let result = calculate(&CalendarRequest::new(year, month, day, 12))?;
        assert_eq!(
            result.four_pillars.day,
            Cycle::from_index((jdn - 2_451_551).rem_euclid(60) as u8),
            "{year}-{month:02}-{day:02}"
        );
    }
    Ok(())
}

#[test]
fn october_1582_has_all_thirty_one_gregorian_dates() -> Result<(), CalendarError> {
    for day in 1..31 {
        let before = calculate(&CalendarRequest::new(1582, 10, day, 12))?;
        let after = calculate(&CalendarRequest::new(1582, 10, day + 1, 12))?;
        assert_eq!(
            after.four_pillars.day,
            Cycle::from_index(before.four_pillars.day.index + 1)
        );
        if before.lunar_date.month == after.lunar_date.month {
            assert_eq!(after.lunar_date.day, before.lunar_date.day + 1);
        } else {
            assert_eq!(after.lunar_date.day, 1);
        }
    }
    // Same instant: proleptic Gregorian 1582-10-14 is Julian 1582-10-04.
    // tyme4rs's published lunar regression assigns that day lunar 9/18.
    let result = calculate(&CalendarRequest::new(1582, 10, 14, 12))?;
    assert_eq!((result.lunar_date.month, result.lunar_date.day), (9, 18));
    Ok(())
}

#[test]
fn gregorian_century_leap_rules_apply_before_and_after_1582() {
    for year in [100, 1500, 1700, 1900, 2100, 9900] {
        assert!(matches!(
            calculate(&CalendarRequest::new(year, 2, 29, 12)),
            Err(CalendarError::InvalidDateTime(_))
        ));
    }
    for year in [4, 400, 1200, 1600, 2000, 2400, 9600] {
        assert!(calculate(&CalendarRequest::new(year, 2, 29, 12)).is_ok());
    }
}

#[test]
fn endpoints_keep_lunar_dates_and_adjacent_terms_outside_the_input_year_range()
-> Result<(), CalendarError> {
    let first = calculate(&CalendarRequest::new(1, 1, 1, 0))?;
    assert_eq!(first.solar_term.start.year, 0);
    // Neighboring lunar years are valid output even though year 0 is not an
    // accepted input. Ancient month labels follow the modern astronomical
    // convention, not the provider's historical mean-conjunction tables.
    assert_eq!(first.lunar_date.year, 0);
    let last = calculate(&CalendarRequest::new(9999, 12, 31, 23))?;
    assert_eq!(last.next_solar_term.start.year, 10000);
    assert_eq!(
        (
            last.lunar_date.year,
            last.lunar_date.month,
            last.lunar_date.day
        ),
        (9999, 12, 2)
    );
    Ok(())
}

#[test]
fn endpoint_timezones_and_late_zi_preserve_local_day_semantics() -> Result<(), CalendarError> {
    for (year, month, day) in [(1, 1, 1), (9999, 12, 31)] {
        for offset in [-840, 0, 480, 840] {
            let mut request = CalendarRequest::new(year, month, day, 22);
            request.utc_offset_minutes = offset;
            let before = calculate(&request)?;
            request.hour = 23;
            request.minute = 59;
            request.second = 59;
            let zi = calculate(&request)?;
            request.day_boundary = DayBoundary::Midnight;
            let midnight = calculate(&request)?;
            assert_eq!(midnight.four_pillars.day, before.four_pillars.day);
            assert_eq!(
                zi.four_pillars.day,
                Cycle::from_index(before.four_pillars.day.index + 1)
            );
            assert_eq!(zi.four_pillars.hour, midnight.four_pillars.hour);
            assert_eq!(zi.lunar_date, before.lunar_date);
        }
    }
    Ok(())
}

#[test]
fn distant_years_and_historical_lunar_transitions_produce_bounded_results()
-> Result<(), CalendarError> {
    // Include the provider's historical lunar-year exceptions, leap centuries,
    // Gregorian reform, both endpoints, and each millennium of extrapolation.
    for year in [
        1, 4, 8, 9, 23, 24, 100, 239, 240, 241, 400, 1000, 1500, 1582, 1600, 2000, 2101, 3000,
        4000, 5000, 6000, 7000, 8000, 9000, 9999,
    ] {
        for month in 1..=12 {
            for day in [1, 28] {
                for offset in [-840, 840] {
                    let mut request = CalendarRequest::new(year, month, day, 23);
                    request.utc_offset_minutes = offset;
                    let result = calculate(&request)?;
                    assert!((1..=30).contains(&result.lunar_date.day));
                    let local = (year, month, day, 23, 0, 0);
                    assert!(timestamp(result.solar_term.start) <= local);
                    assert!(local < timestamp(result.next_solar_term.start));
                    assert_eq!(
                        result.next_solar_term.index,
                        (result.solar_term.index + 1) % 24
                    );
                }
            }
        }
    }
    Ok(())
}

#[test]
fn modern_pillars_and_lunar_dates_preserve_the_pinned_provider_contract()
-> Result<(), CalendarError> {
    // This is a compatibility test, not an independent astronomy reference.
    for year in [1900, 1924, 1984, 2000, 2024, 2026, 2033, 2100] {
        for month in 1..=12 {
            for hour in [0, 12, 23] {
                let result = calculate(&CalendarRequest::new(year, month, 7, hour))?;
                let provider =
                    SolarTime::from_ymd_hms(year as isize, month as usize, 7, hour as usize, 0, 0);
                let pillars = provider.get_sixty_cycle_hour();
                assert_eq!(
                    result.four_pillars.year.index,
                    pillars.get_year().get_index() as u8
                );
                assert_eq!(
                    result.four_pillars.month.index,
                    pillars.get_month().get_index() as u8
                );
                assert_eq!(
                    result.four_pillars.day.index,
                    pillars.get_day().get_index() as u8
                );
                assert_eq!(
                    result.four_pillars.hour.index,
                    pillars.get_sixty_cycle().get_index() as u8
                );
                assert_eq!(
                    result.lunar_date.name,
                    provider.get_solar_day().get_lunar_day().to_string()
                );
            }
        }
    }
    Ok(())
}

#[test]
fn hko_2033_leap_eleven_requires_a_thirteen_month_winter_cycle() -> Result<(), CalendarError> {
    // Independent published conversion tables:
    // https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2033e.txt
    // https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2034e.txt
    // August's month lacks a principal term but is not intercalary: its winter
    // cycle has twelve months. December starts the actual leap eleventh month.
    for (year, month, day, lunar_year, lunar_month, leap) in [
        (2033, 8, 25, 2033, 8, false),
        (2033, 11, 22, 2033, 11, false),
        (2033, 12, 22, 2033, 11, true),
        (2034, 1, 20, 2033, 12, false),
        (2034, 2, 19, 2034, 1, false),
    ] {
        let result = calculate(&CalendarRequest::new(year, month, day, 12))?;
        assert_eq!(
            (
                result.lunar_date.year,
                result.lunar_date.month,
                result.lunar_date.day,
                result.lunar_date.is_leap_month
            ),
            (lunar_year, lunar_month, 1, leap)
        );
    }
    Ok(())
}

#[test]
fn hko_early_twentieth_century_new_moons_use_astronomical_civil_dates() -> Result<(), CalendarError>
{
    // Independent HKO conversion tables distinguish the complete conjunction
    // calculation from the provider's older mean/low-precision day tables:
    // https://www.hko.gov.hk/en/gts/time/calendar/pdf/files/1906e.pdf
    // https://www.hko.gov.hk/en/gts/time/calendar/pdf/files/1909e.pdf
    // https://www.hko.gov.hk/en/gts/time/calendar/pdf/files/1911e.pdf
    for (year, month, day, lunar_year, lunar_month, lunar_day) in [
        (1906, 4, 23, 1906, 3, 30),
        (1906, 4, 24, 1906, 4, 1),
        (1909, 9, 14, 1909, 8, 1),
        (1911, 1, 1, 1910, 12, 1),
    ] {
        let result = calculate(&CalendarRequest::new(year, month, day, 12))?;
        assert_eq!(
            (
                result.lunar_date.year,
                result.lunar_date.month,
                result.lunar_date.day
            ),
            (lunar_year, lunar_month, lunar_day)
        );
        assert!(!result.lunar_date.is_leap_month);
    }
    Ok(())
}

#[test]
fn nasa_new_moons_use_a_uniform_eight_hour_offset_before_1960() -> Result<(), CalendarError> {
    // Independent NASA/GSFC phases in UT, converted by adding eight hours:
    // https://eclipse.gsfc.nasa.gov/phase/phases1901.html
    // Unlike the HKO historical date labels for 1914 and the 1916 New Year,
    // this library consistently extends the modern UT+08 convention backward.
    // NASA establishes the conjunction's Beijing civil date, hence day 1;
    // it does not provide Chinese month numbers or historical almanac labels.
    for (year, month, day) in [
        (1906, 4, 24),  // 1906-04-23 16:07 UT
        (1909, 9, 14),  // 1909-09-14 15:08 UT
        (1911, 1, 1),   // 1910-12-31 16:21 UT
        (1914, 11, 18), // 1914-11-17 16:02 UT
        (1916, 2, 4),   // 1916-02-03 16:05 UT
        (1916, 4, 3),   // 1916-04-02 16:21 UT
        (1927, 10, 25), // 1927-10-25 15:37 UT
        (1928, 10, 13), // 1928-10-13 15:56 UT
        (1942, 9, 10),  // 1942-09-10 15:53 UT
        (1943, 11, 27), // 1943-11-27 15:23 UT
        (1954, 2, 3),   // 1954-02-03 15:55 UT
        (1957, 3, 2),   // 1957-03-01 16:12 UT
    ] {
        // The entire civil date starts the month, including hours preceding
        // the conjunction itself. Lunar dates do not change at the moon instant.
        let result = calculate(&CalendarRequest::new(year, month, day, 0))?;
        assert_eq!(result.lunar_date.day, 1, "{year}-{month:02}-{day:02}");
    }
    Ok(())
}

#[test]
fn ancient_months_do_not_inherit_historical_year_table_gaps() -> Result<(), CalendarError> {
    // Changing the request's Gregorian year cannot relabel an in-progress
    // lunar month, including the provider's historical reform transitions.
    for year in [8, 23, 24, 239, 659, 660, 754, 755, 773, 774, 1069, 1070] {
        let before = calculate(&CalendarRequest::new(year, 12, 31, 12))?.lunar_date;
        let after = calculate(&CalendarRequest::new(year + 1, 1, 1, 12))?.lunar_date;
        if (before.year, before.month, before.is_leap_month)
            == (after.year, after.month, after.is_leap_month)
        {
            assert_eq!(after.day, before.day + 1);
        } else {
            assert!([29, 30].contains(&before.day));
            assert_eq!(after.day, 1);
        }
    }
    // The old historical month table had no month for this entire interval.
    for (month, day) in [(1, 28), (1, 31), (2, 1), (2, 26)] {
        let result = calculate(&CalendarRequest::new(24, month, day, 12))?;
        assert!((1..=30).contains(&result.lunar_date.day));
    }
    Ok(())
}
