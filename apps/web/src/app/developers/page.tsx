import Link from "next/link";
import { DocumentLayout } from "@/components/document-layout";
import { pageMetadata, REPOSITORY_URL } from "@/lib/site/metadata";

const introduction =
  "通过 Rust、Python、Node.js、WebAssembly、CLI 或 MCP 接入同一套八字与奇门排盘引擎。请求显式指定公历时间，结果保留盘式、规则与各宫元素。";
export const metadata = pageMetadata("/developers", "开发者接入", introduction);
const examples = [
  {
    title: "Python",
    install: "python -m pip install qimen-rs",
    code: 'from qimen_rs import calculate\n\nchart = calculate({"year": 2026, "month": 9, "day": 18, "hour": 18})\nprint(chart["palaces"])',
    docs: "bindings/python",
    note: "Python 3.10+，返回字典；calculate_json 接受和返回 JSON 字符串。",
  },
  {
    title: "Node.js",
    install: "npm install @spensercai/qimen-rs",
    code: "import { calculate } from '@spensercai/qimen-rs';\n\nconst chart = calculate({ year: 2026, month: 9, day: 18, hour: 18 });\nconsole.log(chart.palaces);",
    docs: "bindings/node",
    note: "Node.js 20+，支持 ESM、CommonJS 和 TypeScript。安装时请保留平台可选依赖。",
  },
  {
    title: "浏览器 / WebAssembly",
    install: "npm install @spensercai/qimen-wasm",
    code: "import init, { calculate } from '@spensercai/qimen-wasm';\n\nawait init();\nconst chart = calculate({ year: 2026, month: 9, day: 18, hour: 18 });\nconsole.log(chart.palaces);",
    docs: "bindings/wasm",
    note: "用于能解析 npm 和 WASM 资源的浏览器打包器。须先等待初始化完成，并部署包中的 .wasm 文件；批量计算建议放到 Worker。",
  },
  {
    title: "Rust",
    install: "cargo add qimen-core",
    code: 'use qimen_core::{ChartRequest, calculate};\n\nfn main() -> Result<(), qimen_core::Error> {\n    let chart = calculate(&ChartRequest::new(2026, 9, 18, 18))?;\n    println!("{:?}", chart.calendar.four_pillars);\n    Ok(())\n}',
    docs: "crates/qimen-core",
    note: "库不读取机器时区或当前时间，输入校验与错误通过类型返回。",
  },
];

export default function DevelopersPage() {
  return (
    <DocumentLayout title="开发者接入" introduction={introduction}>
      <p>
        默认使用 UTC+08:00 与子初换日，扩展标注默认关闭。完整参数见{" "}
        <Link href="/guide/conventions">时间约定</Link> 和{" "}
        <Link href="/guide/extensions">扩展参数表</Link>。
      </p>
      {examples.map((example) => (
        <section key={example.title}>
          <h2>{example.title}</h2>
          <pre>
            <code>{example.install}</code>
          </pre>
          <pre>
            <code>{example.code}</code>
          </pre>
          <p>{example.note}</p>
          <a href={`${REPOSITORY_URL}/tree/main/${example.docs}`}>
            完整 {example.title} 接口说明
          </a>
        </section>
      ))}
      <section>
        <h2>CLI 与 MCP</h2>
        <p>
          从 GitHub Release 下载对应平台程序，或通过 Cargo 安装 CLI 和 MCP
          服务。
        </p>
        <pre>
          <code>
            {
              "cargo install qimen-cli qimen-mcp\nqimen paipan --year 2026 --month 9 --day 18 --hour 18 --json"
            }
          </code>
        </pre>
        <p>
          MCP 默认使用 stdio，客户端命令填入 qimen-mcp；也可启动 Streamable
          HTTP，连接本机 /mcp 端点。
        </p>
        <pre>
          <code>
            {
              "qimen-mcp --transport streamable-http\n# MCP endpoint: http://127.0.0.1:8080/mcp"
            }
          </code>
        </pre>
        <p>
          工具包括 bazi 与 paipan；后者可接收可选扩展参数。向网络开放 HTTP
          服务前请按部署环境配置访问控制。
        </p>
        <a href={`${REPOSITORY_URL}/blob/main/docs/usage.md`}>
          CLI 与 MCP 参数说明
        </a>
      </section>
    </DocumentLayout>
  );
}
