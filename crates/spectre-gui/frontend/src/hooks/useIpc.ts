import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface IpcState<T> {
  data: T | null;
  loading: boolean;
  error: string | null;
}

/** Base hook wrapping Tauri invoke() with loading/error/data state */
export function useIpc<T, P = void>(command: string) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const execute = useCallback(
    async (params?: P): Promise<T> => {
      setLoading(true);
      setError(null);
      try {
        const result = await invoke<T>(
          command,
          params !== undefined ? (params as Record<string, unknown>) : undefined,
        );
        setData(result);
        return result;
      } catch (e) {
        const msg = typeof e === "string" ? e : String(e);
        setError(msg);
        throw e;
      } finally {
        setLoading(false);
      }
    },
    [command],
  );

  const reset = useCallback(() => {
    setData(null);
    setLoading(false);
    setError(null);
  }, []);

  return { data, loading, error, execute, reset };
}
