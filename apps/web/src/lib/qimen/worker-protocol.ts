import type { Chart, ChartRequest } from "./types";

export interface WorkerRequest {
  type: "calculate";
  id: number;
  request: ChartRequest;
}

export type WorkerResponse =
  | { type: "result"; id: number; chart: Chart }
  | { type: "error"; id: number; message: string };

export function isWorkerRequest(value: unknown): value is WorkerRequest {
  if (typeof value !== "object" || value === null) return false;
  const request = value as Partial<WorkerRequest>;
  return (
    request.type === "calculate" &&
    Number.isSafeInteger(request.id) &&
    typeof request.id === "number" &&
    request.id > 0 &&
    "request" in request
  );
}

/** Keep engine exceptions serializable; errors never masquerade as a successful chart. */
export async function calculateMessage(
  message: WorkerRequest,
  calculate: (request: ChartRequest) => Promise<Chart>,
): Promise<WorkerResponse> {
  try {
    return {
      type: "result",
      id: message.id,
      chart: await calculate(message.request),
    };
  } catch (error) {
    return {
      type: "error",
      id: message.id,
      message:
        error instanceof Error ? error.message : "排盘未能完成，请重试。",
    };
  }
}
