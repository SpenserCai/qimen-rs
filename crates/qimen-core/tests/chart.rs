//! Public-API regression cases, including a visually transcribed external chart.

use qimen_core::{
    Branch, Chart, ChartRequest, Deity, Door, Dun, Star, Stem, Yuan, calculate, calculate_json,
    calculate_json_pretty,
};

#[test]
fn center_origin_regression_through_the_calendar_entrypoint() {
    // Independently cross-checked in lunar-python 1.4.8: 癸卯 乙丑 甲戌 乙丑.
    let chart = calculate(&ChartRequest::new(2024, 1, 11, 1)).unwrap();
    assert_eq!(chart.calendar.four_pillars.day.to_string(), "甲戌");
    assert_eq!(chart.calendar.four_pillars.hour.to_string(), "乙丑");
    assert_eq!(chart.calendar.solar_term.name, "小寒");
    assert_eq!(
        (chart.dun, chart.yuan, chart.ju),
        (Dun::Yang, Yuan::Lower, 5)
    );
    assert_eq!(chart.leaders.original_palace, 5);
    assert_eq!(chart.leaders.star, Star::TianQin);
    assert_eq!(chart.leaders.star_palace, 4);
    assert_eq!(chart.leaders.door, Door::Si);
    assert_eq!(chart.leaders.door_palace, 6);
}

#[test]
fn winter_solstice_upper_yuan_yang_one_through_calendar() {
    // Independent calendar anchor: 甲辰 丙子 甲子 乙丑.
    let chart = calculate(&ChartRequest::new(2024, 12, 26, 1)).unwrap();
    assert_eq!(chart.calendar.four_pillars.day.to_string(), "甲子");
    assert_eq!(chart.calendar.four_pillars.hour.to_string(), "乙丑");
    assert_eq!(chart.calendar.solar_term.name, "冬至");
    assert_eq!(
        (chart.dun, chart.yuan, chart.ju),
        (Dun::Yang, Yuan::Upper, 1)
    );
    assert_eq!(chart.leaders.star_palace, 9);
    assert_eq!(chart.leaders.door_palace, 2);
}

#[test]
fn external_2021_yang_seven_plate_matches_explicitly_normalized_deity_names() {
    // https://github.com/kentang2017/kinqimen/issues/4
    // https://user-images.githubusercontent.com/34528743/110466119-46aaf180-8110-11eb-882c-5f058f7bf6de.png
    // This software names the yang-dun fifth/sixth deities 勾陈/朱雀.
    // Our documented convention consistently names those positions 白虎/玄武.
    // Only those two names are normalized; every palace placement is transcribed.
    let chart = calculate(&ChartRequest::new(2021, 3, 9, 9)).unwrap();
    assert_eq!(chart.calendar.four_pillars.year.to_string(), "辛丑");
    assert_eq!(chart.calendar.four_pillars.month.to_string(), "辛卯");
    assert_eq!(chart.calendar.four_pillars.day.to_string(), "丙辰");
    assert_eq!(chart.calendar.four_pillars.hour.to_string(), "癸巳");
    assert_eq!(
        (chart.dun, chart.yuan, chart.ju),
        (Dun::Yang, Yuan::Middle, 7)
    );
    assert_eq!(chart.xun.head.to_string(), "甲申");
    assert_eq!(chart.leaders.star, Star::TianYing);
    assert_eq!(chart.leaders.star_palace, 3);
    assert_eq!(chart.leaders.door, Door::Scene);
    assert_eq!(chart.leaders.door_palace, 9);
    let earth = [
        Stem::Xin,
        Stem::Ren,
        Stem::Gui,
        Stem::Ding,
        Stem::Bing,
        Stem::Yi,
        Stem::Wu,
        Stem::Ji,
        Stem::Geng,
    ];
    let heaven = [
        Some(Stem::Gui),
        Some(Stem::Yi),
        Some(Stem::Geng),
        Some(Stem::Ren),
        None,
        Some(Stem::Ji),
        Some(Stem::Xin),
        Some(Stem::Ding),
        Some(Stem::Wu),
    ];
    let stars = [
        Some(Star::TianChong),
        Some(Star::TianXin),
        Some(Star::TianYing),
        Some(Star::TianRui),
        None,
        Some(Star::TianRen),
        Some(Star::TianPeng),
        Some(Star::TianFu),
        Some(Star::TianZhu),
    ];
    let doors = [
        Some(Door::Xiu),
        Some(Door::Si),
        Some(Door::Shang),
        Some(Door::Du),
        None,
        Some(Door::Kai),
        Some(Door::Jing),
        Some(Door::Sheng),
        Some(Door::Scene),
    ];
    let deities = [
        Some(Deity::JiuDi),
        Some(Deity::LiuHe),
        Some(Deity::ZhiFu),
        Some(Deity::TengShe),
        None,
        Some(Deity::XuanWu),
        Some(Deity::BaiHu),
        Some(Deity::JiuTian),
        Some(Deity::TaiYin),
    ];
    for index in 0..9 {
        let palace = &chart.palaces[index];
        assert_eq!(palace.earth_stem, earth[index]);
        assert_eq!(
            palace.heaven_stems.first().map(|stem| stem.stem),
            heaven[index]
        );
        assert_eq!(palace.stars.first().copied(), stars[index]);
        assert_eq!(palace.door, doors[index]);
        assert_eq!(palace.deity, deities[index]);
    }
    assert_eq!(chart.palaces[3].stars, [Star::TianRui, Star::TianQin]);
    assert_eq!(chart.palaces[3].heaven_stems[1].stem, Stem::Bing);
    assert_eq!(chart.xun.void_branches, [Branch::Wu, Branch::Wei]);
    assert_eq!(chart.pillar_voids.day, [Branch::Zi, Branch::Chou]);
    assert_eq!(chart.horse.branch, Branch::Hai);
    assert_eq!(chart.horse.palace, 6);
}

