use std::io::{self, Write};

use qimen_core::{
    ChartExtensions, GrowthRule, HiddenStemRule, StemPlacement, StemPlate, StrengthRule, Strengths,
    TombRule,
};

fn placement(item: &StemPlacement) -> String {
    let plate = match item.plate {
        StemPlate::Heaven => "天盘",
        StemPlate::Earth => "地盘",
        StemPlate::Hidden => "暗干",
    };
    let source = item.source_palace.map_or_else(String::new, |palace| {
        if item.is_center_hosted {
            format!("(寄，自{palace}宫)")
        } else if matches!(item.plate, StemPlate::Heaven) {
            format!("(自{palace}宫)")
        } else {
            String::new()
        }
    });
    format!("{}宫 {plate} {}{source}", item.palace, item.stem)
}

fn strengths(output: &mut impl Write, values: &Strengths) -> io::Result<()> {
    let rule = match values.rule {
        StrengthRule::ClassicalStarsAndFiveElements => "九星传统旺衰；门干五行旺衰",
    };
    writeln!(
        output,
        "旺衰 · {rule} · 节令月支 {} / {} · 分列落宫、月令",
        values.month_branch, values.month_element
    )?;
    for palace in &values.palaces {
        for star in &palace.stars {
            writeln!(
                output,
                "  {}宫 {}({})：落宫{}，月令{}",
                palace.palace,
                star.star,
                star.element,
                star.at_palace.name(),
                star.at_month.name()
            )?;
        }
        if let Some(door) = &palace.door {
            writeln!(
                output,
                "  {}宫 {}({})：落宫{}，月令{}",
                palace.palace,
                door.door,
                door.element,
                door.at_palace.name(),
                door.at_month.name()
            )?;
        }
        for stem in &palace.stems {
            writeln!(
                output,
                "  {}({})：落宫{}，月令{}",
                placement(&stem.placement),
                stem.element,
                stem.at_palace.name(),
                stem.at_month.name()
            )?;
        }
    }
    Ok(())
}

pub(crate) fn write(output: &mut impl Write, values: &ChartExtensions) -> io::Result<()> {
    writeln!(output, "\n扩展明细")?;
    if let Some(hidden) = &values.hidden_stems {
        let rule = match hidden.rule {
            HiddenStemRule::DutyDoorHourStemWithCenterFallback => {
                "时干从值使宫飞布，重地盘本位干时改中五起"
            }
        };
        writeln!(
            output,
            "暗干 · {rule} · 有效时干 {} · 起{}宫{}",
            hidden.effective_hour_stem,
            hidden.start_palace,
            if hidden.used_center_fallback {
                "（已改中五起）"
            } else {
                ""
            }
        )?;
        let palaces = hidden
            .palaces
            .iter()
            .map(|palace| format!("{}宫 {}", palace.palace, palace.stem))
            .collect::<Vec<_>>()
            .join(" · ");
        writeln!(output, "  {palaces}")?;
    }
    if let Some(day) = &values.day_horse {
        writeln!(
            output,
            "日马 · 日柱 {} · {} / {}宫（三合日马）",
            day.pillar, day.horse.branch, day.horse.palace
        )?;
    }
    if let Some(values) = &values.strength {
        strengths(output, values)?;
    }
    if let Some(growth) = &values.growth_stages {
        let rule = match growth.rule {
            GrowthRule::YangForwardYinReverseFireEarth => "阳顺阴逆，土随火；双支宫逐支列出",
        };
        writeln!(output, "十二长生 · {rule}")?;
        for stem in &growth.stems {
            let branches = if stem.branches.is_empty() {
                "中五无地支，不适用".to_owned()
            } else {
                stem.branches
                    .iter()
                    .map(|branch| format!("{} {}", branch.branch, branch.stage.name()))
                    .collect::<Vec<_>>()
                    .join(" / ")
            };
            writeln!(output, "  {}：{branches}", placement(&stem.placement))?;
        }
    }
    if let Some(punishments) = &values.punishments {
        writeln!(output, "六仪击刑 · 天盘六仪为常用判据；地盘、暗干分别列示")?;
        for stem in &punishments.stems {
            let condition = match (stem.hidden_jia, stem.punished_branch) {
                (Some(jia), Some(branch)) => format!(
                    "{jia} · 刑支{branch} · {}",
                    if stem.is_punished {
                        "击刑"
                    } else {
                        "未击刑"
                    }
                ),
                _ => "非六仪，不适用".to_owned(),
            };
            writeln!(output, "  {}：{condition}", placement(&stem.placement))?;
        }
    }
    if let Some(tombs) = &values.tombs {
        let rule = match tombs.rule {
            TombRule::GrowthStageFireEarth => "十二长生入墓，阳顺阴逆土随火（乙墓戌）",
            TombRule::TraditionalThreeWonders => "传统三奇入墓：乙未、丙戌、丁丑；六仪不适用",
        };
        writeln!(output, "入墓 · {rule}")?;
        for stem in &tombs.stems {
            let condition = match (stem.tomb_branch, stem.is_in_tomb) {
                (Some(branch), Some(is_in_tomb)) => format!(
                    "墓支{branch} · {}",
                    if is_in_tomb { "入墓" } else { "未入墓" }
                ),
                _ => "所选规则不适用".to_owned(),
            };
            writeln!(output, "  {}：{condition}", placement(&stem.placement))?;
        }
    }
    if let Some(pressure) = &values.door_pressure {
        writeln!(output, "门迫 · 门五行克落宫五行")?;
        for door in &pressure.doors {
            writeln!(
                output,
                "  {}宫 {}({}) / 宫{}：{}",
                door.palace,
                door.door,
                door.door_element,
                door.palace_element,
                if door.is_pressed {
                    "门迫"
                } else {
                    "无门迫"
                }
            )?;
        }
    }
    Ok(())
}
