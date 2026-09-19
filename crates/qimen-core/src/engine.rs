//! The deterministic rotating-plate engine. Indices here originate only from
//! validated calendar results or the fixed tables below.

use qimen_calendar::{Branch, CalendarResult, Cycle, Stem};

use crate::{
    AnnotationBasis, CenterPalaceRule, Chart, ChartRequest, Conventions, Deity, Direction, Door,
    Dun, Element, HeavenStem, Horse, Leaders, Method, Palace, PillarVoids, Star, TianQinRule,
    Trigram, Xun, Yuan,
};

// Clockwise perimeter, starting at north. The intrinsic star and door sequences
// never reverse in yin dun; only the deity traversal reverses.
const RING: [u8; 8] = [1, 8, 3, 4, 9, 2, 7, 6];
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
const STARS: [Star; 9] = [
    Star::TianPeng,
    Star::TianRui,
    Star::TianChong,
    Star::TianFu,
    Star::TianQin,
    Star::TianXin,
    Star::TianZhu,
    Star::TianRen,
    Star::TianYing,
];
const DOORS: [Door; 9] = [
    Door::Xiu,
    Door::Si,
    Door::Shang,
    Door::Du,
    Door::Si,
    Door::Kai,
    Door::Jing,
    Door::Sheng,
    Door::Scene,
];
const DEITIES: [Deity; 8] = [
    Deity::ZhiFu,
    Deity::TengShe,
    Deity::TaiYin,
    Deity::LiuHe,
    Deity::BaiHu,
    Deity::XuanWu,
    Deity::JiuDi,
    Deity::JiuTian,
];

// 冬至 through 大雪, each row 上元/中元/下元. This is the standard
// 时家拆补 table, independent of 置闰 or 茅山 yuan assignment.
const JU: [[u8; 3]; 24] = [
    [1, 7, 4],
    [2, 8, 5],
    [3, 9, 6],
    [8, 5, 2],
    [9, 6, 3],
    [1, 7, 4],
    [3, 9, 6],
    [4, 1, 7],
    [5, 2, 8],
    [4, 1, 7],
    [5, 2, 8],
    [6, 3, 9],
    [9, 3, 6],
    [8, 2, 5],
    [7, 1, 4],
    [2, 5, 8],
    [1, 4, 7],
    [9, 3, 6],
    [7, 1, 4],
    [6, 9, 3],
    [5, 8, 2],
    [6, 9, 3],
    [5, 8, 2],
    [4, 7, 1],
];

pub(crate) fn build(request: &ChartRequest, calendar: CalendarResult) -> Chart {
    let day = calendar.four_pillars.day;
    let hour = calendar.four_pillars.hour;
    let yuan_index = day.index / 5 % 3;
    let yuan = [Yuan::Upper, Yuan::Middle, Yuan::Lower][usize::from(yuan_index)];
    let term_index = calendar.solar_term.index;
    let dun = if term_index < 12 { Dun::Yang } else { Dun::Yin };
    let ju = JU[usize::from(term_index)][usize::from(yuan_index)];
    let xun = xun(hour);
    let horse = horse(hour.branch);
    let (palaces, leaders) = plates(dun, ju, hour, &xun, &horse);
    let pillars = calendar.four_pillars;
    Chart {
        schema_version: crate::SCHEMA_VERSION.to_owned(),
        input: request.clone(),
        calendar,
        method: Method::ShiJiaChaiBuZhuanPan,
        conventions: Conventions {
            center_palace: CenterPalaceRule::AlwaysKun,
            tian_qin: TianQinRule::FollowsTianRui,
            void_basis: AnnotationBasis::Hour,
            horse_basis: AnnotationBasis::Hour,
        },
        dun,
        yuan,
        ju,
        yuan_head: Cycle::from_index(day.index / 5 * 5),
        xun,
        leaders,
        horse,
        pillar_voids: PillarVoids {
            year: void_branches(pillars.year),
            month: void_branches(pillars.month),
            day: void_branches(pillars.day),
            hour: void_branches(pillars.hour),
        },
        palaces,
        extensions: None,
    }
}

fn xun(hour: Cycle) -> Xun {
    let index = hour.index / 10;
    Xun {
        head: Cycle::from_index(index * 10),
        hidden_stem: INSTRUMENTS[usize::from(index)],
        void_branches: void_branches(hour),
    }
}

fn void_branches(pillar: Cycle) -> [Branch; 2] {
    let first = 10 - pillar.index / 10 * 2;
    [Branch::from_index(first), Branch::from_index(first + 1)]
}

pub(crate) fn horse(branch: Branch) -> Horse {
    let branch = match branch.index() % 4 {
        0 => Branch::Yin,
        1 => Branch::Hai,
        2 => Branch::Shen,
        _ => Branch::Si,
    };
    Horse {
        branch,
        palace: branch_palace(branch),
    }
}

