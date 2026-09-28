import { ImageResponse } from "next/og";

export const alt = "qimen-rs · 开源奇门遁甲排盘";
export const size = { width: 1200, height: 630 };
export const contentType = "image/png";
export const dynamic = "force-static";

/** Static artwork: no remote fetches or user-controlled rendering input. */
export default function Image() {
  return new ImageResponse(
    <div
      style={{
        display: "flex",
        width: "100%",
        height: "100%",
        background: "#071322",
        color: "#f3dca0",
        padding: 70,
        alignItems: "center",
        justifyContent: "space-between",
      }}
    >
      <div style={{ display: "flex", flexDirection: "column" }}>
        <div style={{ fontSize: 20, letterSpacing: 6, color: "#8ccac0" }}>
          QIMEN DUNJIA
        </div>
        <div style={{ fontSize: 100, marginTop: 30 }}>qimen-rs</div>
        <div style={{ fontSize: 26, marginTop: 25, color: "#ecebdc" }}>
          Open source. Calculate locally.
        </div>
        <div style={{ fontSize: 19, marginTop: 55, color: "#afbfcb" }}>
          Rust / Python / Node.js / WASM / MCP
        </div>
      </div>
      <div
        style={{
          display: "flex",
          width: 310,
          height: 310,
          border: "1px solid #d5b779",
          borderRadius: "50%",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <div
          style={{
            display: "flex",
            width: 270,
            height: 270,
            border: "1px solid #526c7e",
            borderRadius: "50%",
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          <div
            style={{
              display: "flex",
              flexWrap: "wrap",
              width: 174,
              height: 174,
              border: "1px solid #d5b779",
            }}
          >
            {Array.from({ length: 9 }, (_, index) => (
              <div
                key={index}
                style={{
                  display: "flex",
                  width: "33.333%",
                  height: "33.333%",
                  border: "1px solid #526c7e",
                  background: index === 4 ? "#183a40" : "transparent",
                }}
              />
            ))}
          </div>
        </div>
      </div>
    </div>,
    size,
  );
}