#[test]
fn external_2016_yin_eight_plate_matches_all_visible_cells() {
    // Independently published screenshot of corrected 阴八局, not output from our engine.
    // https://github.com/kentang2017/kinqimen/issues/27
    // https://user-images.githubusercontent.com/36002974/159656439-c576bbf7-754e-4ddb-b26b-83080d50205d.jpg
    // Screenshot labels raw destinations as five; the cells host them at 坤二.
    let chart = calculate(&ChartRequest::new(2016, 12, 2, 17)).unwrap();
    assert_eq!(chart.calendar.four_pillars.year.to_string(), "丙申");
    assert_eq!(chart.calendar.four_pillars.month.to_string(), "己亥");
    assert_eq!(chart.calendar.four_pillars.day.to_string(), "戊午");
    assert_eq!(chart.calendar.four_pillars.hour.to_string(), "辛酉");
    assert_eq!(chart.calendar.solar_term.name, "小雪");
    assert_eq!(chart.dun, Dun::Yin);
    assert_eq!(chart.yuan, Yuan::Middle);
    assert_eq!(chart.ju, 8);
    assert_eq!(chart.yuan_head.to_string(), "甲寅");
    assert_eq!(chart.xun.head.to_string(), "甲寅");
    assert_eq!(chart.xun.hidden_stem, Stem::Gui);
    assert_eq!(chart.xun.void_branches, [Branch::Zi, Branch::Chou]);
    assert_eq!(chart.leaders.original_palace, 3);
    assert_eq!(chart.leaders.star, Star::TianChong);
    assert_eq!(chart.leaders.star_palace, 2);
    assert_eq!(chart.leaders.door, Door::Shang);
    assert_eq!(chart.leaders.door_raw_palace, 5);
    assert_eq!(chart.leaders.door_palace, 2);
    let earth = [
        Stem::Bing,
        Stem::Ding,
        Stem::Gui,
        Stem::Ren,
        Stem::Xin,
        Stem::Geng,
        Stem::Ji,
        Stem::Wu,
        Stem::Yi,
    ];
    let heaven = [
        Some(Stem::Ding),
        Some(Stem::Gui),
        Some(Stem::Geng),
        Some(Stem::Bing),
        None,
        Some(Stem::Yi),
        Some(Stem::Ren),
        Some(Stem::Ji),
        Some(Stem::Wu),
    ];
    let stars = [
        Some(Star::TianRui),
        Some(Star::TianChong),
        Some(Star::TianXin),
        Some(Star::TianPeng),
        None,
        Some(Star::TianYing),
        Some(Star::TianFu),
        Some(Star::TianZhu),
        Some(Star::TianRen),
    ];
    let doors = [
        Some(Door::Si),
        Some(Door::Shang),
        Some(Door::Kai),
        Some(Door::Xiu),
        None,
        Some(Door::Scene),
        Some(Door::Du),
        Some(Door::Jing),
        Some(Door::Sheng),
    ];
    let deities = [
        Some(Deity::XuanWu),
        Some(Deity::ZhiFu),
        Some(Deity::LiuHe),
        Some(Deity::TaiYin),
        None,
        Some(Deity::JiuDi),
        Some(Deity::JiuTian),
        Some(Deity::BaiHu),
        Some(Deity::TengShe),
    ];
    for index in 0..9 {
        let palace = &chart.palaces[index];
        assert_eq!(palace.number, index as u8 + 1);
        assert_eq!(palace.earth_stem, earth[index]);
        assert_eq!(
            palace.heaven_stems.first().map(|stem| stem.stem),
            heaven[index]
        );
        assert_eq!(palace.stars.first().copied(), stars[index]);
        assert_eq!(palace.door, doors[index]);
        assert_eq!(palace.deity, deities[index]);
    }
    assert_eq!(chart.palaces[0].stars, [Star::TianRui, Star::TianQin]);
    assert_eq!(chart.palaces[0].heaven_stems[1].stem, Stem::Xin);
    assert_eq!(chart.palaces[1].hosted_earth_stem, Some(Stem::Xin));
}

