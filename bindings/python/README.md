# qimen-rs Python 绑定

Python 3.10+；使用 PyO3 和 maturin，计算期间释放 GIL。`calculate` 返回 Python 字典，
`calculate_json` 提供与 Rust 完全一致的 JSON 接口。CPython 常规 GIL 构建使用 abi3 wheel。

```python
from qimen_rs import calculate

chart = calculate({
    "year": 2024, "month": 2, "day": 10, "hour": 12,
    "utc_offset_minutes": 480,
    "day_boundary": "zi_start",
})
print(chart["palaces"])
```

输入是指定 UTC 偏移下的民用时间；不会读取机器时区。默认 UTC+08:00，23:00 换日。
`midnight` 可显式选择 00:00 换日。无效日期、未知参数等由核心统一拒绝为 `ValueError`；
非字典输入或不能编码为 JSON 的 Python 对象抛出 `TypeError`。返回字段约定见仓库 schema 文档。

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
公共类型在 `qimen_rs.types` 中，返回 schema 为 `1.1`。
完整规则与适用范围见 [扩展约定](../../docs/extensions.md)。

Optional annotations are disabled by default. Pass a typed `extensions` mapping
either as a keyword argument or inside the request. Each annotation records its
selected convention; unsupported names or rules are rejected by Rust. Omitted
annotations were not requested, while a null tomb result means the selected rule
does not apply. Existing calls without options retain the base chart fields.

在仓库根目录开发和测试：

```bash
python -m venv .venv
# 激活虚拟环境后
python -m pip install 'maturin>=1.9,<2'
maturin develop --manifest-path bindings/python/Cargo.toml
python -m unittest discover -s bindings/python/tests -v
```

构建发行文件：

```bash
maturin build --release --manifest-path bindings/python/Cargo.toml --out dist
maturin sdist --manifest-path bindings/python/Cargo.toml --out dist
```

尚未发布到 PyPI 的版本请从源码构建或安装 CI wheel。算法仅在 `qimen-core` 中实现；
Python 包负责类型转换和错误映射，不维护独立排盘规则。
