# 测试与验证

[中文首页](../README.md) · [基础盘算法](algorithm-sources.md) · [参与开发](../CONTRIBUTING.md)

本页说明测试分层、覆盖范围和复现方式。测试执行结果由对应提交的 CI 记录提供；参考输入、字段转录和流派差异保存在[测试资料](../crates/qimen-core/tests/fixtures/README.md)。

## 测试分层

| 层次 | 验证内容 | 入口 |
| --- | --- | --- |
| 历法 | 日期校验、四柱、节气边界、日界与固定时区偏移 | `crates/qimen-calendar/tests/` |
| 基础盘 | 阴阳遁、三元局数、星门神、旬首、寄宫、空亡与时马 | `crates/qimen-core/tests/chart.rs` |
| 可选扩展 | 暗干、旺衰、长生、击刑、入墓、日马、门迫与独立开关 | `crates/qimen-core/tests/extensions.rs` |
| 输入契约 | JSON 类型、枚举、重复键、未知字段及兼容性 | `crates/qimen-core/tests/request.rs` |
| 应用 | CLI 参数、文本与 JSON 输出；MCP 生命周期和工具调用 | `apps/qimen-cli/tests/`、`apps/qimen-mcp/tests/` |
| 语言绑定 | Python wheel、Node 原生模块、TypeScript 类型及 WASM 运行行为 | `bindings/*/tests/` |

公开行为测试放在各 crate 的 `tests/` 目录。需要访问私有实现时，使用独立单元测试模块；公共 API 用 doctest 提供可运行示例。

## 质量门禁

在仓库根目录运行：

```sh
python scripts/check-quality.py
```

该脚本依次检查格式、全 workspace 编译、Clippy、Rust 测试、rustdoc 和 Schema 一致性，并将编译与文档警告视为错误。运行日志保存在 `quality-diagnostics/`。最低 Rust 版本以根 `Cargo.toml` 的 `rust-version` 为准；当前 MSRV 检查命令为：

```sh
cargo +1.94.0 check --workspace --all-targets --locked
```

按模块执行算法测试：

```sh
cargo test -p qimen-calendar --all-features --locked
cargo test -p qimen-core --all-features --locked
cargo test -p qimen-core --features schema --locked \
  --test extensions --test request
```

Python、Node.js 与 WebAssembly 需要在各自运行时执行绑定测试，完整命令见[参与开发](../CONTRIBUTING.md#语言绑定)。CI 在 Linux、macOS、Windows 上执行质量检查；二进制目标和打包约定见[发布指南](releasing.md#产物和平台)。

## 算法覆盖范围

### 历法与基础盘

- 立春换年、节令换月，以及二十四节气交接前一秒、当秒和后一秒。
- 子初与午夜换日、晚子时时柱连续性、固定 UTC 偏移和跨公历日的交节时刻。
- 闰日、农历闰月、1900–2100 输入端点，以及非法日期与整数极值的错误返回。
- 日界规则向拆补三元的传递，以及夏至在同一时辰内切换阴阳遁。
- 阴阳十八局与六十时柱的 1080 种盘面组合，验证盘层完整性、宫序和寄宫关系。
- 阴遁与阳遁的外部参考盘，按明确的字段和流派约定进行比较。

独立公历与农历日期参考包括香港天文台的 [2026 年公历与农历日期对照表](https://www.hko.gov.hk/en/gts/time/calendar/pdf/files/2026e.pdf)。结构不变量验证盘面的内部约束；外部参考案例验证指定输入及字段，两类测试互为补充。

### 可选扩展与接口

- 甲时遁干替代、数字九宫顺逆飞布、重干起中宫，以及本位干和寄干的区别。
- 九个可见干与十二地支的 108 个长生组合，六仪击刑正反例和两种入墓规则的适用范围。
- 九星与门干使用不同旺衰关系表，月令取节气月柱，日马跟随所选日界。
- 扩展全关、单开与全开，保持基础盘字段不受注记选项影响。
- 区分未启用、规则不适用和已启用但未命中，保留盘层、源宫与寄干身份。
- 拒绝重复 JSON 键、未知字段、错误枚举及不符合 Schema 的位置数组或对象。
- MCP 当前及兼容协议版本的初始化、工具发现、调用、结构化结果与错误响应。

## 跨实现历法差分

`scripts/compare-calendar.py` 使用 `lunar_python==1.4.8` 对照实际编译的 `qimen bazi --json`：

| 项目 | 范围 |
| --- | --- |
| 年份 | 13 个代表年份，包含 1900–2100 两端、世纪和闰年 |
| 采样 | 每年 12 个月，各取 `00:00:00` 和 `23:59:59` |
| 日界 | `zi_start` 与 `midnight`，共 624 组对照 |
| 比较字段 | 四柱、农历年月日、闰月、当前节气名称及交节时刻 |
| 时间口径 | UTC+08:00 |

在仓库根目录运行：

```sh
python -m venv .venv-calendar
.venv-calendar/bin/python -m pip install lunar_python==1.4.8
cargo build -p qimen-cli --locked
.venv-calendar/bin/python scripts/compare-calendar.py \
  --output calendar-check.json
```

Windows 使用 `.venv-calendar/Scripts/python.exe`。`--binary` 可指定另一份 CLI；省略 `--output` 时只输出终端统计。比较存在差异时退出码为 1，环境或报告写入错误为 2。该脚本独立于主 CI，不向生产库引入 Python 依赖。

lunar-python 和 tyme4rs 同出自 6tail，并具有寿星天文历算法的共同来源，因此差分用于跨实现一致性检查，不能作为独立的天文精度证明。节气时间精确到秒表示输出分辨率；临近交节的差异分析应同时记录依赖版本和交节结果。其他固定偏移的时间语义由 Rust 边界测试覆盖。

## 计算差异反馈

提交差异时，记录完整输入、UTC 偏移、日界、定局、寄宫和真太阳时设置，并保留参考资料的版本与字段范围。按四柱、节气、局数和逐宫盘层定位差异；确认原因后增加最小回归案例。提交格式见[参与开发](../CONTRIBUTING.md#提交计算差异)。
