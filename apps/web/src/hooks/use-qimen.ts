"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { validateRequest } from "@/lib/qimen/request";
import { QimenWorkerClient } from "@/lib/qimen/worker-client";
import type { Chart, ChartRequest } from "@/lib/qimen/types";

interface CalculationState {
  status: "idle" | "calculating" | "success" | "error";
  result: Chart | null;
  /** The immutable input for the last successful chart, never the editable form. */
  request: ChartRequest | null;
  error: string | null;
}

const EMPTY_STATE: CalculationState = {
  status: "idle",
  result: null,
  request: null,
  error: null,
};

export function useQimen() {
  const [state, setState] = useState<CalculationState>(EMPTY_STATE);
  const client = useRef<QimenWorkerClient | null>(null);
  const generation = useRef(0);
  const busy = useRef(false);
  const mounted = useRef(true);

  useEffect(() => {
    mounted.current = true;
    return () => {
      mounted.current = false;
      generation.current += 1;
      busy.current = false;
      client.current?.cancel();
    };
  }, []);

  const calculate = useCallback(
    async (request: ChartRequest): Promise<void> => {
      if (busy.current) return;
      const validated = validateRequest(request);
      if (!validated.ok) {
        setState((previous) => ({
          ...previous,
          status: "error",
          error: validated.error,
        }));
        return;
      }
      const current = ++generation.current;
      busy.current = true;
      setState((previous) => ({
        ...previous,
        status: "calculating",
        error: null,
      }));
      try {
        client.current ??= new QimenWorkerClient();
        const result = await client.current.calculate(validated.request);
        if (mounted.current && current === generation.current) {
          setState({
            status: "success",
            result,
            request: validated.request,
            error: null,
          });
        }
      } catch (error) {
        if (mounted.current && current === generation.current) {
          setState((previous) => ({
            ...previous,
            status: "error",
            error:
              error instanceof Error ? error.message : "排盘未能完成，请重试。",
          }));
        }
      } finally {
        if (current === generation.current) busy.current = false;
      }
    },
    [],
  );

  const cancel = useCallback(() => {
    generation.current += 1;
    busy.current = false;
    client.current?.cancel();
    setState((previous) => ({
      ...previous,
      status: previous.result ? "success" : "idle",
      error: null,
    }));
  }, []);

  const reset = useCallback(() => {
    generation.current += 1;
    busy.current = false;
    client.current?.cancel();
    setState(EMPTY_STATE);
  }, []);

  return { ...state, calculate, cancel, reset };
}
