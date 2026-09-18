# 准确性验证与复现

以下记录本次开发期间实际执行并通过的本地验证。它们分别验证不同层面的正确性，不代表全部流派或整个日期范围已经获得独立天文认证。

## 跨实现历法差分

`scripts/compare-calendar.py` 使用独立 Python 环境中的 `lunar_python==1.4.8` 对照实际编译的 `qimen bazi --json`：

- 13 个代表年份，覆盖 1900–2100 的两端及世纪、闰年等年份。
- 每年全部 12 个月，各取 `00:00:00` 和 `23:59:59`，合计 312 个民用时刻。
- 分别计算 `zi_start` 与 `midnight`，共 **624 个对照用例全部通过**。
- 比较四柱、农历年月日与闰月、当前节气名称和起始时刻。实际对照的节气时刻差全部为 0 秒。

**局限：** lunar-python 和 tyme4rs 均出自 6tail，且具有寿星天文历算法共同来源。这是跨实现一致性和回归证据，不是彼此独立的天文精度证明。节气输出到秒是计算分辨率，不等于真实天文误差小于一秒。此脚本只比较 UTC+08:00；固定偏移的跨时区语义由 Rust 边界测试覆盖。

复现时，在仓库根目录运行：

```sh
python -m venv .venv-calendar
.venv-calendar/bin/python -m pip install lunar_python==1.4.8
cargo build -p qimen-cli --locked
.venv-calendar/bin/python scripts/compare-calendar.py --output calendar-check.json
```

Windows 将虚拟环境 Python 路径替换为 `.venv-calendar/Scripts/python.exe`。`--binary` 可指定另一份 CLI，例如 release 构建；省略 `--output` 时只在终端输出统计。脚本核对参考库版本，存在任何比较差异时退出码为 1，环境或报告写入错误为 2。该工具不接入主 CI，也不向生产库引入 Python 依赖。

## 历法与排盘边界

已通过的 Rust 回归包含：

- 年柱在立春瞬间改变，月柱在节令瞬间改变；24 节气交界前一秒、当秒与后一秒。
- 23 点与午夜换日、晚子时时柱连续、固定 UTC 偏移与跨公历日的交节时间。
- 闰日、农历闰月、1900/2100 输入端点，以及非法日期和整数极值的错误返回。
- 日柱换日约定传入拆补三元与定局；夏至在同一时辰内按交节瞬间切换阴阳遁。
- 阴阳十八局 × 六十时柱的 1080 种盘面结构不变量，以及带来源的外部参考盘。

独立公历—农历日期证据还包括香港天文台 [2026 年公历与农历日期对照表](https://www.hko.gov.hk/en/gts/time/calendar/pdf/files/2026e.pdf)：2 月 17 日为正月初一，9 月 25 日为八月十五。两项已写入历法测试。外部参考盘的来源及流派差异见 [算法来源](algorithm-sources.md)。

```sh
cargo test -p qimen-calendar --all-features --locked
cargo test -p qimen-core --all-features --locked
```

若与用户持有的软件不一致，先记录完整输入和时区、换日、定局、寄宫等配置，再逐项比较四柱、节气、局数与九宫；保留最小差异样本作为后续回归。
