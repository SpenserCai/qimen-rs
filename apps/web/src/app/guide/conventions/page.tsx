import Link from "next/link";
import { DocumentLayout } from "@/components/document-layout";
import { pageMetadata, REPOSITORY_URL } from "@/lib/site/metadata";

const introduction =
  "比较奇门遁甲排盘结果前，应先核对日期、时区、换日、定局和寄宫规则。qimen-rs 对这些约定使用明确的参数与结果字段。";
export const metadata = pageMetadata(
  "/guide/conventions",
  "时间与排盘约定",
  introduction,
);

export default function ConventionsPage() {
  return (
    <DocumentLayout title="时间与排盘约定" introduction={introduction}>
      <section>
        <h2>公历、时区与真太阳时</h2>
        <p>
          支持公元 1–9999 年的前推格里高利历。输入是指定 UTC
          偏移下的民用时间，默认
          UTC+08:00；不会读取计算机时区或自动判断夏令时，也不会自动换算真太阳时。
        </p>
        <p>
          采用真太阳时的调用者应在外部完成修正，并明确自己的时间口径。不能将经纬度、历史夏令时或日出时刻静默混入默认计算。
        </p>
      </section>
      <section>
        <h2>四柱与换日</h2>
        <p>
          年柱以立春划分，月柱按节令交接划分。默认子初换日，即 23:00
          起计入次日；也可选择午夜 00:00
          换日。两种规则可能改变子时的日柱以及依赖日柱的注记。
        </p>
      </section>
      <section>
        <h2>定局与寄宫</h2>
        <p>
          当前使用时家拆补转盘，按节气和三元定局，中五寄坤二、天禽随天芮。基础空亡采用时旬，基础驿马采用时支；日马是独立的可选扩展。
        </p>
        <p>
          盘面按南上北下、东左西右呈现，宫号不随显示位置改变。中宫没有八门和八神，输出保留空项与寄干来源。
        </p>
      </section>
      <section>
        <h2>历史与远期日期</h2>
        <p>
          农历按现行定气定朔规则前推与外推，不等同于历史上实际颁行的历书。支持的计算范围不代表各年代都具有相同的天文精度；边界处相邻节气可能落在公元
          0 年或 10000 年。
        </p>
        <p>
          <a href={`${REPOSITORY_URL}/blob/main/docs/algorithm-sources.md`}>
            算法来源与规则说明
          </a>{" "}
          · <Link href="/guide/extensions">扩展注记约定</Link>
        </p>
      </section>
    </DocumentLayout>
  );
}
