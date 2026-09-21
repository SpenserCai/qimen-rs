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
| `apps/qimen-mcp` | rmcp 工具、stdio 与 Streamable HTTP 传输 | 调用库，不复制算法 |
| `apps/web` | Next.js / React 可视化排盘、交互与分享 | 仅调用 npm WASM 绑定，不复制算法 |
| `bindings/python` | PyO3 + maturin Python 包 | 调用 core |
| `bindings/node` | napi-rs Node-API 包 | 调用 core |
| `bindings/wasm` | wasm-bindgen 浏览器 / JS 包 | 调用 core |

历法对外统一前推格里高利历、公元 1–9999 年；内部隔离 tyme4rs 的混合历和年边界，不改变输入含义；农历月序按现行定气定朔规则统一前推/外推，天文求解仍由上游提供，不重新采用不连续的历史闰月表。跨边界节气可含年份 0 / 10000，农历可含年份 0。扩大计算范围不能扩大天文精度或历史历法复原承诺。

核心库保持确定性：不读取系统当前时间、环境时区、网络或用户配置文件；调用者显式提供输入。库错误使用类型，CLI / MCP / 语言绑定负责映射。不要在 library 中退出进程或向 stdout 写日志。stdio MCP stdout 仅可输出协议消息；HTTP 入口负责监听、Host / Origin 校验及退出，不在工具实现中重复传输逻辑。

可选注解集中于 `qimen-core::extensions`，由 `ExtensionOptions` 显式启用，默认全关；初始化配置使用 `Calculator`，跨语言请求使用 `CalculationRequest`。历法层不接收奇门扩展配置，应用不复制注解公式。每项结果必须记录规则，不能把古典三奇入墓和阴阳顺逆长生墓混为一谈。基础盘不随注解开关改变；新盘式复用注解前应逐项核验适用条件，规则来源集中在 `docs/extensions.md`。

Web 使用 Next.js App Router、React、Tailwind CSS 与 Motion；保持页面组合、展示组件、状态 hook、输入校验 / 分享及 WASM Worker 的职责分离。`apps/web` 是独立 npm 私有应用，明确排除在 Cargo workspace 之外。WASM 只在 Worker 中初始化，计算异常、加载失败和过期请求需显式处理；不得在组件中补写历法或排盘公式。TypeScript 类型复用绑定契约，所有外部输入先校验。

动效以 transform / opacity 为主，响应 `prefers-reduced-motion` 和用户开关；短暂罗盘动画不能长时间阻塞计算或掩盖错误。交互保留键盘与焦点语义、加载与错误状态，手机不能依赖 hover。日期、盘面与浏览器偏好分离；分享只携带显式请求，导出针对已完成结果。

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
- 绑定验证实际 Python wheel 导入、Node 原生加载与 WASM 执行；MCP 两种传输均验证工具发现、调用、错误与协议协商；HTTP 另覆盖会话、Host / Origin 和退出。

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

`check-quality.py` 执行格式、全 workspace check、Clippy、Rust 测试和 rustdoc，并对警告报错。Python / Node / WASM 通过各自运行时测试，不使用 `cargo test --workspace --all-features`：PyO3 的 `extension-module` 供动态模块使用，不适用于普通 Rust 测试可执行文件链接。WASM 在仓库根目录运行：

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
wasm-pack build bindings/wasm --target nodejs --out-dir pkg-node --release -- --locked
node bindings/wasm/tests/smoke.cjs
wasm-pack build bindings/wasm --target web --out-dir pkg --scope spensercai --release -- --locked
```

`python scripts/check-schemas.py` 检查结构一致性；生成入口为 `crates/qimen-core/examples/schema.rs`。最低编译器验证以根 `rust-version` 为准。`scripts/compare-calendar.py` 提供跨实现历法差分；lunar-python 与 tyme4rs 同源，不能把差分当作独立天文精度证明。详细测试来源保留在 `tests/fixtures/`，执行日志留在 CI / PR。

Web 的独立门禁在 `apps/web` 执行：

```sh
npm ci
npm run format:check
npm run lint
npm run typecheck
npm test
npm run build
npx playwright install --with-deps chromium webkit
npm run test:e2e
```

ESLint 使用 `--max-warnings=0`，TypeScript 开启严格检查。Vitest 验证输入、分享及状态边界；Playwright 对正式构建验证真实 WASM、桌面 / 移动端交互、减弱动效与失败恢复。测试放独立测试文件；不能用 mock 结果代替真实排盘链路。视觉修改至少检查桌面、窄屏与键盘路径。更新 npm 依赖时提交 `apps/web/package-lock.json`，生产资源由构建脚本从已锁定的 WASM 包准备，不提交生成的二进制。

CI 必须通过 Linux / Windows / macOS 与所有声明的二进制目标。不能把“已写 CI”报告成“CI 已通过”；无法验证时明确列出未运行项与原因。提交前检查 diff，保留其他协作者的修改；并行开发按目录分工，在共享 API 改动前同步接口。

## 发布与维护

默认 README 为中文，英文入口为 `README.en.md`。用户可见变化要同步双语说明；详细计算约定集中在 `docs/`，不要在应用与绑定内复制算法说明。

除本文件外，公开 Markdown 面向使用者：安装、API、参数、结果和计算约定；贡献流程、门禁与发布设置集中在本文件。公开文档不记录开发对话、逐次测试结果或审查过程；测试来源和字段转录保存在测试目录，执行结果留在 CI / PR。架构图使用简短或分行标签，箭头明确表示依赖方向；中英文图示保持一致，修改后验证实际渲染。README 代码示例须可直接复制运行，不使用仅在 rustdoc 中隐藏的 `#` 行。

