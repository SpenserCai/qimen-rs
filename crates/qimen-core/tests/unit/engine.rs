use std::collections::HashSet;

use super::*;

fn make_plates(dun: Dun, ju: u8, hour: u8) -> ([Palace; 9], Leaders) {
    let hour = Cycle::from_index(hour);
    plates(dun, ju, hour, &xun(hour), &horse(hour.branch))
}

#[test]
fn yang_one_jia_zi_is_a_stationary_plate() {
    let (palaces, leaders) = make_plates(Dun::Yang, 1, 0);
    assert_eq!(leaders.star, Star::TianPeng);
    assert_eq!(leaders.door, Door::Xiu);
    assert_eq!(leaders.original_palace, 1);
    assert_eq!(leaders.star_palace, 1);
    assert_eq!(leaders.door_palace, 1);
    for palace in &palaces {
        if palace.number == 5 {
            continue;
        }
        assert_eq!(palace.stars[0], STARS[usize::from(palace.number - 1)]);
        assert_eq!(palace.heaven_stems[0].stem, palace.earth_stem);
        assert_eq!(palace.door, Some(DOORS[usize::from(palace.number - 1)]));
    }
    assert_eq!(palaces[1].stars, [Star::TianRui, Star::TianQin]);
    for (number, deity) in RING.into_iter().zip(DEITIES) {
        assert_eq!(palaces[usize::from(number - 1)].deity, Some(deity));
    }
}

#[test]
fn independently_hand_calculated_yang_one_yi_chou() {
    let (palaces, leaders) = make_plates(Dun::Yang, 1, 1);
    assert_eq!(leaders.star_palace, 9);
    assert_eq!(leaders.door_palace, 2);
    let heaven = [
        Some(Stem::Yi),
        Some(Stem::Bing),
        Some(Stem::Ding),
        Some(Stem::Gui),
        None,
        Some(Stem::Xin),
        Some(Stem::Geng),
        Some(Stem::Ji),
        Some(Stem::Wu),
    ];
    let stars = [
        Some(Star::TianYing),
        Some(Star::TianRen),
        Some(Star::TianZhu),
        Some(Star::TianXin),
        None,
        Some(Star::TianFu),
        Some(Star::TianChong),
        Some(Star::TianRui),
        Some(Star::TianPeng),
    ];
    let doors = [
        Some(Door::Du),
        Some(Door::Xiu),
        Some(Door::Si),
        Some(Door::Jing),
        None,
        Some(Door::Shang),
        Some(Door::Sheng),
        Some(Door::Scene),
        Some(Door::Kai),
    ];
    let deities = [
        Some(Deity::BaiHu),
        Some(Deity::TengShe),
        Some(Deity::JiuDi),
        Some(Deity::JiuTian),
        None,
        Some(Deity::LiuHe),
        Some(Deity::TaiYin),
        Some(Deity::XuanWu),
        Some(Deity::ZhiFu),
    ];
    for index in 0..9 {
        assert_eq!(
            palaces[index].heaven_stems.first().map(|stem| stem.stem),
            heaven[index]
        );
        assert_eq!(palaces[index].stars.first().copied(), stars[index]);
        assert_eq!(palaces[index].door, doors[index]);
        assert_eq!(palaces[index].deity, deities[index]);
    }
    assert_eq!(palaces[7].stars, [Star::TianRui, Star::TianQin]);
    assert_eq!(palaces[7].heaven_stems[1].stem, Stem::Ren);
    assert!(palaces[7].heaven_stems[1].is_center_hosted);
}

#[test]
fn duty_door_counts_from_actual_center_before_hosting() {
    let (_, leaders) = make_plates(Dun::Yang, 5, 1);
    assert_eq!(leaders.original_palace, 5);
    assert_eq!(leaders.star, Star::TianQin);
    assert_eq!(leaders.door, Door::Si);
    assert_eq!(leaders.star_palace, 4);
    assert_eq!(leaders.door_raw_palace, 6);
    assert_eq!(leaders.door_palace, 6);
}

