# 使用指南

[中文首页](../README.md) · [安装](installation.md) · [算法约定](algorithm-sources.md) · [扩展规则](extensions.md)

CLI、MCP 与语言绑定使用同一套计算参数。本页的基础调用适用于 0.1.0+；Streamable HTTP 与扩展日期范围要求 **0.2.0+**。可通过 `qimen --version` 或 `qimen-mcp --version` 检查应用版本。

## 输入时间

| JSON 字段 | 含义 | 默认值 |
| --- | --- | --- |
| `year` | 公历年；0.1.0 为 1900–2100，0.2.0+ 为 1–9999 | 必填 |
| `month` | 月，1–12 | 必填 |
| `day` | 日，按年月校验 | 必填 |
| `hour` | 当地民用小时，0–23 | 必填 |
| `minute` | 分钟，0–59 | `0` |
| `second` | 秒，0–59；不接受闰秒 | `0` |
| `utc_offset_minutes` | 相对 UTC 的分钟偏移，−840 至 +840 | `480` |
| `day_boundary` | `zi_start`（23:00）或 `midnight`（00:00） | `zi_start` |
| `extensions` | 排盘的可选注记；`bazi` 不接受此项 | 全部关闭 |

输入时间是指定偏移下的民用时间。库不根据城市查找时区，不自动查询夏令时，也不进行经度或真太阳时修正；使用这些时间口径时，应先在调用端完成转换，再传入实际时间和偏移。

年柱按立春，月柱按十二节的交节时刻切换；同一绝对时刻用不同 UTC 偏移表达，年柱和月柱不变。日柱和时柱依输入的本地钟表时间确定。

### 历史日期与远期日期

**0.2.0+** 输入接受 **公元 1–9999 年的前推格里高利历**，范围包含端点及全部允许的 UTC 偏移。1582 年以前也使用相同的公历闰年规则，因此 1582-10-05 至 1582-10-14 都是有效日期；1500-02-29 无效，1600-02-29 有效。

采用历史儒略历或 1582 年混合历的软件可能把同名日期解释为不同的实际日期。比较这类历史盘时，应先换算历法日期，再统一 UTC 偏移和换日规则。

相邻节气可能跨出输入年份范围，返回年份 `0` 或 `10000`；公元 1 年初的农历年份也可能为 `0`。这些是完整表示边界计算所需的输出，并不表示输入可以使用公元 0 年。

节气和朔时采用 tyme4rs 的天文求解器。农历月序按现行定气、定朔规则统一前推及外推：冬至所在月为十一月，相邻冬至月之间有十三个月时，首个不含中气的月设为闰月。朔日以 UTC+08:00 民用日界确定；返回的是输入公历年月日对应的中国历日期标签，不为每个 UTC 偏移重新建立地方农历。

支持上古及远期输入表示算法可计算的范围，并不承诺所有年代都具有现代天文精度，也不还原各历史地区的地方时或历代颁行历法。秒级时间字段表示输出分辨率。

## CLI

```bash
qimen paipan --year 2026 --month 9 --day 18 --hour 18
qimen bazi --year 2026 --month 9 --day 18 --hour 18 --json
qimen paipan --year 2026 --month 9 --day 18 --hour 23 \
  --day-boundary midnight --utc-offset-minutes 480 --json
```

`paipan` 输出完整奇门盘，`chart` 是同义子命令；`bazi` 仅返回农历、四柱与节气。命令行参数用连字符，例如 JSON 的 `utc_offset_minutes` 对应 `--utc-offset-minutes`，`zi_start` 对应 `--day-boundary zi-start`。

| 选项 | 用途 |
| --- | --- |
| `--json` | 输出与库相同的 JSON，可重定向到文件 |
| `--extensions all` | 开启全部已实现注记，仅适用于 `paipan` |
| `--extensions hidden-stems,day-horse` | 按逗号分隔，只启用指定注记 |
| `--tomb-rule traditional-three-wonders` | 选择古典三奇入墓；须同时开启 `tombs` 或 `all` |
| `--help` | 查看命令及可用参数 |
| `--version` | 查看程序版本 |

全部扩展名为 `hidden-stems`、`strength`、`growth-stages`、`punishments`、`tombs`、`day-horse`、`door-pressure`。默认入墓规则为 `growth-stage-fire-earth`。详细差异见[扩展规则](extensions.md)。

```bash
qimen paipan --year 2026 --month 9 --day 18 --hour 18 \
  --extensions all --json > chart.json
```

参数或计算失败时，CLI 将错误写入标准错误并返回非零退出码。

## MCP

`qimen-mcp` 提供两个只读工具：

| 工具 | 参数 | 结果 |
| --- | --- | --- |
| `bazi` | 时间字段，不含 `extensions` | 农历、四柱、当前和下一节气 |
| `paipan` | 时间字段及可选 `extensions` | 完整九宫盘与已开启注记 |

