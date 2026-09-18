use std::fmt;

use serde::{Deserialize, Serialize};

/// The ten heavenly stems in their traditional order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Stem {
    /// 甲, yang wood.
    Jia,
    /// 乙, yin wood.
    Yi,
    /// 丙, yang fire.
    Bing,
    /// 丁, yin fire.
    Ding,
    /// 戊, yang earth.
    Wu,
    /// 己, yin earth.
    Ji,
    /// 庚, yang metal.
    Geng,
    /// 辛, yin metal.
    Xin,
    /// 壬, yang water.
    Ren,
    /// 癸, yin water.
    Gui,
}

impl Stem {
    /// Returns a stem, wrapping every ten positions from 甲 = 0.
    #[must_use]
    pub const fn from_index(index: u8) -> Self {
        [
            Self::Jia,
            Self::Yi,
            Self::Bing,
            Self::Ding,
            Self::Wu,
            Self::Ji,
            Self::Geng,
            Self::Xin,
            Self::Ren,
            Self::Gui,
        ][(index % 10) as usize]
    }

    /// Returns the zero-based traditional index.
    #[must_use]
    pub const fn index(self) -> u8 {
        self as u8
    }

    /// Returns the Chinese name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"][self as usize]
    }
}

impl fmt::Display for Stem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// The twelve earthly branches in their traditional order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Branch {
    /// 子, rat.
    Zi,
    /// 丑, ox.
    Chou,
    /// 寅, tiger.
    Yin,
    /// 卯, rabbit.
    Mao,
    /// 辰, dragon.
    Chen,
    /// 巳, snake.
    Si,
    /// 午, horse.
    Wu,
    /// 未, goat.
    Wei,
    /// 申, monkey.
    Shen,
    /// 酉, rooster.
    You,
    /// 戌, dog.
    Xu,
    /// 亥, pig.
    Hai,
}

impl Branch {
    /// Returns a branch, wrapping every twelve positions from 子 = 0.
    #[must_use]
    pub const fn from_index(index: u8) -> Self {
        [
            Self::Zi,
            Self::Chou,
            Self::Yin,
            Self::Mao,
            Self::Chen,
            Self::Si,
            Self::Wu,
            Self::Wei,
            Self::Shen,
            Self::You,
            Self::Xu,
            Self::Hai,
        ][(index % 12) as usize]
    }

    /// Returns the zero-based traditional index.
    #[must_use]
    pub const fn index(self) -> u8 {
        self as u8
    }

    /// Returns the Chinese name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        [
            "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
        ][self as usize]
    }
}

impl fmt::Display for Branch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A stem–branch pair within the sixty-position sexagenary cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Cycle {
    /// Zero-based cycle position: 甲子 = 0, 癸亥 = 59.
    pub index: u8,
    /// Heavenly stem.
    pub stem: Stem,
    /// Earthly branch.
    pub branch: Branch,
}

impl Cycle {
    /// Constructs a consistent cycle, wrapping every sixty positions.
    #[must_use]
    pub const fn from_index(index: u8) -> Self {
        let index = index % 60;
        Self {
            index,
            stem: Stem::from_index(index),
            branch: Branch::from_index(index),
        }
    }

    /// Returns the two-character Chinese name.
    #[must_use]
    pub fn name(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Cycle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.stem, self.branch)
    }
}
