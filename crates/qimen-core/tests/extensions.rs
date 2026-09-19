//! Independent extension fixtures: the user's 2026-09-18 18:15 screenshot,
//! classical tables, and explicit numeric-flight examples in docs/extensions.md.

use qimen_core::{
    Branch, Calculator, Chart, ChartRequest, DayBoundary, Element, ElementStrengthState,
    ExtensionOptions, GrowthStage, StarStrengthState, Stem, StemPlate, TombRule,
    calculate_with_options,
};

fn reference_chart() -> Chart {
    let mut request = ChartRequest::new(2026, 9, 18, 18);
    request.minute = 15;
    Calculator::new(ExtensionOptions::all())
        .calculate(&request)
        .unwrap()
}

#[test]
fn screenshot_hidden_stems_and_day_horse_match_the_independent_display() {
    let chart = reference_chart();
    let annotations = chart.extensions.as_ref().unwrap();
    let hidden = annotations.hidden_stems.as_ref().unwrap();
    assert_eq!(hidden.effective_hour_stem, Stem::Yi);
    assert_eq!(hidden.start_palace, 6);
    assert!(!hidden.used_center_fallback);
    // Transcribed upper-left gray stems, reordered from south-up to Luo Shu 1–9.
    assert_eq!(
        hidden.palaces.each_ref().map(|item| item.stem),
        [
            Stem::Ren,
            Stem::Xin,
            Stem::Geng,
            Stem::Ji,
            Stem::Wu,
            Stem::Yi,
            Stem::Bing,
            Stem::Ding,
            Stem::Gui,
        ]
    );
    let day = annotations.day_horse.as_ref().unwrap();
    assert_eq!(day.pillar.to_string(), "乙未");
    assert_eq!((day.horse.branch, day.horse.palace), (Branch::Si, 4));
    assert_eq!((chart.horse.branch, chart.horse.palace), (Branch::Hai, 6));
    assert!(chart.palace(6).unwrap().is_horse);
    assert!(!chart.palace(4).unwrap().is_horse);
}

#[test]
fn hidden_stem_numeric_flight_handles_jia_and_does_not_confuse_hosted_leaders() {
    let options = ExtensionOptions {
        hidden_stems: ExtensionOptions::all().hidden_stems,
        ..ExtensionOptions::default()
    };
    // 阳一局甲子时: 甲子遁戊. Repeating 戊 at duty-door 坎一 starts from 中五.
    let chart = calculate_with_options(&ChartRequest::new(2024, 12, 26, 0), &options).unwrap();
    let hidden = chart.extensions.unwrap().hidden_stems.unwrap();
    assert!(hidden.used_center_fallback);
    assert_eq!(
        (hidden.start_palace, hidden.effective_hour_stem),
        (5, Stem::Wu)
    );
    assert_eq!(
        hidden.palaces.map(|item| item.stem),
        [
            Stem::Gui,
            Stem::Ding,
            Stem::Bing,
            Stem::Yi,
            Stem::Wu,
            Stem::Ji,
            Stem::Geng,
            Stem::Xin,
            Stem::Ren,
        ]
    );

    // 阳一局乙丑时: 乙 at duty-door 坤二, then forward numeric flight.
    let chart = calculate_with_options(&ChartRequest::new(2024, 12, 26, 1), &options).unwrap();
    let hidden = chart.extensions.unwrap().hidden_stems.unwrap();
    assert!(!hidden.used_center_fallback);
    assert_eq!(hidden.start_palace, 2);
    assert_eq!(
        hidden.palaces.map(|item| item.stem),
        [
            Stem::Bing,
            Stem::Yi,
            Stem::Wu,
            Stem::Ji,
            Stem::Geng,
            Stem::Xin,
            Stem::Ren,
            Stem::Gui,
            Stem::Ding,
        ]
    );

    // This independent base-plate fixture has BOTH leaders at 坤二, but 辛 is
    // the hosted center earth stem, not 坤二's native 丁: no center fallback.
    let chart = calculate_with_options(&ChartRequest::new(2016, 12, 2, 17), &options).unwrap();
    assert_eq!(
        (chart.leaders.star_palace, chart.leaders.door_palace),
        (2, 2)
    );
    assert_eq!(chart.palace(2).unwrap().hosted_earth_stem, Some(Stem::Xin));
    let hidden = chart.extensions.unwrap().hidden_stems.unwrap();
    assert!(!hidden.used_center_fallback);
    assert_eq!(hidden.start_palace, 2);
    assert_eq!(
        hidden.palaces.map(|item| item.stem),
        [
            Stem::Ren,
            Stem::Xin,
            Stem::Geng,
            Stem::Ji,
            Stem::Wu,
            Stem::Yi,
            Stem::Bing,
            Stem::Ding,
            Stem::Gui,
        ]
    );
}