#[test]
fn strict_input_and_versioned_output_roundtrip() {
    let compact = calculate_json(r#"{"year":2026,"month":9,"day":18,"hour":14}"#).unwrap();
    let chart: Chart = serde_json::from_str(&compact).unwrap();
    assert_eq!(chart.schema_version, "1.0");
    assert_eq!(chart.input.utc_offset_minutes, 480);
    assert_eq!(chart.input.minute, 0);
    let pretty = calculate_json_pretty(r#"{"year":2026,"month":9,"day":18,"hour":14}"#).unwrap();
    assert!(pretty.contains('\n'));
    assert_eq!(serde_json::from_str::<Chart>(&pretty).unwrap(), chart);
    assert!(chart.palace(0).is_none());
    assert!(chart.palace(10).is_none());
    assert!(chart.palace(255).is_none());
    assert_eq!(chart.palace(9).unwrap().number, 9);
}

#[test]
fn malformed_and_out_of_range_inputs_return_errors() {
    for input in [
        "",
        "{}",
        "[]",
        "null",
        r#"{"year":2026,"month":9,"day":18,"hour":14,"timezone":"UTC"}"#,
        r#"{"year":2026,"month":2,"day":30,"hour":14}"#,
        r#"{"year":2026,"month":9,"day":18,"hour":24}"#,
        r#"{"year":2026,"month":9,"day":18,"hour":-1}"#,
        r#"{"year":2026,"month":9,"day":18,"hour":14,"utc_offset_minutes":841}"#,
        r#"{"year":1899,"month":1,"day":1,"hour":0}"#,
    ] {
        assert!(calculate_json(input).is_err(), "{input}");
    }
}

#[cfg(feature = "schema")]
#[test]
fn schemas_are_generated_from_the_actual_public_types() {
    let request = serde_json::to_value(qimen_core::request_schema()).unwrap();
    assert_eq!(request["additionalProperties"], false);
    assert_eq!(request["properties"]["utc_offset_minutes"]["default"], 480);
    let output = serde_json::to_value(qimen_core::chart_schema()).unwrap();
    assert!(output["properties"]["palaces"].is_object());
    assert!(output["$defs"]["Palace"]["properties"]["heaven_stems"].is_object());
}
