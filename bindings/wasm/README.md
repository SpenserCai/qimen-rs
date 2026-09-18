# qimen-rs WebAssembly 绑定

使用 wasm-bindgen 和 serde-wasm-bindgen，在浏览器内离线计算。复用 Rust 的日期校验和
排盘引擎，输出与 Python、Node、CLI 和 MCP 相同的 JSON 字段以及 `null` 表示方式。

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
wasm-pack build bindings/wasm --target web --scope spensercai --out-dir pkg
```

```javascript
import init, { calculate } from './pkg/qimen_wasm.js';

await init();
const chart = calculate({
  year: 2024, month: 2, day: 10, hour: 12,
  utc_offset_minutes: 480,
  day_boundary: 'zi_start',
});
console.log(chart.palaces);
```

使用 HTTP 静态服务器提供 `.wasm` 文件，或通过支持 Wasm 的打包器构建。
`calculateJson(requestJson)` 接受并返回 JSON 字符串；无效请求抛出 JavaScript `Error`。
请求时间是民用时间，默认 UTC+08:00、23:00 换日，不读取浏览器时区。

Node 环境的 Wasm 冒烟测试：

```bash
wasm-pack build bindings/wasm --target nodejs --out-dir pkg-node
node bindings/wasm/tests/smoke.cjs
```

发行目标是 `@spensercai/qimen-wasm`（`--scope spensercai`），`pkg/` 内包含 JS、Wasm 和
TypeScript 声明。浏览器 web 构建与 Node 测试构建须分别生成，不能混用加载方式。
