import { create } from "zustand";
import type { ScanProgressEvent, ScanResultEvent, ScanCompleteEvent } from "@/types/scan";

interface ScanState {
  scanning: boolean;
  progress: ScanProgressEvent | null;
  results: ScanResultEvent[];
  summary: ScanCompleteEvent | null;
  error: string | null;

  setProgress: (progress: ScanProgressEvent) => void;
  addResult: (result: ScanResultEvent) => void;
  setComplete: (summary: ScanCompleteEvent) => void;
  setError: (error: string) => void;
  reset: () => void;
  startScanning: () => void;
}

export const useScanStore = create<ScanState>((set) => ({
  scanning: false,
  progress: null,
  results: [],
  summary: null,
  error: null,

  startScanning: () =>
    set({ scanning: true, progress: null, results: [], summary: null, error: null }),

  setProgress: (progress) => set({ progress }),

  addResult: (result) =>
    set((state) => ({ results: [...state.results, result] })),

  setComplete: (summary) => set({ scanning: false, summary }),

  setError: (error) => set({ scanning: false, error }),

  reset: () =>
    set({ scanning: false, progress: null, results: [], summary: null, error: null }),
}));