#[test]
fn screenshot_star_and_door_strengths_use_distinct_tables() {
    use ElementStrengthState::{Qiu as EQiu, Si, Wang as EWang, Xiang as EXiang, Xiu as EXiu};
    use StarStrengthState::{Fei, Qiu, Wang, Xiang, Xiu};

    let chart = reference_chart();
    let strengths = chart.extensions.unwrap().strength.unwrap();
    assert_eq!(strengths.month_branch, Branch::You);
    assert_eq!(strengths.month_element, Element::Metal);
    // Independent gray labels below each star/door in the user screenshot.
    let expected = [
        (1, Wang, Xiang, EXiu, EWang),
        (2, Xiu, Qiu, EXiu, EQiu),
        (3, Wang, Fei, Si, EXiu),
        (4, Qiu, Wang, EWang, Si),
        (6, Wang, Wang, EWang, EWang),
        (7, Xiu, Xiu, EXiu, EXiu),
        (8, Fei, Xiang, Si, EXiang),
        (9, Wang, Qiu, EXiu, Si),
    ];
    for (number, star_palace, star_month, door_palace, door_month) in expected {
        let palace = &strengths.palaces[number - 1];
        for star in &palace.stars {
            assert_eq!(
                (star.at_palace, star.at_month),
                (star_palace, star_month),
                "palace {number}"
            );
        }
        let door = palace.door.as_ref().unwrap();
        assert_eq!(
            (door.at_palace, door.at_month),
            (door_palace, door_month),
            "palace {number}"
        );
    }
    assert_eq!(strengths.palaces[5].stars.len(), 2); // 天禽 remains separate.
    assert!(strengths.palaces[4].stars.is_empty());
    assert!(strengths.palaces[4].door.is_none());
}

#[test]
fn screenshot_growth_stages_preserve_both_branches_and_hosted_stem_identity() {
    use GrowthStage::{ChangSheng, GuanDai, Jue, LinGuan, Mu, Si, Tai, Yang};

    let chart = reference_chart();
    let growth = chart.extensions.unwrap().growth_stages.unwrap();
    // Gray stage pairs from the screenshot, not generated from a formula here.
    let expected = [
        (
            2,
            StemPlate::Heaven,
            Stem::Gui,
            false,
            vec![(Branch::Wei, Mu), (Branch::Shen, Si)],
        ),
        (
            6,
            StemPlate::Heaven,
            Stem::Bing,
            false,
            vec![(Branch::Xu, Mu), (Branch::Hai, Jue)],
        ),
        (
            8,
            StemPlate::Earth,
            Stem::Ji,
            false,
            vec![(Branch::Chou, Mu), (Branch::Yin, Si)],
        ),
        (
            4,
            StemPlate::Earth,
            Stem::Gui,
            false,
            vec![(Branch::Chen, Yang), (Branch::Si, Tai)],
        ),
        (
            2,
            StemPlate::Earth,
            Stem::Ren,
            true,
            vec![(Branch::Wei, Yang), (Branch::Shen, ChangSheng)],
        ),
        (
            6,
            StemPlate::Heaven,
            Stem::Ren,
            true,
            vec![(Branch::Xu, GuanDai), (Branch::Hai, LinGuan)],
        ),
    ];
    for (palace, plate, stem, hosted, branches) in expected {
        let result = growth
            .stems
            .iter()
            .find(|item| {
                let placement = &item.placement;
                (
                    placement.palace,
                    placement.plate,
                    placement.stem,
                    placement.is_center_hosted,
                ) == (palace, plate, stem, hosted)
            })
            .unwrap();
        assert_eq!(
            result
                .branches
                .iter()
                .map(|item| (item.branch, item.stage))
                .collect::<Vec<_>>(),
            branches
        );
        if hosted {
            assert_eq!(result.placement.source_palace, Some(5));
        }
    }
    for result in growth
        .stems
        .iter()
        .filter(|item| item.placement.palace == 5)
    {
        assert!(result.branches.is_empty());
    }
}

