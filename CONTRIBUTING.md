# 参与开发

[中文首页](README.md) · [测试指南](docs/validation.md) · [维护契约](AGENTS.md)

项目结构与长期约束见 [AGENTS.md](AGENTS.md)。默认开发工具链为 Rust stable，edition 2024；声明的最低 Rust 版本在根 `Cargo.toml`。公共行为测试放在各 crate 的 `tests/`，私有单元测试使用独立模块文件。

## 获取和验证

```sh
git clone https://github.com/SpenserCai/qimen-rs.git
cd qimen-rs
rustup component add rustfmt clippy
python scripts/check-quality.py
```

Python 脚本要求 Python 3.11+。质量命令将警告视为错误，依次运行：

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked --exclude qimen-python --exclude qimen-node --exclude qimen-wasm
cargo doc --workspace --no-deps --locked
```

脚本还执行 `python scripts/check-schemas.py` 检查请求与输出 Schema。脚本为编译和文档设置 `RUSTFLAGS=-D warnings` 与 `RUSTDOCFLAGS=-D warnings`；手动执行也应设置。`quality-diagnostics/` 保存各步日志，CI 即使失败也上传诊断。依赖变更后更新并提交锁文件；日常验证使用 `--locked`。

## 语言绑定

所有绑定委托同一 Rust 核心，不另写排盘算法。Rust check / Clippy 检查绑定源码，运行时测试验证实际打包和类型转换。

Python（CPython 3.10+，abi3）：

```sh
python -m venv .venv
# 激活当前平台的虚拟环境后：
python -m pip install maturin
maturin develop --manifest-path bindings/python/Cargo.toml
python -m unittest discover -s bindings/python/tests -v
```

Node.js：

```sh
cd bindings/node
npm ci --ignore-scripts
npm run build -- -- --locked
npm test
```

WebAssembly（从仓库根目录执行）：

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
wasm-pack build bindings/wasm --target nodejs --out-dir pkg-node --release -- --locked
node bindings/wasm/tests/smoke.cjs
wasm-pack build bindings/wasm --target web --out-dir pkg --scope spensercai --release -- --locked
```

不要用 `cargo test --workspace --all-features` 代替绑定测试：`extension-module` 刻意改变 Python 链接行为，普通 Rust 测试程序与 Python 扩展模块需要不同的链接方式。

## 提交计算差异

使用仓库“排盘准确性反馈” issue 模板，提供：

1. 精确公历输入、时区 / UTC 偏移和所有选项。
2. qimen-rs 版本与完整 JSON 结果。
3. 参考软件版本、流派、换日 / 真太阳时 / 寄宫设置及完整参考资料。
4. 具体差异，例如月柱、局数、值使落宫，而不只描述“盘不一样”。

先消除约定差异，再增加有独立来源的回归测试，最后修正实现。新增流派须说明其推导规则与验证范围；不能以支持选项存在代替完整实现。

## Pull request

PR 说明问题、结果行为、验证和兼容性影响。公共输出变化同步 Rust API、JSON schema、绑定类型、中文 / 英文文档。CI 的 `Quality gate` 是建议设置为分支保护必需的检查；合并前所有支持平台必须通过。

发布准备与令牌配置见 [发布指南](docs/releasing.md)。
