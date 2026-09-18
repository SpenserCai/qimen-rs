# qimen-rs

[简体中文](README.md) · [Algorithm sources](docs/algorithm-sources.md) · [Agent guide](AGENTS.md) · [Releasing](docs/releasing.md)

A Rust library for Four Pillars (BaZi) and Qimen Dunjia charts from Gregorian dates and civil times. The initial implementation uses **hour-based Qimen, Chai Bu (拆补), rotating plates (转盘), fixed center-to-Kun hosting, and Tian Qin accompanying Tian Rui**.

The computation is offline. CLI, MCP, Python, Node.js, and WebAssembly all call the same Rust libraries.

## Architecture

| Component | Responsibility |
| --- | --- |
| `crates/qimen-calendar` | Validated dates, astronomical solar-term boundaries, lunar labels, Four Pillars; isolates tyme4rs |
| `crates/qimen-core` | Ju selection, plate rotation, typed versioned chart and JSON API |
| `bindings/python` | PyO3 / maturin, native dictionaries and JSON |
| `bindings/node` | napi-rs / N-API, JavaScript objects and TypeScript declarations |
| `bindings/wasm` | wasm-bindgen for browsers and other WASM hosts |
| `apps/qimen-cli` | Terminal chart, BaZi and JSON output |
| `apps/qimen-mcp` | Official rmcp SDK, stdio MCP tools |

Dependencies flow from applications and bindings into the core, then into the calendar adapter. New schools require independently verified core strategies; adapters must not duplicate calculation rules.

## Getting started

Use Rust stable with edition 2024. Until registry packages are published, build from source:

```bash
git clone https://github.com/SpenserCai/qimen-rs.git
cd qimen-rs
cargo run -p qimen-cli -- paipan --year 2026 --month 9 --day 18 --hour 15
cargo run -p qimen-cli -- paipan --year 2026 --month 9 --day 18 --hour 15 --json
cargo run -p qimen-cli -- bazi --year 2026 --month 9 --day 18 --hour 15
```

```rust
use qimen_core::{ChartRequest, calculate};

let chart = calculate(&ChartRequest::new(2026, 9, 18, 15))?;
println!("{}", serde_json::to_string_pretty(&chart)?);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Canonical contracts: [request schema](docs/schema/request.schema.json), [chart schema](docs/schema/chart.schema.json), and [complete example](docs/examples/2026-09-18T150000+0800.json).

The common JSON request is:

```json
{"year":2026,"month":9,"day":18,"hour":15,"minute":0,"second":0,"utc_offset_minutes":480,"day_boundary":"zi_start"}
```

Minutes and seconds default to zero, the fixed UTC offset to +480 minutes, and the day boundary to 23:00. Unknown fields and invalid dates are rejected. Output includes a schema version; JSON property order is not an API contract.

See [Python](bindings/python/README.md), [Node.js](bindings/node/README.md), and [WebAssembly](bindings/wasm/README.md) for binding build instructions and examples.

## MCP

```bash
cargo build --release -p qimen-mcp
./target/release/qimen-mcp
```

Example client configuration (replace the executable path):

```json
{"mcpServers":{"qimen":{"command":"/absolute/path/to/qimen-mcp","args":[]}}}
```

The server exposes calendar/BaZi and complete-chart tools. The SDK handles the **2026-07-28 lifecycle** and legacy initialization. Protocol compatibility is integration-tested. Standard output is reserved for MCP messages.

## Calculation contract

| Topic | Convention |
| --- | --- |
| Date range | Gregorian years 1900–2100 inclusive |
| Clock | Explicit fixed UTC offset, default UTC+08:00; callers supply the applicable DST offset |
| Year pillar | Changes at the precise start-of-spring instant, not Lunar New Year |
| Month pillar | Changes at the twelve Jie instants, not at calendar month boundaries |
| Day pillar | `zi_start` changes at 23:00; `midnight` changes at 00:00 |
| Late Zi hour | Under `midnight`, 23:00 retains the current day pillar but uses the next day's hour stem, keeping Zi hour continuous |
| Solar terms | Compared as instants; year/month pillars are invariant under equivalent UTC-offset representations |
| Day/hour clock | The supplied local civil clock; no implicit apparent-solar-time correction |
| Dun | Yang from winter solstice, Yin from summer solstice, at the term instant |
| Yuan / Ju | Jia/Ji Fu Tou determines the five-day Yuan; current term selects Chai Bu Ju |
| Center | Fixed hosting in Kun 2; original center retained, Tian Qin travels with Tian Rui |
| Duty door | Fly from the original xun-head palace before applying center hosting |
| Void / horse | Hour-pillar xun void and traveling horse markers |

Other programs may use different schools, hosting rules, apparent solar time or day boundaries. Match those settings before comparing outputs. Other schools are not currently advertised as implemented.

Second-resolution solar-term timestamps reflect the calculation's output resolution, not guaranteed one-second agreement with every astronomical almanac. Record dependency versions and boundary times when investigating near-boundary differences.

## Chart contents

The shared result includes the request and conventions, lunar date, Four Pillars, adjacent solar terms, Dun/Yuan/Ju, Fu Tou, hour xun head and hidden stem, duty star and door with original/actual palaces, and all nine Luo Shu palaces. Each palace carries direction, trigram, element, earth/heaven stems, hosted stems and stars, doors, deities, void and horse markers. Hosting remains explicit rather than dropping the center or Tian Qin.

Interpretation and predictions are outside the foundational chart data model.

## Quality and maintenance

CI checks formatting, warning-free compilation and Clippy, tests and documentation on Linux, macOS and Windows, plus native binding and WASM smoke tests. Tests live in separate files: public behavior in `tests/`, private unit tests in separate test modules when needed. Inline unit tests are also valid Rust; separation is this project's maintainability choice.

Calendar fixtures, worked chart cases, structural invariants and adapter integration tests serve different purposes. Passing invariants alone is not independent proof of every chart. See [sources and verification limits](docs/algorithm-sources.md) and [validation evidence](docs/validation.md).

For comparisons, provide the complete Gregorian timestamp, offset, rollover/solar-time/hosting settings and all nine palaces. Confirmed external cases should become permanent regression fixtures.

Registry publishing is separate from normal CI. Configure credentials and follow the [release guide](docs/releasing.md). Initial implementation does not automatically publish or reserve package names.

## License

[MIT](LICENSE). Dependencies and research references retain their respective licenses; incompatible reference implementations are not copied into this project.
