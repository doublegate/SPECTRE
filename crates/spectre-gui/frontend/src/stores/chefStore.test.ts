import { describe, it, expect, beforeEach } from "vitest";
import { useChefStore } from "./chefStore";
import type { ChefResult } from "@/types/chef";

describe("chefStore", () => {
  beforeEach(() => {
    useChefStore.getState().reset();
    useChefStore.setState({ operations: [] });
  });

  it("has correct default state", () => {
    const state = useChefStore.getState();
    expect(state.operations).toEqual([]);
    expect(state.lastResult).toBeNull();
    expect(state.processing).toBe(false);
  });

  it("setOperations sets available operations", () => {
    useChefStore.getState().setOperations(["From_Base64", "To_Hex"]);
    expect(useChefStore.getState().operations).toEqual(["From_Base64", "To_Hex"]);
  });

  it("setResult stores result and clears processing", () => {
    useChefStore.getState().setProcessing(true);
    const result: ChefResult = { operation: "From_Base64", output: "decoded data", success: true };
    useChefStore.getState().setResult(result);
    expect(useChefStore.getState().lastResult).toEqual(result);
    expect(useChefStore.getState().processing).toBe(false);
  });

  it("setProcessing toggles processing state", () => {
    useChefStore.getState().setProcessing(true);
    expect(useChefStore.getState().processing).toBe(true);
  });

  it("reset clears result and processing", () => {
    useChefStore.getState().setProcessing(true);
    useChefStore.getState().reset();
    expect(useChefStore.getState().lastResult).toBeNull();
    expect(useChefStore.getState().processing).toBe(false);
  });
});
