//! Month numbering under the modern Chinese-calendar rules, extended in both
//! directions. Astronomical conjunctions and solar terms remain upstream; the
//! provider's historical month-name and leap-year tables are not consulted.

use std::f64::consts::TAU;

use tyme4rs::tyme::solar::SolarTerm;
use tyme4rs::tyme::util::ShouXingUtil;

use crate::{CalendarError, Cycle, LunarDate};

const J2000: f64 = 2_451_545.0;

// These labels describe month/day numbers only, not an astronomical algorithm.
const MONTH_NAMES: [&str; 12] = [
    "正月",
    "二月",
    "三月",
    "四月",
    "五月",
    "六月",
    "七月",
    "八月",
    "九月",
    "十月",
    "十一月",
    "十二月",
];
const DAY_NAMES: [&str; 30] = [
    "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一", "十二",
    "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二", "廿三", "廿四",
    "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
];

struct WinterCycle {
    year: i32,
    starts: Vec<i64>,
    leap_index: Option<usize>,
}

impl WinterCycle {
    fn new(year: i32) -> Result<Self, CalendarError> {
        let terms: Vec<i64> = (0..=12)
            .map(|index| {
                (SolarTerm::from_index(year as isize, index * 2)
                    .get_julian_day()
                    .get_day()
                    + 0.5)
                    .floor() as i64
            })
            .collect();
        let first = moon_at_or_before(terms[0]);
        let last = moon_at_or_before(terms[12]);
        if !(12..=13).contains(&(last - first)) {
            return Err(CalendarError::ConversionFailed(
                "winter solstices do not bracket twelve or thirteen lunar months".to_owned(),
            ));
        }
        let starts: Vec<i64> = (first..=last).map(moon_day).collect();
        if starts
            .windows(2)
            .any(|days| !(29..=30).contains(&(days[1] - days[0])))
        {
            return Err(CalendarError::ConversionFailed(
                "consecutive conjunction dates do not form 29- or 30-day lunar months".to_owned(),
            ));
        }
        let leap_index = if last - first == 13 {
            let index = (1..13).find(|&index| {
                !terms
                    .iter()
                    .any(|&term| starts[index] <= term && term < starts[index + 1])
            });
            Some(index.ok_or_else(|| {
                CalendarError::ConversionFailed(
                    "a thirteen-month winter cycle has no month without a principal term"
                        .to_owned(),
                )
            })?)
        } else {
            None
        };
        Ok(Self {
            year,
            starts,
            leap_index,
        })
    }

    fn date(&self, local_day: i64) -> Option<LunarDate> {
        let index = self
            .starts
            .windows(2)
            .position(|days| days[0] <= local_day && local_day < days[1])?;
        let numbered_index = index - usize::from(self.leap_index.is_some_and(|leap| index >= leap));
        let month = (10 + numbered_index) % 12 + 1;
        let year = self.year - i32::from(numbered_index < 2);
        let day = (local_day - self.starts[index] + 1) as usize;
        let is_leap_month = self.leap_index == Some(index);
        let year_name = Cycle::from_index((year - 4).rem_euclid(60) as u8);
        Some(LunarDate {
            year,
            month: month as u32,
            day: day as u32,
            is_leap_month,
            name: format!(
                "农历{year_name}年{}{}{}",
                if is_leap_month { "闰" } else { "" },
                MONTH_NAMES[month - 1],
                DAY_NAMES[day - 1],
            ),
        })
    }
}

pub(crate) fn calculate(year: i32, local_day: i64) -> Result<LunarDate, CalendarError> {
    let cycle = WinterCycle::new(year)?;
    if let Some(date) = cycle.date(local_day) {
        return Ok(date);
    }
    // A civil December date may already belong to the next winter-solstice
    // month. At the public endpoints only numeric astronomy crosses year bounds.
    let neighboring_year = year + if local_day < cycle.starts[0] { -1 } else { 1 };
    WinterCycle::new(neighboring_year)?
        .date(local_day)
        .ok_or_else(|| {
            CalendarError::ConversionFailed(
                "no winter cycle contains the Gregorian date".to_owned(),
            )
        })
}

fn moon_day(lunation: i64) -> i64 {
    // Use the provider's complete conjunction solver. Its shuo_high shortcut
    // only refines estimates within 30 minutes of midnight, a threshold that
    // is insufficient for the distant-future extrapolation accepted here.
    let terrestrial_days = ShouXingUtil::m_sa_lon_t(lunation as f64 * TAU) * 36525.0;
    let beijing_days = terrestrial_days - ShouXingUtil::dtt(terrestrial_days) + 1.0 / 3.0;
    (beijing_days + J2000 + 0.5).floor() as i64
}

fn moon_at_or_before(day: i64) -> i64 {
    // The mean synodic month is solely a search estimate; actual boundaries
    // always come from the upstream conjunction calculation.
    let mut lunation = ((day as f64 - 2_451_551.0) / 29.5306).floor() as i64;
    while moon_day(lunation) > day {
        lunation -= 1;
    }
    while moon_day(lunation + 1) <= day {
        lunation += 1;
    }
    lunation
}

#[cfg(test)]
#[path = "../tests/unit/lunar.rs"]
mod tests;
