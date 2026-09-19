# qimen-core

Rust 2024 时家拆补转盘奇门遁甲排盘库。输入公历年月日时及固定 UTC 偏移，
返回八字、节气、三元局数、旬首、值符值使，以及九宫完整地盘、天盘、九星、八门、八神、旬空与马星。

```rust
let request = qimen_core::ChartRequest::new(2026, 9, 18, 14);
let chart = qimen_core::calculate(&request)?;
println!("{}{}局", chart.dun, chart.ju);
# Ok::<(), qimen_core::Error>(())
```

默认口径：UTC+08:00、23:00 子初换日、按当前节气拆补、中五固定寄坤、天禽随芮、
时旬空、时马。JSON 输出的 `input` 与 `conventions` 均记录实际口径。
只提供排盘计算，不加入吉凶解释。

`calculate_json` 为其他语言绑定提供相同的严格输入与版本化输出格式。
支持公历 1900–2100 年；不自动应用真太阳时、地理时区或夏令时。

暗干、旺衰、十二长生、击刑、入墓、日马、门迫作为独立扩展，默认关闭。
通过初始化配置或每次调用参数开启，结果记录所选流派规则且不改变基础盘：

```rust
let calculator = qimen_core::Calculator::new(qimen_core::ExtensionOptions::all());
let chart = calculator.calculate(&qimen_core::ChartRequest::new(2026, 9, 18, 18))?;
assert!(chart.extensions.is_some());
# Ok::<(), qimen_core::Error>(())
```

`calculate_with_options` 支持单次配置；`CalculationRequest` 与 JSON 接口在
原有平铺公历字段之外接受 `extensions` 对象。各项结果包含规则名，未启用的
字段不序列化。方法适用范围与原典依据见仓库的 `docs/extensions.md`。
Schema 1.1 保留原请求格式，并增加可选扩展输出。

Calendar conversion is delegated to `qimen-calendar`; plate construction is deterministic,
pure Rust, with no network, system-clock, environment, or filesystem dependency.
