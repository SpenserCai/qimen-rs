import { afterEach, describe, expect, it, vi } from "vitest";
import {
  QimenWorkerClient,
  type CalculationWorker,
} from "../../src/lib/qimen/worker-client";
import {
  calculateMessage,
  isWorkerRequest,
  type WorkerRequest,
  type WorkerResponse,
} from "../../src/lib/qimen/worker-protocol";
import type { Chart } from "../../src/lib/qimen/types";

const request = { year: 2026, month: 9, day: 18, hour: 18 };
// This suite exercises transport identity, not the chart algorithm. The real
// published engine is checked separately and in the browser end-to-end suite.
const chart = { schema_version: "transport-fixture" } as Chart;

class FakeWorker implements CalculationWorker {
  sent: WorkerRequest[] = [];
  terminated = false;
  private messages = new Set<(event: MessageEvent<WorkerResponse>) => void>();
  private errors = new Set<(event: Event) => void>();

  postMessage(message: WorkerRequest) {
    this.sent.push(message);
  }
  terminate() {
    this.terminated = true;
  }

  addEventListener(
    type: "message",
    listener: (event: MessageEvent<WorkerResponse>) => void,
  ): void;
  addEventListener(
    type: "error" | "messageerror",
    listener: (event: Event) => void,
  ): void;
  addEventListener(
    type: string,
    listener:
      | ((event: MessageEvent<WorkerResponse>) => void)
      | ((event: Event) => void),
  ) {
    if (type === "message")
      this.messages.add(
        listener as (event: MessageEvent<WorkerResponse>) => void,
      );
    else this.errors.add(listener as (event: Event) => void);
  }

  removeEventListener(
    type: "message",
    listener: (event: MessageEvent<WorkerResponse>) => void,
  ): void;
  removeEventListener(
    type: "error" | "messageerror",
    listener: (event: Event) => void,
  ): void;
  removeEventListener(
    type: string,
    listener:
      | ((event: MessageEvent<WorkerResponse>) => void)
      | ((event: Event) => void),
  ) {
    if (type === "message")
      this.messages.delete(
        listener as (event: MessageEvent<WorkerResponse>) => void,
      );
    else this.errors.delete(listener as (event: Event) => void);
  }

  receive(response: WorkerResponse) {
    for (const listener of this.messages)
      listener(new MessageEvent("message", { data: response }));
  }

  crash() {
    for (const listener of this.errors) listener(new Event("error"));
  }
}

afterEach(() => vi.useRealTimers());

describe("worker client", () => {
  it("ignores stale messages, returns the matching chart, and reuses an initialized worker", async () => {
    const worker = new FakeWorker();
    const factory = vi.fn(() => worker);
    const client = new QimenWorkerClient(factory);
    const first = client.calculate(request);
    worker.receive({ type: "result", id: 999, chart });
    await expect(client.calculate(request)).rejects.toThrow("正在起局");
    worker.receive({ type: "result", id: 1, chart });
    await expect(first).resolves.toBe(chart);
    const second = client.calculate(request);
    worker.receive({ type: "result", id: 2, chart });
    await expect(second).resolves.toBe(chart);
    expect(factory).toHaveBeenCalledTimes(1);
    client.cancel();
  });

  it("terminates a cancelled request and starts a fresh worker for a retry", async () => {
    const firstWorker = new FakeWorker();
    const nextWorker = new FakeWorker();
    const factory = vi
      .fn()
      .mockReturnValueOnce(firstWorker)
      .mockReturnValue(nextWorker);
    const client = new QimenWorkerClient(factory);
    const first = client.calculate(request);
    const rejected = expect(first).rejects.toMatchObject({
      name: "AbortError",
    });
    client.cancel();
    await rejected;
    expect(firstWorker.terminated).toBe(true);
    const second = client.calculate(request);
    firstWorker.receive({ type: "result", id: 1, chart });
    nextWorker.receive({ type: "result", id: 2, chart });
    await expect(second).resolves.toBe(chart);
    client.cancel();
  });

  it("replaces failed imports on explicit retry instead of retaining the browser module cache", async () => {
    const workers = [new FakeWorker(), new FakeWorker()];
    let index = 0;
    const client = new QimenWorkerClient(() => workers[index++]!);
    const pending = client.calculate(request);
    workers[0]!.receive({ type: "error", id: 1, message: "加载失败" });
    await expect(pending).rejects.toThrow("加载失败");
    expect(workers[0]!.terminated).toBe(true);
    const retried = client.calculate(request);
    workers[1]!.receive({ type: "result", id: 2, chart });
    await expect(retried).resolves.toBe(chart);
    client.cancel();
  });

  it("surfaces worker crashes and times out an unresponsive load", async () => {
    vi.useFakeTimers();
    const worker = new FakeWorker();
    const client = new QimenWorkerClient(() => worker, 1000);
    const pending = client.calculate(request);
    const rejected = expect(pending).rejects.toThrow("加载超时");
    await vi.advanceTimersByTimeAsync(1000);
    await rejected;
    expect(worker.terminated).toBe(true);

    const crashing = new FakeWorker();
    const crashed = new QimenWorkerClient(() => crashing).calculate(request);
    crashing.crash();
    await expect(crashed).rejects.toThrow("意外中断");
    expect(crashing.terminated).toBe(true);
  });
});

describe("worker protocol", () => {
  it("preserves request IDs across both successful and failed engine calls", async () => {
    const message: WorkerRequest = { type: "calculate", id: 37, request };
    await expect(calculateMessage(message, async () => chart)).resolves.toEqual(
      { type: "result", id: 37, chart },
    );
    await expect(
      calculateMessage(message, async () => {
        throw new Error("Invalid date");
      }),
    ).resolves.toEqual({ type: "error", id: 37, message: "Invalid date" });
  });

  it("ignores malformed worker envelopes", () => {
    for (const message of [
      null,
      "hello",
      {},
      { type: "calculate", id: -1, request },
      { type: "calculate", id: 1.5, request },
      { type: "result", id: 1, request },
    ]) {
      expect(isWorkerRequest(message)).toBe(false);
    }
    expect(isWorkerRequest({ type: "calculate", id: 1, request })).toBe(true);
  });
});
