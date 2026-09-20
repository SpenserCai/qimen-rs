# qimen-rs Python 绑定

[中文首页](../../README.md) · [English](../../README.en.md) · [扩展规则](../../docs/extensions.md)

Python 3.10+；使用 PyO3 和 maturin，计算期间释放 GIL。`calculate` 返回 Python 字典，
`calculate_json` 提供与 Rust 完全一致的 JSON 接口。CPython 常规 GIL 构建使用 abi3 wheel。

## 安装

```bash
python -m pip install qimen-rs
```

扩展到公元 1–9999 年的时间范围要求 0.2.0+；安装命令获取当前正式版。平台要求见[安装指南](../../docs/installation.md)。

## 快速开始

```python
from qimen_rs import calculate

chart = calculate({
    "year": 2024,
    "month": 2,
    "day": 10,
    "hour": 12,
    "utc_offset_minutes": 480,
    "day_boundary": "zi_start",
})
print(chart["palaces"])
```

输入是指定 UTC 偏移下的民用时间；不会读取机器时区。默认 UTC+08:00，23:00 换日。
`midnight` 可显式选择 00:00 换日。无效日期、未知参数等由核心统一拒绝为 `ValueError`；
非字典输入或不能编码为 JSON 的 Python 对象抛出 `TypeError`。返回字段见[结果 Schema](../../docs/schema/chart.schema.json)，时间与历法口径见[使用指南](../../docs/usage.md)。

## 可选扩展

可选标注默认关闭，通过 `extensions` 参数选择明确的计算规则：

```python
from qimen_rs import ExtensionOptions, calculate

options: ExtensionOptions = {
    "hidden_stems": "duty_door_hour_stem_with_center_fallback",
    "strength": "classical_stars_and_five_elements",
    "growth_stages": "yang_forward_yin_reverse_fire_earth",
    "punishments": "six_instrument_branches",
    "tombs": "growth_stage_fire_earth",
    "day_horse": "day_branch_three_harmony",
    "door_pressure": "door_controls_palace",
}
chart = calculate({"year": 2026, "month": 9, "day": 18, "hour": 18}, extensions=options)
print(chart["extensions"]["day_horse"]["horse"])
```

只填写需要的项目即可；也可将 `extensions` 放入请求字典，与 JSON 接口完全一致。
同时使用两种位置会报 `ValueError`，避免静默覆盖。空配置不生成 `extensions` 输出字段。
每项结果记录实际 `rule`，寄干保留来源和盘层，日马与基础盘的时马分别返回。
`tombs` 也可选择 `traditional_three_wonders`；该规则只判断三奇，不适用的六仪结果为
`None`，与“未入墓”的 `False` 有区别。两种入墓约定不能混用。
公共类型在 `qimen_rs.types` 中；JSON Schema 与软件包版本分别编号。
完整规则与适用范围见 [扩展约定](../../docs/extensions.md)。

## 从源码安装

在所需版本的仓库根目录执行，需 Rust 1.94+：

```bash
python -m venv .venv
```

激活虚拟环境，Linux / macOS 使用 `source .venv/bin/activate`，Windows PowerShell 使用 `.venv/Scripts/Activate.ps1`，然后运行：

```bash
python -m pip install ./bindings/python
```

安装后仍使用相同的 `from qimen_rs import calculate` 接口。0.2.0+ 支持公元 1–9999 年的前推格里高利历；历史日期与边界输出见[时间说明](../../docs/usage.md#历史日期与远期日期)。
