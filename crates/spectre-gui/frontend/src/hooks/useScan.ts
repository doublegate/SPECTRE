import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useIpc } from "./useIpc";
import { useScanStore } from "@/stores/scanStore";
import type { ScanRequest, ScanProgressEvent, ScanResultEvent, ScanCompleteEvent, ScanErrorEvent } from "@/types/scan";

export function useScan() {
  const startScan = useIpc<string, { request: ScanRequest }>("start_scan");
  const stopScan = useIpc<string, { scan_id: string }>("stop_scan");
  const getScanResults = useIpc<unknown[], { scan_id: string }>("get_scan_results");
  const getActiveScans = useIpc<string[]>("get_active_scans");
  const store = useScanStore();

  useEffect(() => {
    const listeners: Promise<() => void>[] = [];

    listeners.push(
      listen<ScanProgressEvent>("scan:progress", (event) => {
        store.setProgress(event.payload);
      }),
    );

    listeners.push(
      listen<ScanResultEvent>("scan:result", (event) => {
        store.addResult(event.payload);
      }),
    );

    listeners.push(
      listen<ScanCompleteEvent>("scan:complete", (event) => {
        store.setComplete(event.payload);
      }),
    );

    listeners.push(
      listen<ScanErrorEvent>("scan:error", (event) => {
        store.setError(event.payload.error, event.payload.scan_id);
      }),
    );

    return () => {
      listeners.forEach((p) => p.then((unlisten) => unlisten()));
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return {
    startScan: async (request: ScanRequest) => {
      const scanId = await startScan.execute({ request });
      if (scanId) {
        store.startScan(scanId);
      }
      return scanId;
    },
    stopScan: async (scanId: string) => {
      return await stopScan.execute({ scan_id: scanId });
    },
    getScanResults: getScanResults.execute,
    getActiveScans: getActiveScans.execute,
    loading: startScan.loading,
    error: startScan.error,
  };
}
