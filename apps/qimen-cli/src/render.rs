use std::io::{self, Write};

use qimen_calendar::{CalendarRequest, CalendarResult, CivilDateTime, DayBoundary};
use qimen_core::{Chart, Palace};
use unicode_width::UnicodeWidthStr;

fn offset(minutes: i32) -> String {
    let sign = if minutes < 0 { '-' } else { '+' };
    let absolute = minutes.unsigned_abs();
    format!("{sign}{:02}:{:02}", absolute / 60, absolute % 60)
}

fn timestamp(date: &CivilDateTime) -> String {
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02} {}",
        date.year,
        date.month,
        date.day,
        date.hour,
        date.minute,
        date.second,
        offset(date.utc_offset_minutes)
    )
}

pub(crate) fn calendar(
    output: &mut impl Write,
    input: &CalendarRequest,
    result: &CalendarResult,
) -> io::Result<()> {
    writeln!(
        output,
        "公历 {:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC{}",
        input.year,
        input.month,
        input.day,
        input.hour,
        input.minute,
        input.second,
        offset(input.utc_offset_minutes)
    )?;
    let boundary = match input.day_boundary {
        DayBoundary::ZiStart => "子初 23:00 换日",
        DayBoundary::Midnight => "子正 00:00 换日",
    };
    writeln!(output, "民用时间 · {boundary} · {}", result.lunar_date.name)?;
    let pillars = result.four_pillars;
    writeln!(
        output,
        "八字 年 {}  月 {}  日 {}  时 {}",
        pillars.year, pillars.month, pillars.day, pillars.hour
    )?;
    writeln!(
        output,
        "节气 {} {} → {} {}",
        result.solar_term.name,
        timestamp(&result.solar_term.start),
        result.next_solar_term.name,
        timestamp(&result.next_solar_term.start)
    )
}

fn palace_lines(palace: &Palace) -> [String; 7] {
    let trigram = palace.trigram.map_or("中", |item| item.name());
    let stars = palace
        .stars
        .iter()
        .map(|item| item.name())
        .collect::<Vec<_>>()
        .join("+");
    let heaven = palace
        .heaven_stems
        .iter()
        .map(|item| {
            if item.is_center_hosted {
                format!("{}(寄)", item.stem)
            } else {
                item.stem.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    let earth = palace.hosted_earth_stem.map_or_else(
        || palace.earth_stem.to_string(),
        |hosted| format!("{} {}(寄)", palace.earth_stem, hosted),
    );
    let voids: String = palace
        .void_branches
        .iter()
        .map(|branch| branch.name())
        .collect();
    let voids = if voids.is_empty() { "—" } else { &voids };
    let horse = if palace.is_horse { " · 驿马" } else { "" };
    [
        format!(
            "{trigram}{}宫 {} / {}",
            palace.number, palace.direction, palace.element
        ),
        format!("八神 {}", palace.deity.map_or("—", |item| item.name())),
        format!("九星 {}", if stars.is_empty() { "—" } else { &stars }),
        format!("八门 {}", palace.door.map_or("—", |item| item.name())),
        format!("天盘 {}", if heaven.is_empty() { "—" } else { &heaven }),
        format!("地盘 {earth}"),
        format!("空亡 {voids}{horse}"),
    ]
}

fn border(
    output: &mut impl Write,
    width: usize,
    left: char,
    middle: char,
    right: char,
) -> io::Result<()> {
    let line = "─".repeat(width + 2);
    writeln!(output, "{left}{line}{middle}{line}{middle}{line}{right}")
}

pub(crate) fn chart(output: &mut impl Write, chart: &Chart) -> io::Result<()> {
    calendar(output, &chart.input, &chart.calendar)?;
    writeln!(
        output,
        "{} · {}{}局 · {} · 符头 {}",
        chart.method, chart.dun, chart.ju, chart.yuan, chart.yuan_head
    )?;
    writeln!(
        output,
        "旬首 {}{} · 值符 {} 落{}宫 · 值使 {} 落{}宫",
        chart.xun.head,
        chart.xun.hidden_stem,
        chart.leaders.star,
        chart.leaders.star_palace,
        chart.leaders.door,
        chart.leaders.door_palace
    )?;
    writeln!(
        output,
        "空亡 年{}{} 月{}{} 日{}{} 时{}{} · 驿马 {} / {}宫",
        chart.pillar_voids.year[0],
        chart.pillar_voids.year[1],
        chart.pillar_voids.month[0],
        chart.pillar_voids.month[1],
        chart.pillar_voids.day[0],
        chart.pillar_voids.day[1],
        chart.pillar_voids.hour[0],
        chart.pillar_voids.hour[1],
        chart.horse.branch,
        chart.horse.palace
    )?;
    writeln!(
        output,
        "中五寄坤二 · 天禽随天芮 · 宫内空亡与驿马按时柱 · 上南下北"
    )?;

    let cells = chart.palaces.each_ref().map(palace_lines);
    let width = cells
        .iter()
        .flatten()
        .map(|line| line.width())
        .max()
        .unwrap_or(20)
        .max(20);
    border(output, width, '┌', '┬', '┐')?;
    for (row_index, row) in [[3, 8, 1], [2, 4, 6], [7, 0, 5]].iter().enumerate() {
        for ((left, middle), right) in cells[row[0]].iter().zip(&cells[row[1]]).zip(&cells[row[2]])
        {
            for text in [left, middle, right] {
                write!(output, "│ {text}{} ", " ".repeat(width - text.width()))?;
            }
            writeln!(output, "│")?;
        }
        if row_index < 2 {
            border(output, width, '├', '┼', '┤')?;
        }
    }
    border(output, width, '└', '┴', '┘')
}
