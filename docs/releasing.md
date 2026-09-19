# 发布指南

[中文首页](../README.md) · [参与开发](../CONTRIBUTING.md) · [测试指南](validation.md)

本仓库采用统一稳定版本：Rust workspace、Python 分发和 Node / WASM npm 包保持相同版本。发布由 `vX.Y.Z` 标签触发，手动重跑也必须选择版本标签；分支上的手动触发会被明确拒绝。

## 产物和平台

| 渠道 | 产物 | 默认行为 |
| --- | --- | --- |
| GitHub Release | 同一压缩包中的 `qimen`、`qimen-mcp`、双语 README、LICENSE、SHA-256 | 标签验证及完整 CI 通过后生成 |
| crates.io | `qimen-calendar` → `qimen-core` → `qimen-cli` → `qimen-mcp` | 需显式启用 |
| PyPI | `qimen-rs`；导入名 `qimen_rs`；abi3 wheel + sdist | 需显式启用 |
| npm | `@spensercai/qimen-rs` + 五个平台原生包 | 需显式启用 |
| npm | `@spensercai/qimen-wasm` 浏览器 WASM 包 | 需显式启用 |

原生目标为 Linux x64 / arm64、macOS x64 / arm64、Windows x64。每个 Python wheel 和 Node 原生库在对应架构运行测试；WASM 在 Node 中执行后另行打包浏览器产物。

Python Linux wheel 使用 manylinux_2_28。应用与 Node GNU 二进制使用 Ubuntu 22.04（x64）/ 24.04（arm64）构建，分别要求 glibc 2.35+ / 2.39+；不是 musl / Alpine 包。macOS 原生包分别在 macOS 15 Intel / macOS 14 Apple Silicon 验证。其他目标不属于预构建产物的支持范围。

## 一次性 GitHub 配置

发布开关与凭据分别配置：

1. 在 **Settings → Secrets and variables → Actions → Variables** 添加下表的 Repository variables，值填写小写 `true`。
2. 在 **Settings → Environments** 创建对应环境，并添加下表的 Environment secrets。也可使用同名 Repository secrets。
3. 如果环境设置了部署分支或标签限制，需要允许发布标签 `v*`。

| 渠道 | Repository variable | Environment | Secret |
| --- | --- | --- | --- |
| crates.io | `PUBLISH_CRATES=true` | `crates-io` | `CARGO_REGISTRY_TOKEN` |
| PyPI | `PUBLISH_PYPI=true` | `pypi` | `PYPI_API_TOKEN` |
| Node.js npm | `PUBLISH_NPM=true` | `npm` | `NPM_TOKEN` |
| WebAssembly npm | `PUBLISH_WASM=true` | `npm` | 共用 `NPM_TOKEN` |

发布开关必须使用 Repository variables，供任务启动前的条件判断读取。未设置的渠道保持关闭；启用后需要提供有效凭据。GitHub Release 使用工作流内置的 `GITHUB_TOKEN`，无需额外创建个人访问令牌。

### 注册表权限

- **crates.io**：账号验证邮箱后创建 API token，授予四个 Rust 包的创建或发布权限。参见 [Cargo 发布说明](https://doc.rust-lang.org/cargo/reference/publishing.html)。
- **PyPI**：API token 需要具备 `qimen-rs` 的上传权限；已存在的项目优先使用项目范围令牌。保存完整值，包括 `pypi-` 前缀。参见 [PyPI token 说明](https://pypi.org/help/#apitoken)。
- **npm**：确认 `@spensercai` scope 的归属。Granular access token 的 Packages and scopes 权限选择 **Read and write (publish and stage)**，并启用 **Bypass two-factor authentication**。授权范围覆盖主包、五个平台分包及可选的 WASM 包；仅授予组织管理权限不足以发布包。参见 [npm token 说明](https://docs.npmjs.com/creating-and-viewing-access-tokens/)。

发布前确认包名可用或账号已拥有对应包的发布权限。更改包名或 scope 时，同步修改 manifest、loader 生成配置、工作流和文档。npm 主包及平台包均公开发布，工作流附带 provenance。

当前工作流使用 token 认证。npm 和 PyPI 可进一步采用 OIDC Trusted Publishing；迁移需要同时修改工作流并在注册表配置可信发布者，不能直接删除现有 secrets。配置依据见 [npm](https://docs.npmjs.com/trusted-publishers/) 和 [PyPI](https://docs.pypi.org/trusted-publishers/using-a-publisher/)。

建议分支保护要求 `Quality gate`，限制 release 标签创建权限。依赖和 GitHub Actions 由 Dependabot 定期提出更新，仍须完整 CI 验证。

## 发布一个版本

1. 更新根 workspace `version`、workspace / 子包中本地依赖的版本要求，以及 `bindings/node/package.json` 的版本；Python / WASM 从 Cargo 获取版本。更新锁文件和变更说明。
2. 在 PR 上完成全部质量门禁与参考盘验证。`python scripts/release.py validate vX.Y.Z` 校验标签、npm 版本与已跟踪的 Cargo.lock。
3. 合并已验证的提交，创建并推送准确指向该提交的 `vX.Y.Z` 标签。
4. Release 对标签的同一提交重新运行 CI：格式、编译、Clippy、测试、rustdoc，以及五平台 Python / Node 与 WASM 实际运行测试。全部通过才打包和发布。
5. GitHub Release 完成后，按开关并行发布注册表。Rust crate 在作业内按依赖顺序发布；Node 先验证五个平台产物齐全，再发布平台包，最后发布主包。
6. 从干净环境安装发布版本，核对版本号、参考输入结果与 GitHub SHA-256 校验文件。

注册表发布不是跨渠道事务。一个渠道失败不意味着其他渠道回滚。初次试跑可以保持所有注册表变量关闭，只检查 GitHub 可下载包和 Actions artifacts；不需要事先配置 key。

## 故障恢复

- **质量失败**：下载 `diagnostics-*` 获取完整日志、Cargo.lock 和可用的 rustfmt.patch；修复源代码并重跑。不能取消警告门禁来修复构建。
- **平台产物缺失**：不要直接执行 napi 发布。发布流程通过 `check-artifacts.cjs` 强制检查每一个平台；`napi pre-publish` 自己可能只发警告并跳过缺失目标。
- **npm 局部发布**：先用 `npm view <name>@<version> version` 检查主包和所有平台包。只复用同一标签、同一已验证产物；已发布版本不可覆盖。napi 可识别已存在的平台版本，主包已成功发布则不要再次盲目运行整个任务。
- **crates.io 局部发布**：按顺序确认已发布 crate；发布失败任务需要从尚未发布的 crate 继续，不能改动已发布版本内容。默认脚本不会跳过重复版本以掩盖状态分歧。
- **PyPI 局部发布**：确认已存在的 wheel / sdist 和未上传文件，仅补齐缺失的原始产物；不要重编译不同内容覆盖同版本。
- **已发布版本有算法错误**：保留问题记录、增加回归测试并发布新 patch 版本；必要时按对应注册表流程标记有问题的版本。

不要用 `npm publish --dry-run` 模拟执行带有发布 lifecycle 的脚本。仅检查 tarball 使用 `npm pack --dry-run --ignore-scripts`；检查 napi 配置使用 `napi pre-publish --dry-run --no-gh-release`，这两者不替代实际多平台测试。
