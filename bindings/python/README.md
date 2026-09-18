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
