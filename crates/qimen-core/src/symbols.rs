//! Typed symbols shared by the chart model and the rotating-plate algorithm.

use std::fmt;

use serde::{Deserialize, Serialize};

macro_rules! symbols {
    ($(#[$meta:meta])* $name:ident { $($(#[$variant_meta:meta])* $variant:ident => $label:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $($(#[$variant_meta])* $variant),+
        }

        impl $name {
            /// Returns the traditional Chinese label.
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self { $(Self::$variant => $label),+ }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.name())
            }
        }
    };
}

symbols! {
    /// Yang or yin dun, determined by the current solar term.
    Dun {
        /// 阳遁, winter solstice through the start of summer solstice.
        Yang => "阳遁",
        /// 阴遁, summer solstice through the start of winter solstice.
        Yin => "阴遁",
    }
}

symbols! {
    /// The five-day yuan determined from the day's 甲/己 head.
    Yuan {
        /// 上元: the head's branch is 子午卯酉.
        Upper => "上元",
        /// 中元: the head's branch is 寅申巳亥.
        Middle => "中元",
        /// 下元: the head's branch is 辰戌丑未.
        Lower => "下元",
    }
}

symbols! {
    /// A supported chart construction method.
    Method {
        /// 时家拆补转盘: use the active solar term and five-day head independently.
        ShiJiaChaiBuZhuanPan => "时家拆补转盘",
    }
}

symbols! {
    /// Palace directions on the Later Heaven / Luo Shu arrangement.
    Direction {
        /// North.
        North => "北",
        /// Southwest.
        Southwest => "西南",
        /// East.
        East => "东",
        /// Southeast.
        Southeast => "东南",
        /// Center.
        Center => "中",
        /// Northwest.
        Northwest => "西北",
        /// West.
        West => "西",
        /// Northeast.
        Northeast => "东北",
        /// South.
        South => "南",
    }
}

symbols! {
    /// One of the eight Later Heaven trigrams.
    Trigram {
        /// 坎, palace one.
        Kan => "坎",
        /// 坤, palace two.
        Kun => "坤",
        /// 震, palace three.
        Zhen => "震",
        /// 巽, palace four.
        Xun => "巽",
        /// 乾, palace six.
        Qian => "乾",
        /// 兑, palace seven.
        Dui => "兑",
        /// 艮, palace eight.
        Gen => "艮",
        /// 离, palace nine.
        Li => "离",
    }
}

symbols! {
    /// One of the five traditional elements.
    Element {
        /// 木.
        Wood => "木",
        /// 火.
        Fire => "火",
        /// 土.
        Earth => "土",
        /// 金.
        Metal => "金",
        /// 水.
        Water => "水",
    }
}

symbols! {
    /// One of the nine stars; 天禽 is explicitly carried with 天芮.
    Star {
        /// 天蓬, home palace one.
        TianPeng => "天蓬",
        /// 天芮, home palace two.
        TianRui => "天芮",
        /// 天冲, home palace three.
        TianChong => "天冲",
        /// 天辅, home palace four.
        TianFu => "天辅",
        /// 天禽, home palace five and hosted with 天芮.
        TianQin => "天禽",
        /// 天心, home palace six.
        TianXin => "天心",
        /// 天柱, home palace seven.
        TianZhu => "天柱",
        /// 天任, home palace eight.
        TianRen => "天任",
        /// 天英, home palace nine.
        TianYing => "天英",
    }
}

symbols! {
    /// One of the eight doors.
    Door {
        /// 休门, home palace one.
        Xiu => "休门",
        /// 死门, home palace two.
        Si => "死门",
        /// 伤门, home palace three.
        Shang => "伤门",
        /// 杜门, home palace four.
        Du => "杜门",
        /// 开门, home palace six.
        Kai => "开门",
        /// 惊门, home palace seven.
        Jing => "惊门",
        /// 生门, home palace eight.
        Sheng => "生门",
        /// 景门, home palace nine.
        Scene => "景门",
    }
}

symbols! {
    /// One of the eight deities, starting from the duty star's current palace.
    Deity {
        /// 值符.
        ZhiFu => "值符",
        /// 螣蛇.
        TengShe => "螣蛇",
        /// 太阴.
        TaiYin => "太阴",
        /// 六合.
        LiuHe => "六合",
        /// 白虎.
        BaiHu => "白虎",
        /// 玄武.
        XuanWu => "玄武",
        /// 九地.
        JiuDi => "九地",
        /// 九天.
        JiuTian => "九天",
    }
}
