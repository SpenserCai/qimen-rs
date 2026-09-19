use super::placement::palace_branches;
use super::{
    BranchGrowth, GrowthRule, GrowthStage, GrowthStages, PunishmentRule, Punishments, StemGrowth,
    StemPlacement, StemPunishment, StemTomb, TombRule, Tombs,
};
use crate::{Branch, Cycle, Stem};

const STAGES: [GrowthStage; 12] = [
    GrowthStage::ChangSheng,
    GrowthStage::MuYu,
    GrowthStage::GuanDai,
    GrowthStage::LinGuan,
    GrowthStage::DiWang,
    GrowthStage::Shuai,
    GrowthStage::Bing,
    GrowthStage::Si,
    GrowthStage::Mu,
    GrowthStage::Jue,
    GrowthStage::Tai,
    GrowthStage::Yang,
];

// 三命通会·卷二·论天干阴阳生死: 戊 follows 丙, 己 follows 丁.
const STARTS: [u8; 10] = [11, 6, 2, 9, 2, 9, 5, 0, 8, 3];

fn stage(stem: Stem, branch: Branch) -> GrowthStage {
    let start = i16::from(STARTS[usize::from(stem.index())]);
    let direction = if stem.index().is_multiple_of(2) {
        1
    } else {
        -1
    };
    let index = (direction * (i16::from(branch.index()) - start)).rem_euclid(12);
    STAGES[index as usize]
}

pub(super) fn calculate(placements: &[StemPlacement], rule: GrowthRule) -> GrowthStages {
    GrowthStages {
        rule,
        stems: placements
            .iter()
            .map(|placement| StemGrowth {
                placement: placement.clone(),
                branches: palace_branches(placement.palace)
                    .iter()
                    .map(|&branch| BranchGrowth {
                        branch,
                        stage: stage(placement.stem, branch),
                    })
                    .collect(),
            })
            .collect(),
    }
}

pub(super) fn tombs(placements: &[StemPlacement], rule: TombRule) -> Tombs {
    Tombs {
        rule,
        stems: placements
            .iter()
            .map(|placement| {
                let tomb_branch = match rule {
                    TombRule::GrowthStageFireEarth => {
                        let start = i16::from(STARTS[usize::from(placement.stem.index())]);
                        let direction = if placement.stem.index().is_multiple_of(2) {
                            1
                        } else {
                            -1
                        };
                        Some(Branch::from_index(
                            (start + direction * 8).rem_euclid(12) as u8
                        ))
                    }
                    TombRule::TraditionalThreeWonders => match placement.stem {
                        Stem::Yi => Some(Branch::Wei),
                        Stem::Bing => Some(Branch::Xu),
                        Stem::Ding => Some(Branch::Chou),
                        _ => None,
                    },
                };
                StemTomb {
                    placement: placement.clone(),
                    tomb_branch,
                    is_in_tomb: tomb_branch
                        .map(|branch| palace_branches(placement.palace).contains(&branch)),
                }
            })
            .collect(),
    }
}

pub(super) fn punishments(placements: &[StemPlacement], rule: PunishmentRule) -> Punishments {
    Punishments {
        rule,
        stems: placements
            .iter()
            .map(|placement| {
                // 六仪藏甲: 子刑卯、戌刑未、申刑寅、午自刑、辰自刑、寅刑巳.
                let pair = match placement.stem {
                    Stem::Wu => Some((0, Branch::Mao)),
                    Stem::Ji => Some((10, Branch::Wei)),
                    Stem::Geng => Some((20, Branch::Yin)),
                    Stem::Xin => Some((30, Branch::Wu)),
                    Stem::Ren => Some((40, Branch::Chen)),
                    Stem::Gui => Some((50, Branch::Si)),
                    _ => None,
                };
                StemPunishment {
                    placement: placement.clone(),
                    hidden_jia: pair.map(|(index, _)| Cycle::from_index(index)),
                    punished_branch: pair.map(|(_, branch)| branch),
                    is_punished: pair.is_some_and(|(_, branch)| {
                        palace_branches(placement.palace).contains(&branch)
                    }),
                }
            })
            .collect(),
    }
}
