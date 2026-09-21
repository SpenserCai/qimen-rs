import { validateRequest, type RequestValidation } from "./request";
import type { ChartRequest } from "./types";

/** Keep calculation input in the fragment, which browsers do not send in HTTP requests. */
export function serializeRequest(request: ChartRequest): string {
  const validated = validateRequest(request);
  if (!validated.ok) throw new Error(validated.error);
  return `#${new URLSearchParams({ q: JSON.stringify(validated.request) }).toString()}`;
}

/** null = no shared chart; an invalid shared chart must be shown as an error, not silently replaced. */
export function restoreRequest(hash: string): RequestValidation | null {
  if (!hash.startsWith("#")) return null;
  if (hash.length > 4096)
    return { ok: false, error: "分享链接过长，请重新生成。" };
  const params = new URLSearchParams(hash.slice(1));
  const values = params.getAll("q");
  if (values.length === 0) return null;
  if (values.length !== 1 || !values[0])
    return { ok: false, error: "分享链接缺少有效的排盘参数。" };
  if ([...params.keys()].some((key) => key !== "q")) {
    return { ok: false, error: "分享链接包含不支持的参数，请重新生成。" };
  }
  try {
    return validateRequest(JSON.parse(values[0]) as unknown);
  } catch {
    return { ok: false, error: "分享链接损坏，无法读取排盘参数。" };
  }
}