客户端调用 `paipan` 的 `arguments` 示例：

```json
{
  "year": 2026,
  "month": 9,
  "day": 18,
  "hour": 18,
  "utc_offset_minutes": 480,
  "extensions": {
    "day_horse": "day_branch_three_harmony"
  }
}
```

成功结果位于 MCP 工具响应的 `structuredContent`。计算失败返回工具错误和 `calculation_error`，未知工具或不符合参数结构的请求返回 MCP 参数错误。

### stdio

默认启动方式适合由本地 MCP 客户端管理进程：

```bash
qimen-mcp
```

```json
{
  "mcpServers": {
    "qimen": {
      "command": "/absolute/path/to/qimen-mcp",
      "args": []
    }
  }
}
```

使用可执行文件的实际绝对路径，Windows 路径需按 JSON 规则转义反斜杠。stdio 的标准输出只包含 MCP 消息，诊断信息写入标准错误。

### Streamable HTTP（0.2.0+）

使用 0.2.0 或更高版本运行：

```bash
qimen-mcp --transport streamable-http
```

默认监听 `127.0.0.1:8080`，MCP 端点为 `http://127.0.0.1:8080/mcp`。在支持 Streamable HTTP 的 MCP 客户端中选择 HTTP 传输并填入该 URL；客户端配置文件格式以其文档为准。

指定监听地址和浏览器来源：

```bash
qimen-mcp --transport streamable-http \
  --bind 127.0.0.1:8081 --allow-origin http://localhost:5173
```

| HTTP 参数 | 含义 |
| --- | --- |
| `--transport streamable-http` | 选择 Streamable HTTP；默认传输仍为 `stdio` |
| `--bind IP:PORT` | 监听地址，默认 `127.0.0.1:8080` |
| `--allow-host HOST[:PORT]` | 添加可信 Host，可重复传入；默认允许回环地址和实际绑定的非通配地址 |
| `--allow-origin ORIGIN` | 添加可信浏览器来源，可重复传入；默认拒绝带 Origin 的请求 |

不带 Origin 的原生 MCP 客户端可直接连接默认本地端点。HTTP 参数仅适用于 HTTP 模式。`--allow-origin` 仅配置来源校验。按 rmcp 的匹配规则，省略端口会允许相同 scheme 和 host 的任意端口；需要限制端口时可使用 `--allow-origin https://app.example.com:8443`，客户端 Origin 也须携带该端口。浏览器通常省略默认的 80 / 443 端口，配置时需注意这一表达差异。浏览器跨域调用所需的 CORS 响应头应在反向代理配置。服务不内建 TLS 或 OAuth；向远程用户提供访问时，使用带 TLS 和访问鉴权的反向代理，并保留 MCP 请求头与 SSE 响应。代理使用域名时，将该域名加入 `--allow-host`。

服务采用标准 Streamable HTTP `/mcp` 端点，不提供旧式独立 `/sse` 端点。rmcp 处理 2026-07-28 的服务发现与调用，也兼容 2025-03-26、2025-06-18、2025-11-25 客户端的初始化与会话；stdio 另兼容 2024-11-05。2026-07-28 使用无状态调用；旧版 HTTP 协议由 SDK 管理会话。客户端应通过正常 MCP 生命周期连接，不能将 `paipan` 请求直接作为任意 REST JSON 发到端点。

## 结果格式

| 字段 | 内容 |
| --- | --- |
| `schema_version` | 数据格式版本，与软件包版本分开 |
| `input`、`conventions` | 原始参数、时间与流派约定 |
| `calendar` | 四柱、农历、相邻节气 |
| `dun`、`yuan`、`ju` | 阴阳遁、三元、局数 |
| `xun`、`leaders` | 旬首与遁干、值符值使及原宫和落宫 |
| `palaces` | 九宫位置、星门神、天地盘干、寄干、空亡与马星 |
| `extensions` | 已启用的注记及其规则；未启用时省略 |

宫号使用洛书顺序 `1..9`，不要把数组索引当作宫号。南上九宫显示的三行宫号为 `4,9,2` / `3,5,7` / `8,1,6`。中宫天禽随芮，寄干和原始宫位分别保留。

精确类型见[结果 Schema](schema/chart.schema.json)，实际结构见[完整 JSON 示例](examples/2026-09-18T150000+0800.json)。JSON 属性顺序不属于接口约定；未启用、规则不适用和已启用但未命中分别表达，不应统一当作 `false`。

## 计算差异反馈

通过 [Issues](https://github.com/SpenserCai/qimen-rs/issues) 反馈时，请提供项目版本、完整请求和 JSON 结果，以及参考结果采用的时间、日界、定局、寄宫和真太阳时设置。注明具体差异字段，例如月柱、局数或值使落宫，有助于区分历法口径、流派约定与计算问题。