#[test]
fn full_growth_table_matches_classical_stage_rows_across_visible_stems() {
    // 三命通会·卷二·论天干阴阳生死, columns 子丑寅卯辰巳午未申酉戌亥.
    // Literal rows distinguish yin reversal, 火土同行 and every stage transition.
    let rows = [
        "沐浴 冠带 临官 帝旺 衰 病 死 墓 绝 胎 养 长生",
        "病 衰 帝旺 临官 冠带 沐浴 长生 养 胎 绝 墓 死",
        "胎 养 长生 沐浴 冠带 临官 帝旺 衰 病 死 墓 绝",
        "绝 墓 死 病 衰 帝旺 临官 冠带 沐浴 长生 养 胎",
        "胎 养 长生 沐浴 冠带 临官 帝旺 衰 病 死 墓 绝",
        "绝 墓 死 病 衰 帝旺 临官 冠带 沐浴 长生 养 胎",
        "死 墓 绝 胎 养 长生 沐浴 冠带 临官 帝旺 衰 病",
        "长生 养 胎 绝 墓 死 病 衰 帝旺 临官 冠带 沐浴",
        "帝旺 衰 病 死 墓 绝 胎 养 长生 沐浴 冠带 临官",
        "临官 冠带 沐浴 长生 养 胎 绝 墓 死 病 衰 帝旺",
    ];
    let expected = rows.map(|row| row.split_whitespace().collect::<Vec<_>>());
    let options = ExtensionOptions {
        growth_stages: ExtensionOptions::all().growth_stages,
        ..ExtensionOptions::default()
    };
    let mut visited = [[false; 12]; 10];
    for day in 1..=15 {
        for hour in (0..24).step_by(2) {
            let chart =
                calculate_with_options(&ChartRequest::new(2024, 1, day, hour), &options).unwrap();
            for stem in chart.extensions.unwrap().growth_stages.unwrap().stems {
                let index = usize::from(stem.placement.stem.index());
                for growth in stem.branches {
                    let branch = usize::from(growth.branch.index());
                    assert_eq!(growth.stage.name(), expected[index][branch]);
                    visited[index][branch] = true;
                }
            }
        }
    }
    // Visible Qimen stems exclude 甲; every branch for all nine others was checked.
    assert!(visited[1..].iter().flatten().all(|value| *value));
}

