import Link from "next/link";
import { DocumentLayout } from "@/components/document-layout";
import { pageMetadata, REPOSITORY_URL } from "@/lib/site/metadata";

const introduction =
  "qimen-rs 是开源的八字与奇门遁甲排盘项目。网页提供无需登录的在线工具，底层 Rust 库和多语言接口便于集成到其他应用。";
export const metadata = pageMetadata("/about", "关于 qimen-rs", introduction);

export default function AboutPage() {
  return (
    <DocumentLayout title="关于 qimen-rs" introduction={introduction}>
      <section>
        <h2>本地计算，规则透明</h2>
        <p>
          Web 在浏览器的 WebAssembly Worker
          中完成计算，排盘输入不会发送到业务服务器。页面提供八字、阴阳遁、局数、旬首、值符值使，以及九宫中的天地盘干、星、门、神、空亡与驿马。
        </p>
        <p>
          目前实现时家拆补转盘。暗干、旺衰、十二长生等注记可按明确规则启用；不同流派的结果需要先核对计算约定。
        </p>
      </section>
      <section>
        <h2>开源与使用范围</h2>
        <p>
          项目采用 MIT 许可证，支持 Rust、Python、Node.js、WebAssembly、CLI 和
          MCP。排盘输出是传统术数规则的计算结果，项目目前不提供自动断语或预测服务。
        </p>
        <p>
          <a href={REPOSITORY_URL}>查看 GitHub 源码与许可证</a> ·{" "}
          <Link href="/developers">接入自己的应用</Link>
        </p>
      </section>
      <section>
        <h2>保存与分享</h2>
        <p>
          偏好保存在当前浏览器；复制结果与导出 JSON
          由用户主动操作。分享链接包含排盘时间和选项，收到链接的人可以查看这些参数并重新排盘。
        </p>
        <p>
          <Link href="/guide">阅读使用指南</Link> ·{" "}
          <Link href="/guide/conventions">核对时间约定</Link>
        </p>
      </section>
    </DocumentLayout>
  );
}
