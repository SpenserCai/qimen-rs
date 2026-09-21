import { ENGINE_ASSET_DIRECTORY } from "./engine-config";
import { validateRequest } from "./request";
import {
  calculateMessage,
  isWorkerRequest,
  type WorkerResponse,
} from "./worker-protocol";

type Engine = typeof import("@spensercai/qimen-wasm");
let enginePromise: Promise<Engine> | null = null;

function loadEngine(): Promise<Engine> {
  if (enginePromise) return enginePromise;
  const moduleUrl = new URL(
    `${ENGINE_ASSET_DIRECTORY}qimen_wasm.js`,
    self.location.origin,
  );
  enginePromise = (async () => {
    try {
      // The unmodified npm distribution is copied at build time. Runtime import
      // avoids asking the JS bundler to reinterpret wasm-bindgen's binary URL.
      const engine = (await import(
        /* webpackIgnore: true */ /* turbopackIgnore: true */ moduleUrl.href
      )) as Engine;
      await engine.default({
        module_or_path: new URL("qimen_wasm_bg.wasm", moduleUrl),
      });
      return engine;
    } catch {
      enginePromise = null;
      throw new Error("计算引擎加载失败，请检查网络后重试。");
    }
  })();
  return enginePromise;
}

self.addEventListener("message", (event: MessageEvent<unknown>) => {
  if (!isWorkerRequest(event.data)) return;
  void calculateMessage(event.data, async (request) => {
    const validated = validateRequest(request);
    if (!validated.ok) throw new Error(validated.error);
    const engine = await loadEngine();
    return engine.calculate(validated.request);
  }).then((response: WorkerResponse) => self.postMessage(response));
});
