//! Calendar-to-chart boundary regressions through the public API.

use qimen_core::{ChartRequest, DayBoundary, Dun, Error, Yuan, calculate};

#[test]
fn selected_day_boundary_controls_chai_bu_yuan_and_formation() -> Result<(), Error> {
    // 2024-12-25 is 癸亥; the next day is 甲子. Winter-solstice 下元 uses
    // 阳四局, whereas the new 甲子 上元 uses 阳一局. This boundary distinguishes
    // the selected day-pillar convention from an unconditional midnight date.
    let mut request = ChartRequest::new(2024, 12, 25, 22);
    let before = calculate(&request)?;
    assert_eq!(before.calendar.four_pillars.day.name(), "癸亥");
    assert_eq!((before.yuan, before.ju), (Yuan::Lower, 4));

    request.hour = 23;
    let zi_start = calculate(&request)?;
    assert_eq!(zi_start.calendar.four_pillars.day.name(), "甲子");
    assert_eq!((zi_start.yuan, zi_start.ju), (Yuan::Upper, 1));

    request.day_boundary = DayBoundary::Midnight;
    let midnight = calculate(&request)?;
    assert_eq!(midnight.calendar.four_pillars.day.name(), "癸亥");
    assert_eq!((midnight.yuan, midnight.ju), (Yuan::Lower, 4));
    assert_eq!(
        midnight.calendar.four_pillars.hour,
        zi_start.calendar.four_pillars.hour
    );
    Ok(())
}

#[test]
fn solstice_switches_dun_at_the_term_instant_within_one_hour() -> Result<(), Error> {
    // Pinned provider regression: tyme4rs 1.5.0 gives the 2024 summer-solstice
    // boundary as June 21 04:51:00 UTC+08:00. This checks calendar/core wiring,
    // not an independent one-second astronomical accuracy claim.
    let mut request = ChartRequest::new(2024, 6, 21, 4);
    request.minute = 50;
    request.second = 59;
    let before = calculate(&request)?;
    request.minute = 51;
    request.second = 0;
    let at = calculate(&request)?;

    assert_eq!(before.calendar.solar_term.name, "芒种");
    assert_eq!(at.calendar.solar_term.name, "夏至");
    assert_eq!(
        (before.dun, before.yuan, before.ju),
        (Dun::Yang, Yuan::Middle, 3)
    );
    assert_eq!((at.dun, at.yuan, at.ju), (Dun::Yin, Yuan::Middle, 3));
    assert_eq!(before.calendar.four_pillars, at.calendar.four_pillars);
    Ok(())
}

#[test]
fn full_range_endpoints_form_complete_charts_without_needing_a_previous_civil_day()
-> Result<(), Error> {
    // 拆补's 符头 uses the already calculated day-cycle index, so the first
    // four civil dates must not be rejected merely because their 符头 is earlier.
    for (year, month, day) in [(1, 1, 1), (1, 1, 2), (1, 1, 3), (1, 1, 4), (9999, 12, 31)] {
        for offset in [-840, 840] {
            for boundary in [DayBoundary::ZiStart, DayBoundary::Midnight] {
                let mut request = ChartRequest::new(year, month, day, 23);
                request.utc_offset_minutes = offset;
                request.day_boundary = boundary;
                let result = calculate(&request)?;
                assert!((1..=9).contains(&result.ju));
                assert_eq!(result.palaces.len(), 9);
                assert_eq!(
                    result.yuan_head.index,
                    result.calendar.four_pillars.day.index / 5 * 5
                );
            }
        }
    }
    Ok(())
}
