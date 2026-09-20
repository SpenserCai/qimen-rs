# qimen-core

Rust 2024 时家拆补转盘奇门遁甲排盘库。输入公历年月日时及固定 UTC 偏移，
返回八字、节气、三元局数、旬首、值符值使，以及九宫完整地盘、天盘、九星、八门、八神、旬空与马星。

## 安装

```bash
cargo add qimen-core
```

本文对应 0.2.0 接口；公元 1–9999 年的日期范围要求 0.2.0+。安装命令获取当前正式版，也可通过本地源码依赖使用。

## 快速开始

```rust
fn main() -> Result<(), qimen_core::Error> {
    let request = qimen_core::ChartRequest::new(2026, 9, 18, 14);
    let chart = qimen_core::calculate(&request)?;
    println!("{}{}局", chart.dun, chart.ju);
    Ok(())
}
```

默认口径：UTC+08:00、23:00 子初换日、按当前节气拆补、中五固定寄坤、天禽随芮、
时旬空、时马。JSON 输出的 `input` 与 `conventions` 均记录实际口径。
只提供排盘计算，不加入吉凶解释。

`calculate_json` 为其他语言绑定提供相同的严格输入与版本化输出格式。
0.1.0 支持公历 1900–2100 年；0.2.0+ 支持公元 1–9999 年的前推格里高利历。相邻节气可能返回年份 0 或 10000，年初农历可能使用年份 0。支持范围表示可计算输入，不表示所有年代具备现代天文精度。
不自动应用真太阳时、地理时区或夏令时。

## 可选扩展

暗干、旺衰、十二长生、击刑、入墓、日马、门迫作为独立扩展，默认关闭。
通过初始化配置或每次调用参数开启，结果记录所选流派规则且不改变基础盘：

```rust
fn main() -> Result<(), qimen_core::Error> {
    let calculator = qimen_core::Calculator::new(qimen_core::ExtensionOptions::all());
    let chart = calculator.calculate(&qimen_core::ChartRequest::new(2026, 9, 18, 18))?;
    println!("{:?}", chart.extensions);
    Ok(())
}
```

`calculate_with_options` 支持单次配置；`CalculationRequest` 与 JSON 接口在
原有平铺公历字段之外接受 `extensions` 对象。各项结果包含规则名，未启用的
字段不序列化。方法适用范围与原典依据见[扩展规则](https://github.com/SpenserCai/qimen-rs/blob/main/docs/extensions.md)。
JSON Schema 版本与软件包版本分开；输入与结果结构见[使用指南](https://github.com/SpenserCai/qimen-rs/blob/main/docs/usage.md)。

历法转换由 `qimen-calendar` 提供；排盘是确定性的纯 Rust 计算，不读取网络、系统时间、环境变量或文件。
