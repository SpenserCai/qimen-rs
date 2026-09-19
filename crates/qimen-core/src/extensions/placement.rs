use super::{
    DayHorse, DayHorseRule, HiddenStem, HiddenStemRule, HiddenStems, StemPlacement, StemPlate,
};
use crate::{Branch, Chart, Dun, Stem};

const INSTRUMENTS: [Stem; 9] = [
    Stem::Wu,
    Stem::Ji,
    Stem::Geng,
    Stem::Xin,
    Stem::Ren,
    Stem::Gui,
    Stem::Ding,
    Stem::Bing,
    Stem::Yi,
];

pub(super) fn hidden_stems(chart: &Chart, rule: HiddenStemRule) -> HiddenStems {
    let hour_stem = chart.calendar.four_pillars.hour.stem;
    let effective_hour_stem = if hour_stem == Stem::Jia {
        chart.xun.hidden_stem
    } else {
        hour_stem
    };
    let used_center_fallback = chart
        .palace(chart.leaders.door_palace)
        .is_some_and(|palace| palace.earth_stem == effective_hour_stem);
    let start_palace = if used_center_fallback {
        5
    } else {
        chart.leaders.door_palace
    };
    let start_index = match effective_hour_stem {
        Stem::Wu | Stem::Jia => 0,
        Stem::Ji => 1,
        Stem::Geng => 2,
        Stem::Xin => 3,
        Stem::Ren => 4,
        Stem::Gui => 5,
        Stem::Ding => 6,
        Stem::Bing => 7,
        Stem::Yi => 8,
    };
    let sign = if chart.dun == Dun::Yang {
        1_i16
    } else {
        -1_i16
    };
    let palaces = std::array::from_fn(|index| {
        let steps = (sign * (index as i16 + 1 - i16::from(start_palace))).rem_euclid(9);
        HiddenStem {
            palace: index as u8 + 1,
            stem: INSTRUMENTS[(start_index + steps as usize) % 9],
        }
    });
    HiddenStems {
        rule,
        effective_hour_stem,
        start_palace,
        used_center_fallback,
        palaces,
    }
}

pub(super) fn stem_placements(chart: &Chart, hidden: Option<&HiddenStems>) -> Vec<StemPlacement> {
    let mut result = Vec::with_capacity(if hidden.is_some() { 28 } else { 19 });
    for palace in &chart.palaces {
        result.push(StemPlacement {
            palace: palace.number,
            plate: StemPlate::Earth,
            stem: palace.earth_stem,
            source_palace: Some(palace.number),
            is_center_hosted: false,
        });
        if let Some(stem) = palace.hosted_earth_stem {
            result.push(StemPlacement {
                palace: palace.number,
                plate: StemPlate::Earth,
                stem,
                source_palace: Some(5),
                is_center_hosted: true,
            });
        }
        result.extend(palace.heaven_stems.iter().map(|stem| StemPlacement {
            palace: palace.number,
            plate: StemPlate::Heaven,
            stem: stem.stem,
            source_palace: Some(stem.source_palace),
            is_center_hosted: stem.is_center_hosted,
        }));
        if let Some(hidden) = hidden {
            result.push(StemPlacement {
                palace: palace.number,
                plate: StemPlate::Hidden,
                stem: hidden.palaces[usize::from(palace.number - 1)].stem,
                source_palace: None,
                is_center_hosted: false,
            });
        }
    }
    result
}

/// The actual branches of a Later Heaven palace; center five has none.
pub(super) fn palace_branches(palace: u8) -> &'static [Branch] {
    match palace {
        1 => &[Branch::Zi],
        2 => &[Branch::Wei, Branch::Shen],
        3 => &[Branch::Mao],
        4 => &[Branch::Chen, Branch::Si],
        6 => &[Branch::Xu, Branch::Hai],
        7 => &[Branch::You],
        8 => &[Branch::Chou, Branch::Yin],
        9 => &[Branch::Wu],
        _ => &[],
    }
}

pub(super) fn day_horse(chart: &Chart, rule: DayHorseRule) -> DayHorse {
    let pillar = chart.calendar.four_pillars.day;
    DayHorse {
        rule,
        pillar,
        horse: crate::engine::horse(pillar.branch),
    }
}
