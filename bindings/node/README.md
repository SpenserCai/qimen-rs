# qimen-rs Node.js 绑定

[中文首页](../../README.md) · [English](../../README.en.md) · [扩展规则](../../docs/extensions.md)

基于 napi-rs / Node-API 8，支持 Node.js 20+。提供 CommonJS、ESM 和 TypeScript 声明。
排盘算法、日期校验、默认值和 JSON schema 均来自 Rust 核心。

## 安装

```bash
npm install @spensercai/qimen-rs
```

扩展到公元 1–9999 年的时间范围要求 0.2.0+；安装命令获取当前正式版。平台要求见[安装指南](../../docs/installation.md)。

## 快速开始

```typescript
import { calculate } from '@spensercai/qimen-rs';

const chart = calculate({
  year: 2024,
  month: 2,
  day: 10,
  hour: 12,
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

## 可选扩展

扩展标注默认关闭，使用请求的 `extensions` 参数选择需要的规则：

```typescript
import { calculate, type ExtensionOptions } from '@spensercai/qimen-rs';

const extensions: ExtensionOptions = {
  hidden_stems: 'duty_door_hour_stem_with_center_fallback',
  strength: 'classical_stars_and_five_elements',
  growth_stages: 'yang_forward_yin_reverse_fire_earth',
  punishments: 'six_instrument_branches',
  tombs: 'growth_stage_fire_earth',
  day_horse: 'day_branch_three_harmony',
  door_pressure: 'door_controls_palace',
};
const chart = calculate({ year: 2026, month: 9, day: 18, hour: 18, extensions });
console.log(chart.extensions?.day_horse?.horse);
```

可只传一项；空配置不生成 `extensions` 输出字段。每项结果均记录实际 `rule`，
寄干保留来源及盘层，日马独立于时马。入墓另支持 `traditional_three_wonders`，
只判断三奇；不适用的六仪返回 `null`，与“未入墓”的 `false` 有区别。
全部选项和结果有 TypeScript 类型，JSON Schema 版本与项目版本分开；字段见[结果 Schema](../../docs/schema/chart.schema.json)。
完整规则与适用范围见 [扩展约定](../../docs/extensions.md)。

## 平台支持

提供 Linux x64 / arm64 GNU、macOS x64 / arm64、Windows x64 MSVC 原生包。Linux x64 要求 glibc 2.35+，arm64 要求 glibc 2.39+。平台二进制随 npm optional dependencies 安装，请保留可选依赖。

计算是同步调用；大量批处理可在 Node Worker 中执行。浏览器或不支持的原生平台可使用 [WebAssembly 包](../wasm/README.md)。

## 从源码使用

需 Rust 1.94+、Node.js 20+。在所需版本的仓库根目录执行：

```bash
cd bindings/node
npm ci --ignore-scripts
npm run build -- -- --locked
node --input-type=module -e "import { calculate } from './index.mjs'; console.log(calculate({year: 2026, month: 9, day: 18, hour: 18}).palaces)"
```

构建后可从源码目录的 `index.mjs` 导入 ESM，或用 `require('./index.cjs')` 加载 CommonJS。0.2.0+ 支持公元 1–9999 年的前推格里高利历，详见[时间说明](../../docs/usage.md#历史日期与远期日期)。
