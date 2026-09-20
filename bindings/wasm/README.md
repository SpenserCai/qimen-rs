# qimen-rs WebAssembly 绑定

[中文首页](../../README.md) · [English](../../README.en.md) · [扩展规则](../../docs/extensions.md)

使用 wasm-bindgen 和 serde-wasm-bindgen，在浏览器内离线计算。复用 Rust 的日期校验和
排盘引擎，输出与 Python、Node、CLI 和 MCP 相同的 JSON 字段以及 `null` 表示方式。

## 安装与快速开始

```bash
npm install @spensercai/qimen-wasm
```

安装命令获取当前正式版。扩展日期范围要求 0.2.0+。以下示例用于能解析 npm 包和 WASM 资源的浏览器打包器：

```javascript
import init, { calculate } from '@spensercai/qimen-wasm';

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

先等待 `init()` 完成，再调用计算函数。部署时同时提供包中的 `.wasm` 文件；静态服务器应返回 `application/wasm` MIME 类型。也可将包中的 JS 和 WASM 放入站点同一目录，使用相对路径导入 `qimen_wasm.js`。

`calculateJson(requestJson)` 接受并返回 JSON 字符串；无效请求抛出 JavaScript `Error`。请求时间是民用时间，默认 UTC+08:00、23:00 换日，不读取浏览器时区。

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
`null`。所有扩展参数和结果的 TypeScript 类型包含在包中；JSON Schema 与软件包使用不同的版本号。
对象入口和 JSON 入口均由 Rust 严格校验未知字段、错误规则和参数类型。
完整规则与适用范围见 [扩展约定](../../docs/extensions.md)。

## 从源码使用

需 Rust 1.94+ 和 wasm-pack，在所需版本的仓库根目录构建浏览器包：

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
wasm-pack build bindings/wasm --target web \
  --scope spensercai --out-dir pkg --release -- --locked
```

生成目录为 `bindings/wasm/pkg`。在浏览器页面中按实际位置导入：

```javascript
import init, { calculate } from './pkg/qimen_wasm.js';

await init();
console.log(calculate({ year: 1800, month: 1, day: 1, hour: 12 }).palaces);
```

0.2.0+ 支持公元 1–9999 年的前推格里高利历；历史日期与边界输出见[时间说明](../../docs/usage.md#历史日期与远期日期)。

浏览器包使用 `--target web` 的初始化方式；若自行构建 `--target nodejs` 产物，应遵循其 CommonJS 加载方式，不能互换入口。Node.js 原生使用建议见 [Node.js 绑定](../node/README.md)。
