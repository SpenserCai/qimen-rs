# 安装与平台支持

[中文首页](../README.md) · [English](../README.en.md) · [使用指南](usage.md)

Rust、Python、Node.js 和 WebAssembly 使用相同项目版本。下列命令安装各注册表当前的最新正式版；Streamable HTTP 与公元 1–9999 年的日期范围要求 **0.2.0+**，请确认已安装版本。也可使用对应版本源码构建。

## CLI 与 MCP

从 [GitHub Releases](https://github.com/SpenserCai/qimen-rs/releases) 下载对应系统的归档文件。每个应用压缩包包含 `qimen`、`qimen-mcp`、README 和许可证，并提供 SHA-256 校验文件。解压后直接运行或将可执行文件所在目录加入 `PATH`：

```bash
qimen --version
qimen-mcp --version
qimen paipan --year 2026 --month 9 --day 18 --hour 15
```

Windows 中可在 PowerShell 使用 `./qimen.exe` 和 `./qimen-mcp.exe`。

已安装 Rust 时，也可通过 Cargo 安装：

```bash
cargo install qimen-cli --locked
cargo install qimen-mcp --locked
```

Cargo 安装需要 Rust 1.94+ 和当前平台的本地编译工具。

## Rust 库

```bash
cargo add qimen-core
```

只需要八字、农历和节气时，可单独使用历法库：

```bash
cargo add qimen-calendar
```

Rust API 见 [qimen-core](https://docs.rs/qimen-core/latest/qimen_core/) 和 [qimen-calendar](https://docs.rs/qimen-calendar/latest/qimen_calendar/)。库不需要网络或异步运行时。

## 语言绑定

| 平台 | 安装命令 | 使用说明 |
| --- | --- | --- |
| Python 3.10+ | `python -m pip install qimen-rs` | [Python](../bindings/python/README.md) |
| Node.js 20+ | `npm install @spensercai/qimen-rs` | [Node.js / TypeScript](../bindings/node/README.md) |
| 浏览器 WASM | `npm install @spensercai/qimen-wasm` | [WebAssembly](../bindings/wasm/README.md) |

Python 使用 CPython 常规 GIL 构建的 abi3 wheel。Node.js 主包会自动安装当前平台对应的原生可选依赖；安装时请保留 optional dependencies。

## 预构建平台

| 平台 | 应用与 Node.js 原生包 | Python wheel |
| --- | --- | --- |
| Linux x64 | GNU / glibc 2.35+ | manylinux_2_28，glibc 2.28+ |
| Linux arm64 | GNU / glibc 2.39+ | manylinux_2_28，glibc 2.28+ |
| macOS x64 | 提供 Intel 原生包 | 提供 Intel wheel |
| macOS arm64 | 提供 Apple Silicon 原生包 | 提供 Apple Silicon wheel |
| Windows x64 | MSVC 原生包 | 提供 x64 wheel |

Linux 原生包适用于 glibc 系统；Alpine / musl 不在预构建范围内，可选择源码构建或 WASM。macOS 原生包在 macOS 15 Intel 和 macOS 14 Apple Silicon 上运行；更早系统请依据包内目标信息选择源码构建。WASM 不受上述本地 CPU 架构列表限制，需支持 WebAssembly 的 JavaScript 宿主。

## 从源码使用

使用 Rust 1.94+。在所需版本或分支的源码目录运行：

```bash
cargo build --release --locked -p qimen-cli -p qimen-mcp
./target/release/qimen --version
./target/release/qimen paipan --year 2026 --month 9 --day 18 --hour 15
```

Windows 中将可执行文件名替换为 `qimen.exe` / `qimen-mcp.exe`。也可安装到 Cargo 的可执行文件目录：

```bash
cargo install --path apps/qimen-cli --locked
cargo install --path apps/qimen-mcp --locked
```

在自己的 Rust 项目中使用源码库时，在 `Cargo.toml` 中将路径指向实际源码：

```toml
[dependencies]
qimen-core = { path = "../qimen-rs/crates/qimen-core" }
```

Python、Node.js 和 WASM 的源码使用步骤分别见对应绑定文档。请使用同一项目版本的绑定、类型声明和本地库。

## 常见安装问题

- **Node.js 无法加载原生模块**：确认 Node.js 20+、操作系统和 CPU 架构受支持，并未使用 `--omit=optional`；从其他系统复制来的 `node_modules` 需要在目标系统重新安装。
- **Linux 提示 `GLIBC_x.y not found`**：当前系统低于该原生包的 glibc 要求，可使用源码构建或 WASM。
- **pip 开始编译而不是下载 wheel**：当前解释器或平台没有匹配 wheel。源码安装需要 Rust 1.94+ 与本地编译工具。
- **浏览器无法加载 `.wasm`**：通过 HTTP(S) 静态服务器或支持 WASM 的打包器加载，并检查 `.wasm` 的资源路径与 `application/wasm` MIME 类型。