#[test]
fn screenshot_punishment_and_tombs_match_and_classical_yi_tomb_is_not_conflated() {
    let chart = reference_chart();
    let extensions = chart.extensions.as_ref().unwrap();
    let hits = extensions
        .punishments
        .as_ref()
        .unwrap()
        .stems
        .iter()
        .filter(|item| item.is_punished)
        .map(|item| {
            (
                item.placement.palace,
                item.placement.plate,
                item.placement.stem,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(hits, [(4, StemPlate::Earth, Stem::Gui)]);
    let tombs = extensions.tombs.as_ref().unwrap();
    for (palace, plate, stem, branch) in [
        (2, StemPlate::Heaven, Stem::Gui, Branch::Wei),
        (6, StemPlate::Heaven, Stem::Bing, Branch::Xu),
        (8, StemPlate::Earth, Stem::Ji, Branch::Chou),
    ] {
        let result = tombs
            .stems
            .iter()
            .find(|item| {
                (
                    item.placement.palace,
                    item.placement.plate,
                    item.placement.stem,
                ) == (palace, plate, stem)
            })
            .unwrap();
        assert_eq!(
            (result.tomb_branch, result.is_in_tomb),
            (Some(branch), Some(true))
        );
    }
    assert!(
        tombs
            .stems
            .iter()
            .filter(|item| item.placement.stem == Stem::Yi)
            .all(|item| item.tomb_branch == Some(Branch::Xu))
    );

    let options = ExtensionOptions {
        tombs: Some(TombRule::TraditionalThreeWonders),
        ..ExtensionOptions::default()
    };
    let classical = calculate_with_options(&chart.input, &options)
        .unwrap()
        .extensions
        .unwrap()
        .tombs
        .unwrap();
    assert!(
        classical
            .stems
            .iter()
            .filter(|item| item.placement.stem == Stem::Yi)
            .all(|item| item.tomb_branch == Some(Branch::Wei))
    );
    for item in classical
        .stems
        .iter()
        .filter(|item| item.placement.stem == Stem::Gui)
    {
        assert_eq!((item.tomb_branch, item.is_in_tomb), (None, None));
    }

    // 阴九局癸巳时: hidden 庚 at 兑七 moves to 癸 at 巽四, carrying
    // the original 坎一 heavenly 乙 to 坤二. Its 未 stage is 养, while
    // the independent classical 三奇 rule marks 乙入未墓.
    let input = ChartRequest::new(2026, 9, 19, 9);
    let ordinary = calculate_with_options(&input, &ExtensionOptions::all()).unwrap();
    assert_eq!(ordinary.calendar.four_pillars.hour.to_string(), "癸巳");
    assert_eq!(ordinary.palace(2).unwrap().heaven_stems[0].stem, Stem::Yi);
    let ordinary = ordinary.extensions.unwrap();
    let growth = ordinary
        .growth_stages
        .unwrap()
        .stems
        .into_iter()
        .find(|item| item.placement.palace == 2 && item.placement.plate == StemPlate::Heaven)
        .unwrap();
    assert_eq!(
        (growth.branches[0].branch, growth.branches[0].stage),
        (Branch::Wei, GrowthStage::Yang)
    );
    let ordinary_tomb = ordinary
        .tombs
        .unwrap()
        .stems
        .into_iter()
        .find(|item| item.placement.palace == 2 && item.placement.plate == StemPlate::Heaven)
        .unwrap();
    assert_eq!(ordinary_tomb.is_in_tomb, Some(false));
    let classical = calculate_with_options(&input, &options).unwrap();
    let classical_tomb = classical
        .extensions
        .unwrap()
        .tombs
        .unwrap()
        .stems
        .into_iter()
        .find(|item| item.placement.palace == 2 && item.placement.plate == StemPlate::Heaven)
        .unwrap();
    assert_eq!(classical_tomb.is_in_tomb, Some(true));
}

#[test]
fn door_pressure_does_not_mark_the_reverse_control_relationship() {
    let options = ExtensionOptions {
        door_pressure: ExtensionOptions::all().door_pressure,
        ..ExtensionOptions::default()
    };
    let chart = calculate_with_options(&ChartRequest::new(2024, 12, 26, 1), &options).unwrap();
    let doors = chart.extensions.unwrap().door_pressure.unwrap().doors;
    assert_eq!(
        doors
            .iter()
            .filter(|item| item.is_pressed)
            .map(|item| item.palace)
            .collect::<Vec<_>>(),
        [4]
    );
    let reverse = doors.iter().find(|item| item.palace == 2).unwrap();
    assert_eq!(
        (reverse.door_element, reverse.palace_element),
        (Element::Water, Element::Earth)
    );
    assert!(!reverse.is_pressed);
}

#[test]
fn day_horse_follows_the_selected_day_boundary() {
    let options = ExtensionOptions {
        day_horse: ExtensionOptions::all().day_horse,
        ..ExtensionOptions::default()
    };
    let mut request = ChartRequest::new(2026, 9, 18, 23);
    let zi = calculate_with_options(&request, &options).unwrap();
    request.day_boundary = DayBoundary::Midnight;
    let midnight = calculate_with_options(&request, &options).unwrap();
    let zi = zi.extensions.unwrap().day_horse.unwrap();
    let midnight = midnight.extensions.unwrap().day_horse.unwrap();
    assert_eq!(
        (zi.pillar.to_string(), zi.horse.branch),
        ("丙申".to_owned(), Branch::Yin)
    );
    assert_eq!(
        (midnight.pillar.to_string(), midnight.horse.branch),
        ("乙未".to_owned(), Branch::Si)
    );
}

#[test]
fn every_instrument_punishment_and_growth_tomb_matches_fixed_traditional_positions() {
    // 遁甲演义's six punishment positions; twelve-stage tombs are a separate
    // convention from its 三奇入墓 rule. Literal stem/branch/palace anchors
    // ensure that a wrong table entry cannot hide behind the screenshot's 癸.
    let punishments = [
        (Stem::Wu, "甲子", Branch::Mao, 3),
        (Stem::Ji, "甲戌", Branch::Wei, 2),
        (Stem::Geng, "甲申", Branch::Yin, 8),
        (Stem::Xin, "甲午", Branch::Wu, 9),
        (Stem::Ren, "甲辰", Branch::Chen, 4),
        (Stem::Gui, "甲寅", Branch::Si, 4),
    ];
    let tombs = [
        (Stem::Yi, Branch::Xu, 6),
        (Stem::Bing, Branch::Xu, 6),
        (Stem::Ding, Branch::Chou, 8),
        (Stem::Wu, Branch::Xu, 6),
        (Stem::Ji, Branch::Chou, 8),
        (Stem::Geng, Branch::Chou, 8),
        (Stem::Xin, Branch::Chen, 4),
        (Stem::Ren, Branch::Chen, 4),
        (Stem::Gui, Branch::Wei, 2),
    ];
    let options = ExtensionOptions {
        punishments: ExtensionOptions::all().punishments,
        tombs: ExtensionOptions::all().tombs,
        ..ExtensionOptions::default()
    };
    let mut observed_punishments = [false; 6];
    let mut observed_tombs = [false; 9];
    for day in 1..=15 {
        for hour in (0..24).step_by(2) {
            let chart =
                calculate_with_options(&ChartRequest::new(2024, 1, day, hour), &options).unwrap();
            let result = chart.extensions.unwrap();
            for item in result.punishments.unwrap().stems {
                let expected = punishments
                    .iter()
                    .enumerate()
                    .find(|(_, (stem, _, _, _))| *stem == item.placement.stem);
                if let Some((index, (_, head, branch, palace))) = expected {
                    assert_eq!(item.hidden_jia.unwrap().to_string(), *head);
                    assert_eq!(item.punished_branch, Some(*branch));
                    assert_eq!(item.is_punished, item.placement.palace == *palace);
                    observed_punishments[index] |= item.is_punished;
                } else {
                    assert!(item.hidden_jia.is_none());
                    assert!(item.punished_branch.is_none());
                    assert!(!item.is_punished);
                }
            }
            for item in result.tombs.unwrap().stems {
                let (index, (_, branch, palace)) = tombs
                    .iter()
                    .enumerate()
                    .find(|(_, (stem, _, _))| *stem == item.placement.stem)
                    .unwrap();
                assert_eq!(item.tomb_branch, Some(*branch));
                assert_eq!(item.is_in_tomb, Some(item.placement.palace == *palace));
                observed_tombs[index] |= item.is_in_tomb == Some(true);
            }
        }
    }
    assert!(observed_punishments.into_iter().all(|value| value));
    assert!(observed_tombs.into_iter().all(|value| value));
}
