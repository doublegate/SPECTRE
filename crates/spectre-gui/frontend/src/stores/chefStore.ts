import { create } from "zustand";
import type { ChefResult } from "@/types/chef";

interface ChefState {
  operations: string[];
  lastResult: ChefResult | null;
  processing: boolean;

  setOperations: (ops: string[]) => void;
  setResult: (result: ChefResult) => void;
  setProcessing: (processing: boolean) => void;
  reset: () => void;
}

export const useChefStore = create<ChefState>((set) => ({
  operations: [],
  lastResult: null,
  processing: false,

  setOperations: (operations) => set({ operations }),
  setResult: (result) => set({ lastResult: result, processing: false }),
  setProcessing: (processing) => set({ processing }),
  reset: () => set({ lastResult: null, processing: false }),
}));
