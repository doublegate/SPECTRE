import { create } from "zustand";
import type {
  ScanProgressEvent,
  ScanResultEvent,
  ScanCompleteEvent,
  Host,
} from "@/types/scan";
import { calculateSeverity } from "@/types/scan";

interface ScanState {
  // Active scan state (latest scan)
  activeScanId: string | null;
  scanning: boolean;
  progress: ScanProgressEvent | null;

  // Results indexed by scan_id
  hostsByScan: Map<string, Host[]>;

  // Completed scan summaries
  completedScans: Map<string, ScanCompleteEvent>;

  // Errors
  errors: Map<string, string>;

  // Actions
  startScan: (scanId: string) => void;
  setProgress: (progress: ScanProgressEvent) => void;
  addResult: (result: ScanResultEvent) => void;
  setComplete: (summary: ScanCompleteEvent) => void;
  setError: (error: string, scanId: string) => void;
  reset: (scanId?: string) => void;

  // Getters
  getHosts: (scanId?: string) => Host[];
  getAllHosts: () => Host[];
}

export const useScanStore = create<ScanState>((set, get) => ({
  activeScanId: null,
  scanning: false,
  progress: null,
  hostsByScan: new Map(),
  completedScans: new Map(),
  errors: new Map(),

  startScan: (scanId: string) =>
    set({
      activeScanId: scanId,
      scanning: true,
      progress: null,
    }),

  setProgress: (progress: ScanProgressEvent) => {
    // Only update if it's for the active scan
    if (get().activeScanId === progress.scan_id) {
      set({ progress });
    }
  },

  addResult: (result: ScanResultEvent) => {
    const hostsByScan = new Map(get().hostsByScan);
    const hosts = hostsByScan.get(result.scan_id) || [];

    const host: Host = {
      ip: result.host,
      hostname: result.hostname,
      os: result.os,
      ports: result.ports,
      severity: calculateSeverity(result.ports),
      scan_id: result.scan_id,
    };

    hostsByScan.set(result.scan_id, [...hosts, host]);
    set({ hostsByScan });
  },

  setComplete: (summary: ScanCompleteEvent) => {
    const completedScans = new Map(get().completedScans);
    completedScans.set(summary.scan_id, summary);

    set({
      scanning: get().activeScanId === summary.scan_id ? false : get().scanning,
      completedScans,
    });
  },

  setError: (error: string, scanId: string) => {
    const errors = new Map(get().errors);
    errors.set(scanId, error);

    set({
      scanning: get().activeScanId === scanId ? false : get().scanning,
      errors,
    });
  },

  reset: (scanId?: string) => {
    if (scanId) {
      // Reset specific scan
      const hostsByScan = new Map(get().hostsByScan);
      const completedScans = new Map(get().completedScans);
      const errors = new Map(get().errors);

      hostsByScan.delete(scanId);
      completedScans.delete(scanId);
      errors.delete(scanId);

      set({
        hostsByScan,
        completedScans,
        errors,
        scanning: get().activeScanId === scanId ? false : get().scanning,
        progress: get().activeScanId === scanId ? null : get().progress,
        activeScanId: get().activeScanId === scanId ? null : get().activeScanId,
      });
    } else {
      // Reset everything
      set({
        activeScanId: null,
        scanning: false,
        progress: null,
        hostsByScan: new Map(),
        completedScans: new Map(),
        errors: new Map(),
      });
    }
  },

  getHosts: (scanId?: string) => {
    const id = scanId || get().activeScanId;
    if (!id) return [];
    return get().hostsByScan.get(id) || [];
  },

  getAllHosts: () => {
    const allHosts: Host[] = [];
    get().hostsByScan.forEach((hosts) => {
      allHosts.push(...hosts);
    });
    return allHosts;
  },
}));

// Selector hooks for common use cases
export const useActiveHosts = () => {
  const store = useScanStore();
  return store.getHosts();
};

export const useScanProgress = () => {
  const { scanning, progress } = useScanStore();
  return { scanning, progress };
};

export const useScanSummary = (scanId?: string) => {
  const { completedScans, activeScanId } = useScanStore();
  const id = scanId || activeScanId;
  return id ? completedScans.get(id) : undefined;
};
