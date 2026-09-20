use clap::{Args, Parser, Subcommand, ValueEnum};
use qimen_calendar::{CalendarRequest, DayBoundary};
use qimen_core::{ExtensionOptions, TombRule};

#[derive(Debug, Parser)]
#[command(
    name = "qimen",
    version,
    about = "时家拆补转盘与八字 / Qimen Dunjia and Four Pillars",
    after_help = "示例 / Examples:\n  qimen paipan --year 2024 --month 2 --day 10 --hour 12\n  \
                  qimen bazi --year 2024 --month 2 --day 10 --hour 12 --json\n\n\
                  qimen paipan --year 2026 --month 9 --day 18 --hour 18 --extensions all\n\n\
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
    Paipan(ChartInput),
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
pub(crate) struct ChartInput {
    #[command(flatten)]
    pub(crate) calendar: Input,
    /// 开启扩展（逗号分隔），默认关闭 / Enable optional annotations (comma-separated)
    #[arg(long, value_enum, value_delimiter = ',')]
    extensions: Vec<Extension>,
    /// 入墓约定（仅在开启 tombs 或 all 时生效）/ Tomb convention; requires tombs or all
    #[arg(long, value_enum, requires = "extensions")]
    tomb_rule: Option<TombConvention>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum TombConvention {
    /// 十二长生入墓，阳顺阴逆土随火（默认，乙墓戌）/ Twelve-stage tombs (default)
    GrowthStageFireEarth,
    /// 传统三奇入墓：乙未、丙戌、丁丑；六仪不适用 / Three wonders only
    TraditionalThreeWonders,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Extension {
    /// 暗干：时干从值使宫飞布，伏吟改从中五起 / Duty-door flight with center fallback
    HiddenStems,
    /// 星用传统九星旺衰，门干用五行旺衰；兼列落宫和月令 / Palace and solar-month strengths
    Strength,
    /// 十二长生：阳顺阴逆，土随火 / Yang forward, yin reverse; earth follows fire
    GrowthStages,
    /// 六仪击刑：戊卯、己未、庚寅、辛午、壬辰、癸巳 / Six-instrument punishment
    Punishments,
    /// 入墓：按十二长生，乙墓戌 / Twelve-stage tombs, including Yi at Xu
    Tombs,
    /// 日马：按所选换日规则的日支三合 / Day-pillar horse
    DayHorse,
    /// 门迫：门五行克落宫五行 / Door element controls palace element
    DoorPressure,
    /// 按上述约定开启所有已实现扩展 / Enable all listed annotations and conventions
    All,
}

impl ChartInput {
    pub(crate) fn extension_options(&self) -> Result<ExtensionOptions, clap::Error> {
        let defaults = ExtensionOptions::all();
        let mut options = ExtensionOptions::default();
        for extension in &self.extensions {
            match extension {
                Extension::HiddenStems => options.hidden_stems = defaults.hidden_stems,
                Extension::Strength => options.strength = defaults.strength,
                Extension::GrowthStages => options.growth_stages = defaults.growth_stages,
                Extension::Punishments => options.punishments = defaults.punishments,
                Extension::Tombs => options.tombs = defaults.tombs,
                Extension::DayHorse => options.day_horse = defaults.day_horse,
                Extension::DoorPressure => options.door_pressure = defaults.door_pressure,
                Extension::All => options = defaults.clone(),
            }
        }
        if let Some(rule) = self.tomb_rule {
            if options.tombs.is_none() {
                return Err(clap::Error::raw(
                    clap::error::ErrorKind::ArgumentConflict,
                    "--tomb-rule requires --extensions tombs or --extensions all",
                ));
            }
            options.tombs = Some(match rule {
                TombConvention::GrowthStageFireEarth => TombRule::GrowthStageFireEarth,
                TombConvention::TraditionalThreeWonders => TombRule::TraditionalThreeWonders,
            });
        }
        Ok(options)
    }
}

#[derive(Debug, Args)]
pub(crate) struct Input {
    /// 前推格里高利历年 / Proleptic Gregorian year (1–9999)
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
