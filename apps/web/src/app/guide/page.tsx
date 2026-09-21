import type { Metadata } from "next";
import Link from "next/link";
import { ArrowLeft, ArrowUpRight, Compass } from "lucide-react";

export const metadata: Metadata = {
  title: "使用指南",
  description: "了解公历输入、时区、换日约定、九宫阅读、可选标注与排盘分享。",
};

const annotations = [
  ["暗干", "值使门起时干，采用中宫回退约定。"],
  ["旺衰", "显示九星古法旺衰，以及五行与月令、宫位的关系。"],
  ["十二长生", "阳顺阴逆、火土同宫，按天干及宫内地支分别显示。"],
  ["六仪击刑", "标记六仪落宫所对应的击刑地支。"],
  ["入墓", "可选择十二长生墓或传统三奇入墓；两种约定分别计算。"],
  ["日马", "按日支三合局标记驿马，与基础盘的时马分开。"],
  ["门迫", "标记八门五行克落宫五行的关系。"],
];

export default function GuidePage() {
  return (
    <div className="min-h-screen bg-[#071518] px-5 py-8 text-[#e9eee6] sm:px-8 sm:py-12">
      <div className="mx-auto max-w-3xl">
        <Link
          href="/"
          className="inline-flex min-h-11 items-center gap-2 text-sm text-[#cbb681] transition-colors hover:text-white focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-[#82c8bd]"
        >
          <ArrowLeft size={18} aria-hidden="true" />
          返回排盘
        </Link>
        <main id="main-content" className="pt-10 pb-16">
          <div className="mb-5 flex items-center gap-3 text-xs tracking-[0.25em] text-[#a7bfb9]">
            <Compass size={18} aria-hidden="true" /> QIMEN · GUIDE
          </div>
          <h1 className="font-serif text-4xl leading-tight text-[#e5d1a1] sm:text-5xl">
            使用指南
          </h1>
          <p className="mt-5 text-base leading-8 text-[#b7c8c2]">
            从一个明确的时刻开始，阅读八字与时家拆补转盘。所有计算在你的浏览器内完成，无需注册。
          </p>

          <section
            className="mt-12 border-t border-[#cbb681]/20 pt-8"
            aria-labelledby="start-title"
          >
            <h2 id="start-title" className="font-serif text-2xl text-[#e5d1a1]">
              输入与起局
            </h2>
            <ol className="mt-5 list-decimal space-y-3 pl-5 leading-8 text-[#c4d1cc]">
              <li>
                输入公历日期和当地民用时间。页面初始使用北京时间（UTC+08:00）；也可填写其他
                UTC 偏移。
              </li>
              <li>
                选择换日规则：子初换日从 23:00 起计入次日；午夜换日从 00:00
                起计入次日。
              </li>
              <li>
                按需开启扩展标注，点击「开始排盘」。罗盘短暂旋转后显示结果；静心模式会减弱动效。
              </li>
              <li>
                选择任意宫位查看完整条目。前后时辰按钮以两小时为步长移动排盘时间。
              </li>
            </ol>
            <p className="mt-5 rounded-xl border border-[#82c8bd]/20 bg-[#82c8bd]/5 p-4 text-sm leading-7 text-[#b7c8c2]">
              支持公元 1–9999
              年的前推格里高利历。输入时间不会自动换算为真太阳时；UTC
              偏移须由你按目标地点与日期填写，页面不会自动判断夏令时。
            </p>
          </section>

          <section
            className="mt-10 border-t border-[#cbb681]/20 pt-8"
            aria-labelledby="chart-title"
          >
            <h2 id="chart-title" className="font-serif text-2xl text-[#e5d1a1]">
              读懂九宫
            </h2>
            <p className="mt-5 leading-8 text-[#c4d1cc]">
              九宫按南上北下排列：上排巽四、离九、坤二，中排震三、中五、兑七，下排艮八、坎一、乾六。盘面显示地盘干、天盘干、九星、八门、八神与空亡、时马，顶部列出四柱、阴阳遁、局数、旬首和值符值使。
            </p>
            <p className="mt-4 leading-8 text-[#c4d1cc]">
              本盘采用时家拆补转盘，中五寄坤二、天禽随天芮。宫位详情区保留寄干及来源；宫内多个地支的长生、入墓等条目分别列示。中宫没有八门、八神时以空项呈现。
            </p>
          </section>

          <section
            className="mt-10 border-t border-[#cbb681]/20 pt-8"
            aria-labelledby="annotations-title"
          >
            <h2
              id="annotations-title"
              className="font-serif text-2xl text-[#e5d1a1]"
            >
              可选标注
            </h2>
            <p className="mt-5 leading-8 text-[#c4d1cc]">
              扩展标注默认关闭，可单独启用；开关不改变基础盘。部分标注存在流派约定，请在比较不同排盘工具前核对所选规则。
            </p>
            <dl className="mt-5 divide-y divide-[#cbb681]/15">
              {annotations.map(([name, description]) => (
                <div
                  key={name}
                  className="grid gap-2 py-4 sm:grid-cols-[7rem_1fr]"
                >
                  <dt className="text-[#e5d1a1]">{name}</dt>
                  <dd className="text-sm leading-7 text-[#b7c8c2]">
                    {description}
                  </dd>
                </div>
              ))}
            </dl>
          </section>

          <section
            className="mt-10 border-t border-[#cbb681]/20 pt-8"
            aria-labelledby="share-title"
          >
            <h2 id="share-title" className="font-serif text-2xl text-[#e5d1a1]">
              保存与分享
            </h2>
            <p className="mt-5 leading-8 text-[#c4d1cc]">
              结果工具栏支持复制盘面、导出 JSON
              和复制分享链接。分享链接包含该次排盘的时间、时区、换日规则与扩展选项，打开后会重新计算。尚未提交的输入不改变已展示结果的导出内容。
            </p>
            <p className="mt-4 leading-8 text-[#c4d1cc]">
              偏好设置保存在当前浏览器。计算不上传输入；分享链接会向收到链接的人公开其中的排盘参数。
            </p>
          </section>

          <section
            className="mt-10 border-t border-[#cbb681]/20 pt-8"
            aria-labelledby="calendar-title"
          >
            <h2
              id="calendar-title"
              className="font-serif text-2xl text-[#e5d1a1]"
            >
              计算约定
            </h2>
            <p className="mt-5 leading-8 text-[#c4d1cc]">
              年柱以立春、月柱以节令交接划分。农历按现行定气定朔规则前推与外推；古代日期的结果不等同于当时实际颁行的历书，广年份范围也不代表各年代具有相同的天文精度。
            </p>
            <a
              className="mt-5 inline-flex min-h-11 items-center gap-2 text-sm text-[#cbb681] underline decoration-[#cbb681]/40 underline-offset-4 hover:text-white"
              href="https://github.com/SpenserCai/qimen-rs/tree/main/docs"
              target="_blank"
              rel="noopener noreferrer"
            >
              查看完整计算约定
              <ArrowUpRight size={16} aria-hidden="true" />
            </a>
          </section>
        </main>
        <footer className="border-t border-[#cbb681]/20 pt-6 text-xs text-[#a7bfb9]">
          qimen-rs · 奇门遁甲
        </footer>
      </div>
    </div>
  );
}
