import { describe, it, expect, beforeEach } from "vitest";
import { useUiStore } from "./uiStore";

describe("uiStore", () => {
  beforeEach(() => {
    // Reset store to defaults
    useUiStore.setState({
      theme: "dark",
      sidebarCollapsed: false,
      activeModal: null,
    });
  });

  it("has correct default state", () => {
    const state = useUiStore.getState();
    expect(state.theme).toBe("dark");
    expect(state.sidebarCollapsed).toBe(false);
    expect(state.activeModal).toBeNull();
  });

  it("setTheme changes theme", () => {
    useUiStore.getState().setTheme("matrix");
    expect(useUiStore.getState().theme).toBe("matrix");
  });

  it("setTheme sets data-theme attribute", () => {
    useUiStore.getState().setTheme("tactical");
    expect(document.documentElement.getAttribute("data-theme")).toBe("tactical");
  });

  it("toggleSidebar toggles collapsed state", () => {
    expect(useUiStore.getState().sidebarCollapsed).toBe(false);
    useUiStore.getState().toggleSidebar();
    expect(useUiStore.getState().sidebarCollapsed).toBe(true);
    useUiStore.getState().toggleSidebar();
    expect(useUiStore.getState().sidebarCollapsed).toBe(false);
  });

  it("setSidebarCollapsed sets explicit value", () => {
    useUiStore.getState().setSidebarCollapsed(true);
    expect(useUiStore.getState().sidebarCollapsed).toBe(true);
  });

  it("openModal sets activeModal", () => {
    useUiStore.getState().openModal("settings");
    expect(useUiStore.getState().activeModal).toBe("settings");
  });

  it("closeModal clears activeModal", () => {
    useUiStore.getState().openModal("settings");
    useUiStore.getState().closeModal();
    expect(useUiStore.getState().activeModal).toBeNull();
  });
});
