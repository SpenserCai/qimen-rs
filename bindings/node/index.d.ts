import type { Chart, ChartRequest } from './schema';

export type * from './schema';

/** Calculate a chart. Invalid requests throw Error with native code InvalidArg. */
export declare function calculate(request: ChartRequest): Chart;

/** Use the exact same JSON schema as the Rust, Python, CLI, MCP and Wasm APIs. */
export declare function calculateJson(requestJson: string): string;
