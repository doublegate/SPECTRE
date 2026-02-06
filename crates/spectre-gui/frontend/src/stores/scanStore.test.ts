import { describe, it, expect, beforeEach } from "vitest";
import { useScanStore } from "./scanStore";
import type { ScanProgressEvent, ScanResultEvent, ScanCompleteEvent } from "@/types/scan";

describe("scanStore", () => {
  beforeEach(() => {
    useScanStore.getState().reset();
  });

  it("has correct default state", () => {
    const state = useScanStore.getState();
    expect(state.scanning).toBe(false);
    expect(state.progress).toBeNull();
    expect(state.results).toEqual([]);
    expect(state.summary).toBeNull();
    expect(state.error).toBeNull();
  });

  it("startScanning resets and sets scanning true", () => {
    useScanStore.getState().setError("old error");
    useScanStore.getState().startScanning();
    const state = useScanStore.getState();
    expect(state.scanning).toBe(true);
    expect(state.error).toBeNull();
    expect(state.results).toEqual([]);
  });

  it("setProgress updates progress", () => {
    const progress: ScanProgressEvent = {
      completed: 50,
      total: 100,
      percent: 50.0,
      current_target: "192.168.1.1",
    };
    useScanStore.getState().setProgress(progress);
    expect(useScanStore.getState().progress).toEqual(progress);
  });

  it("addResult appends to results", () => {
    const result: ScanResultEvent = {
      host: "192.168.1.1",
      port: 80,
      protocol: "tcp",
      state: "open",
      service: "http",
    };
    useScanStore.getState().addResult(result);
    useScanStore.getState().addResult({ ...result, port: 443, service: "https" });
    expect(useScanStore.getState().results).toHaveLength(2);
    expect(useScanStore.getState().results[0].port).toBe(80);
    expect(useScanStore.getState().results[1].port).toBe(443);
  });

  it("setComplete stops scanning and sets summary", () => {
    useScanStore.getState().startScanning();
    const summary: ScanCompleteEvent = {
      hosts_scanned: 10,
      open_ports: 5,
      duration_ms: 3000,
    };
    useScanStore.getState().setComplete(summary);
    expect(useScanStore.getState().scanning).toBe(false);
    expect(useScanStore.getState().summary).toEqual(summary);
  });

  it("setError stops scanning and sets error", () => {
    useScanStore.getState().startScanning();
    useScanStore.getState().setError("Scan failed");
    expect(useScanStore.getState().scanning).toBe(false);
    expect(useScanStore.getState().error).toBe("Scan failed");
  });

  it("reset clears all state", () => {
    useScanStore.getState().startScanning();
    useScanStore.getState().setError("err");
    useScanStore.getState().reset();
    const state = useScanStore.getState();
    expect(state.scanning).toBe(false);
    expect(state.error).toBeNull();
  });
});
