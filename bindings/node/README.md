# qimen-rs Node.js 绑定

基于 napi-rs / Node-API 8，支持 Node.js 20+。提供 CommonJS、ESM 和 TypeScript 声明。
排盘算法、日期校验、默认值和 JSON schema 均来自 Rust 核心。

```typescript
import { calculate } from '@spensercai/qimen-rs';

const chart = calculate({
  year: 2024, month: 2, day: 10, hour: 12,
  utc_offset_minutes: 480,
  day_boundary: 'zi_start',
});
console.log(chart.palaces);
```

CommonJS 使用 `const { calculate } = require('@spensercai/qimen-rs')`。
`calculateJson(requestJson)` 输入和输出均为 JSON 字符串。
请求字段使用 snake_case，以保障所有语言同一份 schema。
日期参数是当地民用时间，不是 JavaScript `Date`；未提供 UTC 偏移时固定采用 +08:00。
无效日期或未知字段抛出 `Error`，`error.code === 'InvalidArg'`。计算为同步调用；
大量批处理可在 Node Worker 中运行。

源码构建：

```bash
cd bindings/node
npm install
npm run build
npm test
```

已配置发行目标：Linux x64/arm64 GNU、macOS x64/arm64、Windows x64 MSVC。
Linux x64 预构建包要求 glibc >= 2.35，arm64 要求 glibc >= 2.39；其他环境可使用
浏览器 Wasm 包或源码构建。每个平台二进制作为 npm optional dependency 安装。
尚未发布的版本请使用源码或 CI 产物。

发布收集流程（在 CI 已完成各平台构建和测试后）：

```bash
npm run create-npm-dirs
npm run artifacts
npm run prepublish:native
npm publish --access public --ignore-scripts
```

`artifacts/` 内放置 CI 下载的 `qimen.<platform>.node`，同时恢复生成的 `native.cjs`。
`prepublish:native` 会先检查每个目标文件，再发布平台分包并更新主包的 optionalDependencies。
它只在显式发行任务中调用，不绑定 `npm install` 或 `npm pack` 的生命周期。
