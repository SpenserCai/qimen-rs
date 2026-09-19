use super::{
    DoorPressure, DoorPressureRule, DoorPressures, DoorStrength, ElementStrengthState,
    PalaceStrength, StarStrength, StarStrengthState, StemPlacement, StemStrength, StrengthRule,
    Strengths,
};
use crate::{Branch, Chart, Door, Element, Star, Stem};

const fn generates(source: Element, target: Element) -> bool {
    matches!(
        (source, target),
        (Element::Wood, Element::Fire)
            | (Element::Fire, Element::Earth)
            | (Element::Earth, Element::Metal)
            | (Element::Metal, Element::Water)
            | (Element::Water, Element::Wood)
    )
}

const fn controls(source: Element, target: Element) -> bool {
    matches!(
        (source, target),
        (Element::Wood, Element::Earth)
            | (Element::Earth, Element::Water)
            | (Element::Water, Element::Fire)
            | (Element::Fire, Element::Metal)
            | (Element::Metal, Element::Wood)
    )
}

const fn star_element(star: Star) -> Element {
    match star {
        Star::TianPeng => Element::Water,
        Star::TianChong | Star::TianFu => Element::Wood,
        Star::TianYing => Element::Fire,
        Star::TianXin | Star::TianZhu => Element::Metal,
        Star::TianRui | Star::TianQin | Star::TianRen => Element::Earth,
    }
}

const fn door_element(door: Door) -> Element {
    match door {
        Door::Xiu => Element::Water,
        Door::Shang | Door::Du => Element::Wood,
        Door::Scene => Element::Fire,
        Door::Kai | Door::Jing => Element::Metal,
        Door::Si | Door::Sheng => Element::Earth,
    }
}

const fn stem_element(stem: Stem) -> Element {
    match stem {
        Stem::Jia | Stem::Yi => Element::Wood,
        Stem::Bing | Stem::Ding => Element::Fire,
        Stem::Wu | Stem::Ji => Element::Earth,
        Stem::Geng | Stem::Xin => Element::Metal,
        Stem::Ren | Stem::Gui => Element::Water,
    }
}

const fn branch_element(branch: Branch) -> Element {
    match branch {
        Branch::Yin | Branch::Mao => Element::Wood,
        Branch::Si | Branch::Wu => Element::Fire,
        Branch::Shen | Branch::You => Element::Metal,
        Branch::Hai | Branch::Zi => Element::Water,
        Branch::Chen | Branch::Xu | Branch::Chou | Branch::Wei => Element::Earth,
    }
}

fn star_state(subject: Element, environment: Element) -> StarStrengthState {
    if subject == environment {
        StarStrengthState::Xiang
    } else if generates(subject, environment) {
        StarStrengthState::Wang
    } else if generates(environment, subject) {
        StarStrengthState::Fei
    } else if controls(subject, environment) {
        StarStrengthState::Xiu
    } else {
        StarStrengthState::Qiu
    }
}

fn element_state(subject: Element, environment: Element) -> ElementStrengthState {
    if subject == environment {
        ElementStrengthState::Wang
    } else if generates(environment, subject) {
        ElementStrengthState::Xiang
    } else if generates(subject, environment) {
        ElementStrengthState::Xiu
    } else if controls(subject, environment) {
        ElementStrengthState::Qiu
    } else {
        ElementStrengthState::Si
    }
}

pub(super) fn calculate(
    chart: &Chart,
    placements: &[StemPlacement],
    rule: StrengthRule,
) -> Strengths {
    let month_branch = chart.calendar.four_pillars.month.branch;
    let month_element = branch_element(month_branch);
    Strengths {
        rule,
        month_branch,
        month_element,
        palaces: chart
            .palaces
            .iter()
            .map(|palace| PalaceStrength {
                palace: palace.number,
                element: palace.element,
                stars: palace
                    .stars
                    .iter()
                    .map(|&star| {
                        let element = star_element(star);
                        StarStrength {
                            star,
                            element,
                            at_palace: star_state(element, palace.element),
                            at_month: star_state(element, month_element),
                        }
                    })
                    .collect(),
                door: palace.door.map(|door| {
                    let element = door_element(door);
                    DoorStrength {
                        door,
                        element,
                        at_palace: element_state(element, palace.element),
                        at_month: element_state(element, month_element),
                    }
                }),
                stems: placements
                    .iter()
                    .filter(|stem| stem.palace == palace.number)
                    .map(|placement| {
                        let element = stem_element(placement.stem);
                        StemStrength {
                            placement: placement.clone(),
                            element,
                            at_palace: element_state(element, palace.element),
                            at_month: element_state(element, month_element),
                        }
                    })
                    .collect(),
            })
            .collect(),
    }
}

pub(super) fn door_pressure(chart: &Chart, rule: DoorPressureRule) -> DoorPressures {
    DoorPressures {
        rule,
        doors: chart
            .palaces
            .iter()
            .filter_map(|palace| {
                palace.door.map(|door| {
                    let door_element = door_element(door);
                    DoorPressure {
                        palace: palace.number,
                        door,
                        door_element,
                        palace_element: palace.element,
                        is_pressed: controls(door_element, palace.element),
                    }
                })
            })
            .collect(),
    }
}