fn branch_palace(branch: Branch) -> u8 {
    [1, 8, 8, 3, 4, 4, 9, 2, 2, 7, 6, 6][usize::from(branch.index())]
}

fn host(number: u8) -> u8 {
    if number == 5 { 2 } else { number }
}

fn ring_index(number: u8) -> i16 {
    match number {
        1 => 0,
        8 => 1,
        3 => 2,
        4 => 3,
        9 => 4,
        2 | 5 => 5,
        7 => 6,
        _ => 7, // Remaining valid palace is 乾六.
    }
}

fn rotate(number: u8, shift: i16) -> u8 {
    RING[(ring_index(number) + shift).rem_euclid(8) as usize]
}

fn plates(dun: Dun, ju: u8, hour: Cycle, xun: &Xun, horse: &Horse) -> ([Palace; 9], Leaders) {
    let sign: i16 = if dun == Dun::Yang { 1 } else { -1 };
    let mut earth = [Stem::Wu; 9];
    let mut original = 1_u8;
    let mut target = 1_u8;
    let hour_stem = if hour.stem == Stem::Jia {
        xun.hidden_stem
    } else {
        hour.stem
    };
    for (step, stem) in INSTRUMENTS.into_iter().enumerate() {
        let index = (i16::from(ju) - 1 + sign * step as i16).rem_euclid(9) as usize;
        earth[index] = stem;
        if stem == xun.hidden_stem {
            original = index as u8 + 1;
        }
        if stem == hour_stem {
            target = index as u8 + 1;
        }
    }

    let star_target = host(target);
    // Numeric flight MUST begin at the original palace, including center five.
    // Hosting original=5 at 坤二 before this step incorrectly changes 值使落宫.
    let door_raw =
        (i16::from(original) - 1 + sign * i16::from(hour.index % 10)).rem_euclid(9) as u8 + 1;
    let door_target = host(door_raw);
    let star_shift = ring_index(star_target) - ring_index(host(original));
    let door_shift = ring_index(door_target) - ring_index(host(original));
    let leaders = Leaders {
        star: STARS[usize::from(original - 1)],
        door: DOORS[usize::from(original - 1)],
        original_palace: original,
        star_palace: star_target,
        door_palace: door_target,
        door_raw_palace: door_raw,
    };
    let mut palaces = std::array::from_fn(|index| blank_palace(index as u8 + 1, earth[index]));
    palaces[1].hosted_earth_stem = Some(earth[4]);

    for source in RING {
        let source_index = usize::from(source - 1);
        let star_palace = &mut palaces[usize::from(rotate(source, star_shift) - 1)];
        star_palace.stars.push(STARS[source_index]);
        star_palace.heaven_stems.push(HeavenStem {
            stem: earth[source_index],
            source_palace: source,
            is_center_hosted: false,
        });
        if source == 2 {
            star_palace.stars.push(Star::TianQin);
            star_palace.heaven_stems.push(HeavenStem {
                stem: earth[4],
                source_palace: 5,
                is_center_hosted: true,
            });
        }
        palaces[usize::from(rotate(source, door_shift) - 1)].door = Some(DOORS[source_index]);
    }

    for (step, deity) in DEITIES.into_iter().enumerate() {
        let number = rotate(star_target, sign * step as i16);
        palaces[usize::from(number - 1)].deity = Some(deity);
    }
    for branch in xun.void_branches {
        palaces[usize::from(branch_palace(branch) - 1)]
            .void_branches
            .push(branch);
    }
    palaces[usize::from(horse.palace - 1)].is_horse = true;
    (palaces, leaders)
}

fn blank_palace(number: u8, earth_stem: Stem) -> Palace {
    let index = usize::from(number - 1);
    let direction = [
        Direction::North,
        Direction::Southwest,
        Direction::East,
        Direction::Southeast,
        Direction::Center,
        Direction::Northwest,
        Direction::West,
        Direction::Northeast,
        Direction::South,
    ][index];
    let trigram = [
        Some(Trigram::Kan),
        Some(Trigram::Kun),
        Some(Trigram::Zhen),
        Some(Trigram::Xun),
        None,
        Some(Trigram::Qian),
        Some(Trigram::Dui),
        Some(Trigram::Gen),
        Some(Trigram::Li),
    ][index];
    let element = [
        Element::Water,
        Element::Earth,
        Element::Wood,
        Element::Wood,
        Element::Earth,
        Element::Metal,
        Element::Metal,
        Element::Earth,
        Element::Fire,
    ][index];
    Palace {
        number,
        direction,
        trigram,
        element,
        earth_stem,
        hosted_earth_stem: None,
        heaven_stems: Vec::new(),
        stars: Vec::new(),
        door: None,
        deity: None,
        void_branches: Vec::new(),
        is_horse: false,
    }
}

#[cfg(test)]
#[path = "../tests/unit/engine.rs"]
mod tests;
