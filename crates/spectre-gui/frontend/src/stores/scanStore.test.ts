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
    expect(state.activeScanId).toBeNull();
    expect(state.hostsByScan.size).toBe(0);
    expect(state.completedScans.size).toBe(0);
    expect(state.errors.size).toBe(0);
  });

  it("startScan sets scanning true and active scan ID", () => {
    useScanStore.getState().startScan("scan_123");
    const state = useScanStore.getState();
    expect(state.scanning).toBe(true);
    expect(state.activeScanId).toBe("scan_123");
    expect(state.progress).toBeNull();
  });

  it("setProgress updates progress for active scan", () => {
    useScanStore.getState().startScan("scan_123");
    const progress: ScanProgressEvent = {
      scan_id: "scan_123",
      completed: 50,
      total: 100,
      percent: 50.0,
      current_target: "192.168.1.1",
    };
    useScanStore.getState().setProgress(progress);
    expect(useScanStore.getState().progress).toEqual(progress);
  });

  it("addResult appends hosts for scan ID", () => {
    const result: ScanResultEvent = {
      scan_id: "scan_123",
      host: "192.168.1.1",
      hostname: "host1",
      ports: [
        {
          port: 80,
          protocol: "tcp",
          state: "open",
          service: "http",
          version: "nginx",
          banner: undefined,
        },
      ],
      os: undefined,
    };
    useScanStore.getState().addResult(result);
    const hosts = useScanStore.getState().getHosts("scan_123");
    expect(hosts).toHaveLength(1);
    expect(hosts[0].ip).toBe("192.168.1.1");
    expect(hosts[0].ports).toHaveLength(1);
  });

  it("setComplete stops scanning and sets summary", () => {
    useScanStore.getState().startScan("scan_123");
    const summary: ScanCompleteEvent = {
      scan_id: "scan_123",
      hosts_scanned: 10,
      open_ports: 5,
      services_found: 3,
      duration_ms: 3000,
    };
    useScanStore.getState().setComplete(summary);
    expect(useScanStore.getState().scanning).toBe(false);
    expect(useScanStore.getState().completedScans.get("scan_123")).toEqual(summary);
  });

  it("setError stops scanning and sets error", () => {
    useScanStore.getState().startScan("scan_123");
    useScanStore.getState().setError("Scan failed", "scan_123");
    expect(useScanStore.getState().scanning).toBe(false);
    expect(useScanStore.getState().errors.get("scan_123")).toBe("Scan failed");
  });

  it("reset clears all state", () => {
    useScanStore.getState().startScan("scan_123");
    useScanStore.getState().setError("err", "scan_123");
    useScanStore.getState().reset();
    const state = useScanStore.getState();
    expect(state.scanning).toBe(false);
    expect(state.activeScanId).toBeNull();
    expect(state.hostsByScan.size).toBe(0);
    expect(state.errors.size).toBe(0);
  });

  it("reset with scan ID clears only that scan", () => {
    useScanStore.getState().startScan("scan_123");
    const result: ScanResultEvent = {
      scan_id: "scan_123",
      host: "192.168.1.1",
      hostname: undefined,
      ports: [],
      os: undefined,
    };
    useScanStore.getState().addResult(result);

    useScanStore.getState().reset("scan_123");
    expect(useScanStore.getState().getHosts("scan_123")).toHaveLength(0);
  });
});