#[test]
fn all_eighteen_formations_and_sixty_hours_preserve_plate_invariants() {
    for dun in [Dun::Yang, Dun::Yin] {
        for ju in 1..=9 {
            for hour in 0..60 {
                let (palaces, leaders) = make_plates(dun, ju, hour);
                let context = format!("{dun:?} {ju} {hour}");
                let earth: HashSet<_> = palaces.iter().map(|p| p.earth_stem).collect();
                let heaven: HashSet<_> = palaces
                    .iter()
                    .flat_map(|p| p.heaven_stems.iter().map(|s| s.stem))
                    .collect();
                let stars: Vec<_> = palaces
                    .iter()
                    .flat_map(|p| p.stars.iter().copied())
                    .collect();
                let doors: Vec<_> = palaces.iter().filter_map(|p| p.door).collect();
                let deities: Vec<_> = palaces.iter().filter_map(|p| p.deity).collect();
                assert_eq!(earth.len(), 9, "{context}");
                assert!(!earth.contains(&Stem::Jia), "{context}");
                assert_eq!(heaven, earth, "{context}");
                assert_eq!(stars.len(), 9, "{context}");
                assert_eq!(stars.iter().collect::<HashSet<_>>().len(), 9, "{context}");
                assert_eq!(doors.len(), 8, "{context}");
                assert_eq!(doors.iter().collect::<HashSet<_>>().len(), 8, "{context}");
                assert_eq!(deities.len(), 8, "{context}");
                assert_eq!(deities.iter().collect::<HashSet<_>>().len(), 8, "{context}");
                assert_eq!(
                    palaces.iter().filter(|p| p.is_horse).count(),
                    1,
                    "{context}"
                );
                assert_eq!(
                    palaces.iter().map(|p| p.void_branches.len()).sum::<usize>(),
                    2,
                    "{context}"
                );
                let center = &palaces[4];
                assert!(center.stars.is_empty(), "{context}");
                assert!(center.heaven_stems.is_empty(), "{context}");
                assert_eq!(center.door, None, "{context}");
                assert_eq!(center.deity, None, "{context}");
                let qin = palaces
                    .iter()
                    .find(|p| p.stars.contains(&Star::TianQin))
                    .unwrap();
                assert!(qin.stars.contains(&Star::TianRui), "{context}");
                assert_eq!(qin.heaven_stems.len(), 2, "{context}");
                assert_eq!(qin.heaven_stems[1].stem, center.earth_stem, "{context}");
                assert!(qin.heaven_stems[1].is_center_hosted, "{context}");
                let duty = &palaces[usize::from(leaders.star_palace - 1)];
                assert!(duty.stars.contains(&leaders.star), "{context}");
                assert_eq!(duty.deity, Some(Deity::ZhiFu), "{context}");
                assert_eq!(
                    palaces[usize::from(leaders.door_palace - 1)].door,
                    Some(leaders.door),
                    "{context}"
                );
            }
        }
    }
}

#[test]
fn all_six_xun_have_the_traditional_hidden_stems_and_voids() {
    let expected = [
        (Stem::Wu, [Branch::Xu, Branch::Hai]),
        (Stem::Ji, [Branch::Shen, Branch::You]),
        (Stem::Geng, [Branch::Wu, Branch::Wei]),
        (Stem::Xin, [Branch::Chen, Branch::Si]),
        (Stem::Ren, [Branch::Yin, Branch::Mao]),
        (Stem::Gui, [Branch::Zi, Branch::Chou]),
    ];
    for (index, (hidden, voids)) in expected.into_iter().enumerate() {
        for offset in 0..10 {
            let xun = xun(Cycle::from_index(index as u8 * 10 + offset));
            assert_eq!(xun.hidden_stem, hidden);
            assert_eq!(xun.void_branches, voids);
            assert_eq!(xun.head.index, index as u8 * 10);
        }
    }
}

#[test]
fn horse_covers_all_four_three_branch_groups() {
    for (branches, expected, palace) in [
        ([Branch::Shen, Branch::Zi, Branch::Chen], Branch::Yin, 8),
        ([Branch::Yin, Branch::Wu, Branch::Xu], Branch::Shen, 2),
        ([Branch::Hai, Branch::Mao, Branch::Wei], Branch::Si, 4),
        ([Branch::Si, Branch::You, Branch::Chou], Branch::Hai, 6),
    ] {
        for branch in branches {
            assert_eq!(
                horse(branch),
                Horse {
                    branch: expected,
                    palace
                }
            );
        }
    }
}
