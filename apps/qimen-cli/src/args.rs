use clap::{Args, Parser, Subcommand, ValueEnum};
use qimen_calendar::{CalendarRequest, DayBoundary};

#[derive(Debug, Parser)]
#[command(
    name = "qimen",
    version,
    about = "时家拆补转盘与八字 / Qimen Dunjia and Four Pillars",
    after_help = "示例 / Examples:\n  qimen paipan --year 2024 --month 2 --day 10 --hour 12\n  \
                  qimen bazi --year 2024 --month 2 --day 10 --hour 12 --json\n\n\
                  默认使用 UTC+08:00 民用时间、23:00 换日；不自动换算真太阳时。\n\
                  Default: UTC+08:00 civil time, day rollover at 23:00; no true-solar correction."
)]
pub(crate) struct Options {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// 完整奇门九宫排盘 / Complete nine-palace Qimen chart
    #[command(alias = "chart")]
    Paipan(Input),
    /// 八字、农历与节气 / Four Pillars, lunar date and solar terms
    Bazi(Input),
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum Boundary {
    /// 子初换日 23:00 / Change the day pillar at 23:00
    ZiStart,
    /// 子正换日 00:00 / Change the day pillar at midnight
    Midnight,
}

#[derive(Debug, Args)]
pub(crate) struct Input {
    /// 公历年 / Gregorian year (1900–2100)
    #[arg(long)]
    year: i32,
    /// 公历月 / Gregorian month (1–12)
    #[arg(long)]
    month: u32,
    /// 公历日 / Gregorian day
    #[arg(long)]
    day: u32,
    /// 当地小时 / Local civil hour (0–23)
    #[arg(long)]
    hour: u32,
    /// 分钟 / Minute (0–59)
    #[arg(long, default_value_t = 0)]
    minute: u32,
    /// 秒 / Second (0–59)
    #[arg(long, default_value_t = 0)]
    second: u32,
    /// UTC 偏移分钟，须包含当地夏令时 / Fixed UTC offset including applicable DST
    #[arg(long, default_value_t = 480, allow_hyphen_values = true)]
    utc_offset_minutes: i32,
    /// 日柱换日规则 / Day-pillar rollover convention
    #[arg(long, value_enum, default_value = "zi-start")]
    day_boundary: Boundary,
    /// 输出标准 JSON / Emit the library's standard JSON result
    #[arg(long)]
    pub(crate) json: bool,
}

impl Input {
    pub(crate) fn request(&self) -> CalendarRequest {
        CalendarRequest {
            year: self.year,
            month: self.month,
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
            utc_offset_minutes: self.utc_offset_minutes,
            day_boundary: match self.day_boundary {
                Boundary::ZiStart => DayBoundary::ZiStart,
                Boundary::Midnight => DayBoundary::Midnight,
            },
        }
    }
}
