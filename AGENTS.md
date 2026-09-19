# qimen-rs 维护契约

本文件面向 Codex 与后续维护者，记录稳定约束及验证入口。任务进度写在 issue / PR；不要把聊天记录、逐次命令或流水账追加到本文件。

## 目标与边界

- 从明确时区的公历时间产生可解释、可复现的八字及奇门排盘；默认算法为**时家拆补转盘**。
- 历法事实、流派约定与术数解读分开。任何结果必须能说明时区、换日规则、定局方法及寄宫规则；不要把流派分歧静默“修正”为另一流派。
- 只承诺已实现并有参考例、边界测试的算法。新增流派应有独立实现与显式选择，不能用空分支、错误回退或近似结果冒充支持。
- 当前仓库的代码、测试、算法说明是接管入口；遇到用户提供的对照软件结果，先核对配置再改算法。

## 架构依赖

| 路径 | 职责 | 依赖方向 |
| --- | --- | --- |
| `crates/qimen-calendar` | 公历校验、节气、干支、时间与换日约定 | 不依赖奇门、应用或绑定 |
| `crates/qimen-core` | 定局、九宫、盘式、序列化公共结果 | 依赖 calendar |
| `apps/qimen-cli` | 命令行参数、中文排盘展示、JSON 输出 | 调用库，不复制算法 |
| `apps/qimen-mcp` | rmcp 工具与 stdio 协议适配 | 调用库，不复制算法 |
| `bindings/python` | PyO3 + maturin Python 包 | 调用 core |
| `bindings/node` | napi-rs Node-API 包 | 调用 core |
| `bindings/wasm` | wasm-bindgen 浏览器 / JS 包 | 调用 core |

核心库保持确定性：不读取系统当前时间、环境时区、网络或用户配置文件；调用者显式提供输入。库错误使用类型，CLI / MCP / 语言绑定负责映射。不要在 library 中退出进程或向 stdout 写日志。MCP stdout 仅可输出协议消息。

可选注解集中于 `qimen-core::extensions`，由 `ExtensionOptions` 显式启用，默认全关；初始化配置使用 `Calculator`，跨语言请求使用 `CalculationRequest`。历法层不接收奇门扩展配置，应用不复制注解公式。每项结果必须记录规则，不能把古典三奇入墓和阴阳顺逆长生墓混为一谈。基础盘不随注解开关改变；新盘式复用注解前应逐项核验适用条件，规则来源集中在 `docs/extensions.md`。

## Rust 与接口约束

- 统一 Rust **edition 2024**、workspace 版本和 lint；最低编译器见根 `Cargo.toml`，工具链见 `rust-toolchain.toml`。
- `unsafe_code` 默认禁止；FFI 生成宏需要例外时，只在最小绑定范围标注原因，不放宽整个 workspace。
- 所有受维护代码要求 **0 error / 0 warning**，包括 `clippy` 和 rustdoc。修复根因，不通过全局 `allow`、丢弃测试或取消 `-D warnings` 绕过质量门禁。
- 业务输入使用明确类型并校验；不要让无效索引、NaN、无效公历或反序列化输入进入算法。库 API 避免可触发 panic 的公开路径。
- 公共类型 / 字段写文档；结果 JSON 是语言与应用之间的契约。变更字段、枚举表示或默认值时同步 schema、TypeScript / Python 类型、例子与兼容性说明。
- 错误返回必须有清楚语义；不把“超出支持范围”替换成看似有效的盘。
- 使用已有模块边界，避免为单一调用增加通用框架、重复 DTO 或不必要 trait；也不要为减少文件而混合历法、排盘与界面。

## 测试与准确性

- 公共行为测试优先放各 crate 的 `tests/`；共享辅助代码放 `tests/common/mod.rs`，固定数据放 `tests/fixtures/`。
- Rust 允许内联单元测试，这不是语言规范错误；本项目约定避免在生产源文件堆放大量测试。确需访问私有实现时，使用独立测试模块文件，公开 API 用 doctest 演示。
- 算法改动必须带能区分正确与错误实现的回归例。不要只比较实现与它自己生成的结果；记录例子的来源、输入及流派参数。
- 历法边界覆盖交节前后、立春换年、节令换月、子时 / 午夜换日、闰日、日期范围端点、跨时区等价时刻。奇门覆盖阴阳遁、三元、九宫顺序、值符值使、旬首遁干、天禽寄宫、空亡及驿马等实际输出字段。
- 截图或第三方盘有分歧时保留最小输入和差异，先查计算约定，不能直接改 golden 让测试变绿。
- 绑定验证实际 Python wheel 导入、Node 原生加载与 WASM 执行；MCP 验证初始化、工具发现、调用、错误和协议版本协商。

## 必过命令

依赖：Rust stable + rustfmt / clippy、Python 3.12（绑定最低支持版本见 pyproject）、Node 22。首次安装 npm 依赖后提交 `package-lock.json`，Rust 依赖提交根 `Cargo.lock`。

```sh
python scripts/check-quality.py
python -m pip install maturin
maturin build --manifest-path bindings/python/Cargo.toml --release --locked --out dist
python -m pip install --no-index --find-links dist qimen-rs
python -m unittest discover -s bindings/python/tests -v
cd bindings/node
npm ci --ignore-scripts
npm run build -- -- --locked
npm test
```

`check-quality.py` 执行格式、全 workspace check、Clippy、Rust 测试和 rustdoc，并对警告报错。Python / Node / WASM 通过各自运行时测试，不使用 `cargo test --workspace --all-features`：PyO3 的 `extension-module` 供动态模块使用，不适用于普通 Rust 测试可执行文件链接。WASM 完整命令见 `CONTRIBUTING.md`。

CI 必须通过 Linux / Windows / macOS 与所有声明的二进制目标。不能把“已写 CI”报告成“CI 已通过”；无法验证时明确列出未运行项与原因。提交前检查 diff，保留其他协作者的修改；并行开发按目录分工，在共享 API 改动前同步接口。

## 发布与维护

默认 README 为中文，英文入口为 `README.en.md`。用户可见变化要同步双语说明；详细计算约定集中在 `docs/`，不要在应用与绑定内复制算法说明。

公开文档描述稳定功能、规则与使用方式，不记录开发对话、逐次测试结果或审查过程；测试来源和字段转录保存在测试目录，执行结果留在 CI / PR。架构图使用简短或分行标签，箭头明确表示依赖方向；中英文图示保持一致，修改后验证实际渲染。README 代码示例须可直接复制运行，不使用仅在 rustdoc 中隐藏的 `#` 行。

发布只走版本标签工作流，规则见 `docs/releasing.md`。GitHub 可下载产物与 crates.io / PyPI / npm 发布开关独立；凭证仅放 GitHub Secrets。没有用户发布授权时，只完成代码、验证和可审阅的发布配置，不实际发布不可覆盖的包版本。

依赖升级通过 Dependabot / PR，保留锁文件。升级历法库要复核参考盘和交节边界，升级 rmcp 要核实当前稳定版及真实协商行为；协议版本字符串不能代替握手测试。
