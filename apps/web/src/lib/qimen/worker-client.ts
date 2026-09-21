import type { Chart, ChartRequest } from "./types";
import type { WorkerRequest, WorkerResponse } from "./worker-protocol";

export interface CalculationWorker {
  postMessage(message: WorkerRequest): void;
  terminate(): void;
  addEventListener(
    type: "message",
    listener: (event: MessageEvent<WorkerResponse>) => void,
  ): void;
  addEventListener(
    type: "error" | "messageerror",
    listener: (event: Event) => void,
  ): void;
  removeEventListener(
    type: "message",
    listener: (event: MessageEvent<WorkerResponse>) => void,
  ): void;
  removeEventListener(
    type: "error" | "messageerror",
    listener: (event: Event) => void,
  ): void;
}

function browserWorker(): CalculationWorker {
  if (typeof Worker === "undefined")
    throw new Error("当前浏览器不支持后台计算，请使用新版浏览器。");
  return new Worker(new URL("./qimen.worker.ts", import.meta.url), {
    type: "module",
    name: "qimen-calculator",
  });
}

/** One worker and one active calculation per workspace; failed workers can be replaced. */
export class QimenWorkerClient {
  private worker: CalculationWorker | null = null;
  private sequence = 0;
  private pending: ((reason: Error) => void) | null = null;

  constructor(
    private readonly createWorker: () => CalculationWorker = browserWorker,
    private readonly timeoutMs = 20_000,
  ) {}

  calculate(request: ChartRequest): Promise<Chart> {
    if (this.pending) return Promise.reject(new Error("正在起局，请稍候。"));
    let worker: CalculationWorker;
    try {
      this.worker ??= this.createWorker();
      worker = this.worker;
    } catch (error) {
      return Promise.reject(error);
    }
    const id = ++this.sequence;
    return new Promise<Chart>((resolve, reject) => {
      const cleanup = () => {
        clearTimeout(timer);
        worker.removeEventListener("message", onMessage);
        worker.removeEventListener("error", onError);
        worker.removeEventListener("messageerror", onError);
        this.pending = null;
      };
      const fail = (error: Error) => {
        cleanup();
        reject(error);
      };
      const onMessage = (event: MessageEvent<WorkerResponse>) => {
        const response = event.data;
        if (!response || response.id !== id) return;
        if (response.type === "error") {
          // A failed dynamic import can be cached by the worker's module map.
          // Replacing the worker makes an explicit retry a fresh network attempt.
          this.destroyWorker();
          fail(new Error(response.message));
        } else if (response.type === "result") {
          cleanup();
          resolve(response.chart);
        }
      };
      const onError = () => {
        this.destroyWorker();
        fail(new Error("后台计算意外中断，请重试。"));
      };
      const timer = setTimeout(() => {
        this.destroyWorker();
        fail(new Error("计算引擎加载超时，请检查网络后重试。"));
      }, this.timeoutMs);
      this.pending = fail;
      worker.addEventListener("message", onMessage);
      worker.addEventListener("error", onError);
      worker.addEventListener("messageerror", onError);
      try {
        worker.postMessage({ type: "calculate", id, request });
      } catch {
        this.destroyWorker();
        fail(new Error("排盘任务无法启动，请重试。"));
      }
    });
  }

  cancel(): void {
    this.destroyWorker();
    this.pending?.(new DOMException("已取消起局。", "AbortError"));
  }

  private destroyWorker(): void {
    this.worker?.terminate();
    this.worker = null;
  }
}
