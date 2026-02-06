import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useIpc } from "./useIpc";
import { useScanStore } from "@/stores/scanStore";
import type { ScanRequest, ScanProgressEvent, ScanResultEvent, ScanCompleteEvent, ScanErrorEvent } from "@/types/scan";

export function useScan() {
  const startScan = useIpc<string, { request: ScanRequest }>("start_scan");
  const stopScan = useIpc<string>("stop_scan");
  const getScanResults = useIpc<unknown[]>("get_scan_results");
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
        store.setError(event.payload.error);
      }),
    );

    return () => {
      listeners.forEach((p) => p.then((unlisten) => unlisten()));
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return {
    startScan: startScan.execute,
    stopScan: stopScan.execute,
    getScanResults: getScanResults.execute,
    loading: startScan.loading,
    error: startScan.error,
  };
}
