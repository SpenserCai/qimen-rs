import { QimenWorkspace } from "@/components/qimen-workspace";
import {
  pageMetadata,
  SITE_URL,
  SITE_DESCRIPTION,
  REPOSITORY_URL,
} from "@/lib/site/metadata";

export const metadata = pageMetadata("/", "奇门遁甲在线排盘", SITE_DESCRIPTION);
const structuredData = {
  "@context": "https://schema.org",
  "@graph": [
    {
      "@type": "WebSite",
      "@id": `${SITE_URL}/#website`,
      url: SITE_URL,
      name: "qimen-rs",
      inLanguage: "zh-CN",
    },
    {
      "@type": "WebApplication",
      "@id": `${SITE_URL}/#app`,
      url: SITE_URL,
      name: "qimen-rs · 奇门遁甲在线排盘",
      description: SITE_DESCRIPTION,
      applicationCategory: "UtilitiesApplication",
      operatingSystem: "Web",
      isAccessibleForFree: true,
      license: `${REPOSITORY_URL}/blob/main/LICENSE`,
      sameAs: [REPOSITORY_URL],
    },
  ],
};

export default function Home() {
  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{
          __html: JSON.stringify(structuredData).replace(/</g, "\\u003c"),
        }}
      />
      <QimenWorkspace />
    </>
  );
}
