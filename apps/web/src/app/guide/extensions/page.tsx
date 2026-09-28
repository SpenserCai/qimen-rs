import { DocumentLayout } from "@/components/document-layout";
import { pageMetadata, REPOSITORY_URL } from "@/lib/site/metadata";

const introduction =
  "暗干、旺衰、十二长生、六仪击刑、入墓、日马与门迫是可选的派生注记。明确选择规则后才会计算，开关不改变基础排盘。";
export const metadata = pageMetadata(
  "/guide/extensions",
  "扩展注记与参数",
  introduction,
);
const options = [
  [
    "hidden_stems",
    "duty_door_hour_stem_with_center_fallback",
    "值使宫起时干，阳顺阴逆飞布九宫；重本位地盘干时改从中五起，甲时先以旬首遁干代甲",
  ],
  [
    "strength",
    "classical_stars_and_five_elements",
    "星用烟波法，门、干用一般五行法；分别返回落宫、节气月令两个参照下的状态",
  ],
  [
    "growth_stages",
    "yang_forward_yin_reverse_fire_earth",
    "十二长生采用阳顺阴逆、戊随丙、己随丁，每个实际干与宫支分别计算",
  ],
  [
    "punishments",
    "six_instrument_branches",
    "六仪击刑，按戊卯、己未、庚寅、辛午、壬辰、癸巳对应关系判断",
  ],
  [
    "tombs",
    "growth_stage_fire_earth",
    "十干十二长生墓，乙墓在戌；ExtensionOptions::all() 和 CLI 全开预设采用此项",
  ],
  [
    "tombs",
    "traditional_three_wonders",
    "古典三奇墓：乙未、丙戌、丁丑；六仪不适用，与上一项二选一",
  ],
  [
    "day_horse",
    "day_branch_three_harmony",
    "根据已计算日柱的日支取三合驿马，遵循请求的换日规则",
  ],
  [
    "door_pressure",
    "door_controls_palace",
    "仅判断门五行克落宫五行，反向的宫克门不属于此项",
  ],
];

export default function ExtensionsPage() {
  return (
    <DocumentLayout title="扩展注记与参数" introduction={introduction}>
      <section>
        <h2>如何开启</h2>
        <p>
          Web 在左侧扩展区选择项目，再点击「开始排盘」生效。语言接口与 MCP 的
          paipan 工具使用 extensions 对象，每个 key 对应明确的规则字符串，不能用
          true / false 代替。
        </p>
        <pre>
          <code>
            {JSON.stringify(
              {
                year: 2026,
                month: 9,
                day: 18,
                hour: 18,
                extensions: { day_horse: "day_branch_three_harmony" },
              },
              null,
              2,
            )}
          </code>
        </pre>
      </section>
      <section>
        <h2>请求 key 与规则 value</h2>
        <div
          className="overflow-x-auto"
          role="region"
          aria-label="扩展参数表，可横向滚动"
          tabIndex={0}
        >
          <table>
            <caption className="sr-only">
              extensions 可用字段、规则与含义
            </caption>
            <thead>
              <tr>
                <th scope="col">key</th>
                <th scope="col">value</th>
                <th scope="col">含义</th>
              </tr>
            </thead>
            <tbody>
              {options.map(([key, value, meaning]) => (
                <tr key={value}>
                  <th scope="row">
                    <code>{key}</code>
                  </th>
                  <td>
                    <code>{value}</code>
                  </td>
                  <td>{meaning}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>
      <section>
        <h2>结果含义</h2>
        <p>
          省略 key 或设置为 null 表示关闭；空对象关闭全部扩展。JSON 没有 all
          字符串预设。输出中各项保留实际
          rule、宫位与盘层，寄干保留来源；日马与基础时马分别返回。
        </p>
        <p>
          未开启、不适用和已开启但未命中是不同状态。传统三奇入墓只判断三奇，六仪的不适用结果为
          null，不能读成未入墓的 false。两种入墓规则二选一。
        </p>
        <p>
          目前支持时家拆补转盘。其他盘式是否适用每项注记，需要核对宫序、星门和寄宫约定。
        </p>
        <a href={`${REPOSITORY_URL}/blob/main/docs/extensions.md`}>
          查看完整规则、返回字段与来源
        </a>
      </section>
    </DocumentLayout>
  );
}
