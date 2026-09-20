use super::{SECONDS_PER_DAY, day_number, from_seconds, julian_seconds};

#[test]
fn usno_julian_date_examples_have_the_published_gregorian_clock() {
    // Independent worked examples: https://aa.usno.navy.mil/faq/JD_formula
    for (jd, year, month, day, hour, minute) in [
        (2_406_842.812_5, 1877, 8, 11, 7, 30),
        (2_443_509.5, 1978, 1, 1, 0, 0),
        (2_443_711.125, 1978, 7, 21, 15, 0),
    ] {
        let result = from_seconds(julian_seconds(jd), 0);
        assert_eq!(
            (
                result.year,
                result.month,
                result.day,
                result.hour,
                result.minute,
                result.second
            ),
            (year, month, day, hour, minute, 0)
        );
    }
}

#[test]
fn every_civil_date_including_internal_neighbor_years_is_contiguous() {
    // Gregorian 0001-01-01 is JD 1721425.5 (ordinal 1); year 0 is a leap
    // year, so its January 1 has integral noon JD 1721060. A running counter
    // independently checks the forward conversion and its inverse at all dates.
    let mut expected_day_number = 1_721_060;
    for year in 0..=10000 {
        let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        for month in 1..=12 {
            let days = match month {
                2 if leap => 29,
                2 => 28,
                4 | 6 | 9 | 11 => 30,
                _ => 31,
            };
            for day in 1..=days {
                assert_eq!(day_number(year, month, day), expected_day_number);
                let midnight = from_seconds(expected_day_number * SECONDS_PER_DAY, 480);
                let last_second =
                    from_seconds((expected_day_number + 1) * SECONDS_PER_DAY - 1, -840);
                assert_eq!(
                    (midnight.year, midnight.month, midnight.day),
                    (year, month, day)
                );
                assert_eq!((midnight.hour, midnight.minute, midnight.second), (0, 0, 0));
                assert_eq!(
                    (last_second.year, last_second.month, last_second.day),
                    (year, month, day)
                );
                assert_eq!(
                    (last_second.hour, last_second.minute, last_second.second),
                    (23, 59, 59)
                );
                assert_eq!(last_second.utc_offset_minutes, -840);
                expected_day_number += 1;
            }
        }
    }
}
