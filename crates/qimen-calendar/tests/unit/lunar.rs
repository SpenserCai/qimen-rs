use super::{WinterCycle, moon_day};
use crate::CalendarError;

#[test]
fn every_supported_winter_cycle_has_contiguous_valid_months() -> Result<(), CalendarError> {
    let mut previous_end = None;
    for year in 1..=10000 {
        let cycle = WinterCycle::new(year)?;
        if let Some(end) = previous_end {
            assert_eq!(cycle.starts[0], end, "winter cycle {year}");
        }
        for days in cycle.starts.windows(2) {
            let first = cycle.date(days[0]).expect("cycle contains its first day");
            let last = cycle
                .date(days[1] - 1)
                .expect("cycle contains its last day");
            assert_eq!(first.day, 1);
            assert_eq!(last.day, (days[1] - days[0]) as u32);
            assert_eq!(
                (first.year, first.month, first.is_leap_month),
                (last.year, last.month, last.is_leap_month)
            );
        }
        previous_end = cycle.starts.last().copied();
    }
    Ok(())
}

#[test]
fn far_future_conjunction_uses_the_complete_solver_near_midnight() {
    // Pinned astronomy regression: the complete tyme4rs solver gives Beijing
    // JD 4657809.497463342 for lunation 74711. Its shuo_high estimate is
    // 4657809.521318832 and wrongly falls on the next civil date. This tests
    // solver selection, not an independent far-future astronomical forecast.
    assert_eq!(moon_day(74711), 4_657_809);
}
