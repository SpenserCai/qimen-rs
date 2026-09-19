# qimen-rs WebAssembly 绑定

[中文首页](../../README.md) · [English](../../README.en.md) · [扩展规则](../../docs/extensions.md)

使用 wasm-bindgen 和 serde-wasm-bindgen，在浏览器内离线计算。复用 Rust 的日期校验和
排盘引擎，输出与 Python、Node、CLI 和 MCP 相同的 JSON 字段以及 `null` 表示方式。

## 快速开始

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
wasm-pack build bindings/wasm --target web \
  --scope spensercai --out-dir pkg --release -- --locked
```

```javascript
import init, { calculate } from './pkg/qimen_wasm.js';

await init();
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

使用 HTTP 静态服务器提供 `.wasm` 文件，或通过支持 Wasm 的打包器构建。
`calculateJson(requestJson)` 接受并返回 JSON 字符串；无效请求抛出 JavaScript `Error`。
请求时间是民用时间，默认 UTC+08:00、23:00 换日，不读取浏览器时区。

## 可选扩展

扩展标注使用与 Node / Rust JSON 接口相同的可选 `extensions` 参数：

```javascript
const annotated = calculate({
  year: 2026,
  month: 9,
  day: 18,
  hour: 18,
  extensions: {
    hidden_stems: 'duty_door_hour_stem_with_center_fallback',
    strength: 'classical_stars_and_five_elements',
    growth_stages: 'yang_forward_yin_reverse_fire_earth',
    punishments: 'six_instrument_branches',
    tombs: 'growth_stage_fire_earth',
    day_horse: 'day_branch_three_harmony',
    door_pressure: 'door_controls_palace',
  },
});
console.log(annotated.extensions.day_horse.horse);
```

所有项目默认关闭，可只选择需要的项目。每项结果携带实际 `rule`；寄干保留来源、盘层，
日马与时马分别返回。`tombs` 可改选 `traditional_three_wonders`，不适用的六仪结果为
`null`。所有扩展参数、结果的 TypeScript 类型包含在生成包中，schema 版本为 `1.1`。
对象入口和 JSON 入口均由 Rust 严格校验未知字段、错误规则和参数类型。
完整规则与适用范围见 [扩展约定](../../docs/extensions.md)。

## 开发与构建

Node 环境的 WASM 运行测试：

```bash
wasm-pack build bindings/wasm --target nodejs \
  --out-dir pkg-node --release -- --locked
node bindings/wasm/tests/smoke.cjs
```

发行目标是 `@spensercai/qimen-wasm`（`--scope spensercai`），`pkg/` 内包含 JS、Wasm 和
TypeScript 声明。浏览器 web 构建与 Node 测试构建须分别生成，不能混用加载方式。

发布开关与凭据配置见[发布指南](../../docs/releasing.md)。
