import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { useTargetParser } from "./useTargetParser";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("useTargetParser", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("initial state", () => {
    const { result } = renderHook(() => useTargetParser());

    expect(result.current.targets).toEqual([]);
    expect(result.current.loading).toBe(false);
    expect(result.current.error).toBeNull();
  });

  it("parseTargets calls IPC and updates state", async () => {
    const mockResult = [
      { original: "10.0.0.1", expanded: ["10.0.0.1"], count: 1 },
    ];

    vi.mocked(invoke).mockResolvedValue(mockResult);

    const { result } = renderHook(() => useTargetParser());

    await result.current.parseTargets(["10.0.0.1"]);

    await waitFor(() => {
      expect(result.current.targets).toEqual(mockResult);
      expect(result.current.loading).toBe(false);
      expect(result.current.error).toBeNull();
    });
  });

  it("clear resets state", () => {
    const { result } = renderHook(() => useTargetParser());

    result.current.clear();

    expect(result.current.targets).toEqual([]);
    expect(result.current.error).toBeNull();
  });
});
