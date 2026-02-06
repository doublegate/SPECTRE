import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ParsedTarget, TargetInput } from "@/types/scan";

export function useTargetParser() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [targets, setTargets] = useState<ParsedTarget[]>([]);

  const parseTargets = async (input: string[]): Promise<ParsedTarget[]> => {
    setLoading(true);
    setError(null);

    try {
      const targetInput: TargetInput = { targets: input };
      const result = await invoke<ParsedTarget[]>("parse_targets", {
        input: targetInput,
      });

      setTargets(result);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const parseFile = async (filePath: string): Promise<ParsedTarget[]> => {
    setLoading(true);
    setError(null);

    try {
      // Read file contents (this would need Tauri file system plugin)
      // For now, we'll just throw an error to implement later
      throw new Error("File parsing not yet implemented - use Tauri filesystem plugin");
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const clear = () => {
    setTargets([]);
    setError(null);
  };

  return {
    targets,
    loading,
    error,
    parseTargets,
    parseFile,
    clear,
  };
}