统一版本源为根 `Cargo.toml` 的 `workspace.package.version`。运行 `python scripts/release.py set-version X.Y.Z` 同步本地依赖、Rust 锁文件、Node 与 Web manifests；Python / WASM / Rust 包继承项目版本。Web 必须保持 `private: true`，其已发布 WASM 依赖通过明确升级及锁文件更新，不自动指向尚未发布的版本。`python scripts/release.py validate` 是提交前与 CI 的一致性入口。软件包版本和 JSON Schema 版本独立。

Release 工作流在新版本合入 `main` 后自动运行，也可在 Actions 中对 `main` 手动启动，无需手工打标签。完整质量检查与构建后，工作流创建 `vX.Y.Z` 并继续发布同一提交；已有版本跳过，既有标签不得移动。版本推进与合并可能触发正式发布，因此没有用户发布授权时只完成代码、验证和 PR，不把未授权的新版本合入发布分支。兼容的手工标签也须通过版本和提交来源校验。

| 渠道 | Repository variable | Environment | Secret |
| --- | --- | --- | --- |
| crates.io | `PUBLISH_CRATES=true` | `crates-io` | `CARGO_REGISTRY_TOKEN` |
| PyPI | `PUBLISH_PYPI=true` | `pypi` | `PYPI_API_TOKEN` |
| Node.js npm | `PUBLISH_NPM=true` | `npm` | `NPM_TOKEN` |
| WASM npm | `PUBLISH_WASM=true` | `npm` | `NPM_TOKEN` |

开关放 Repository variables；凭证放 Environment secrets 或同名 Repository secrets。Environment 部署规则允许 `main`，若使用手工标签兼容路径也允许 `v*`。未开启的注册表跳过，GitHub Release 使用内置 `GITHUB_TOKEN`。npm token 需覆盖 scope 下主包、五个平台分包及 WASM 包，具有发布权限和 Bypass two-factor authentication；仅组织管理权限不足。令牌不得进入代码、日志或 PR。

注册表发布不是跨渠道事务。失败时先核对原运行和各包实际状态，再通过原运行的 `Re-run failed jobs` 恢复；仅补齐同一提交的原始产物，不移动标签或重编译不同内容覆盖既有版本。napi 发布前必须确认五个平台产物齐全；本地检查 tarball 使用 `npm pack --dry-run --ignore-scripts`，不要让发布 lifecycle 在检查中执行。正式发布完成后确认各注册表版本、安装与实际调用，不能只以 job 成功代替产物可用。

Web 使用独立 `.github/workflows/web.yml`：相关 PR 只运行质量与浏览器检查；相关 `main` 推送在检查成功后部署，也可对 `main` 手动运行。它不打标签、不触发 Rust / npm / PyPI 发布。缺少 Vercel 配置时部署步骤明确跳过并写 Actions 摘要，不能将跳过报告成上线成功。

| Web 配置 | GitHub Actions 位置 | 值来源 |
| --- | --- | --- |
| `VERCEL_TOKEN` | Repository secret | 有目标项目部署权限的 Vercel token |
| `VERCEL_ORG_ID` | Repository variable，或同名 secret | Vercel team / account ID，或本地 `.vercel/project.json` 的 `orgId` |
| `VERCEL_PROJECT_ID` | Repository variable，或同名 secret | Vercel 项目 ID，或 `.vercel/project.json` 的 `projectId` |

Vercel 项目设置为 **Next.js、Node.js 22.x、Root Directory `apps/web`**，框架、`npm ci` 安装、`npm run build` 构建及默认输出目录由 `apps/web/vercel.json` 固定，覆盖远端设置，不能按普通静态目录发布 `public/`。该 Root Directory 是远端 Project Setting，不写入 `vercel.json`。独立工作流从仓库根执行固定版本 CLI 的 `vercel pull`、`vercel build --prod` 与 `vercel deploy --prebuilt --prod`，拉取设置后先核对目录；不能在 `apps/web` 下重复应用同一路径。若同时使用 Vercel Git 集成，应关闭它对该项目的自动部署，避免绕过 Actions 门禁或重复发布；项目配置应由用户授权的部署操作管理。

应用运行不需要业务 API key。ChatGPT 连接 Vercel 仅授予当前连接器相应访问，不会自动在 GitHub 配置部署 token；首次部署须确认实际账号 / 项目可访问。不要将 token、拉取的 `.vercel` 或 `.env` 文件签入仓库。部署前验证 Build Output 的 Next.js 框架、首页、指南、浏览器资源及 WASM 文件；部署后工作流必须对正式域名 `https://qimen-rs.vercel.app` 执行真实 WASM 参考盘浏览器检查，域名变更时同步该检查地址。上线检查失败时保留浏览器诊断，不能只报告上传成功。必要时用 Vercel promote / rollback 恢复已验证产物，不在故障期间修改库版本。

Web 使用 Prettier 统一源码与配置格式，运行 `npm run format` 修正。升级 TypeScript / ESLint 时同时核对 Next.js、typescript-eslint 与规则插件的兼容范围，不通过关闭 lint 或忽略 peer dependencies 消除工具链错误。

依赖升级通过 Dependabot / PR，保留锁文件。升级历法库要复核参考盘和交节边界，升级 rmcp 要核实当前稳定版及真实协商行为；协议版本字符串不能代替握手测试。
