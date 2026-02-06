import { describe, it, expect, beforeEach } from "vitest";
import { useDashboardStore } from "./dashboardStore";
import type { Pagination, SortOptions, FindingFilters } from "@/types/dashboard";
import { Severity } from "@/types/dashboard";

describe("dashboardStore", () => {
  beforeEach(() => {
    useDashboardStore.getState().reset();
  });

  it("should initialize with default state", () => {
    const state = useDashboardStore.getState();

    expect(state.stats).toBeNull();
    expect(state.statsLoading).toBe(false);
    expect(state.statsError).toBeNull();
    expect(state.findings).toEqual([]);
    expect(state.findingsTotal).toBe(0);
    expect(state.findingsPagination).toEqual({ page: 1, per_page: 25 });
    expect(state.filters).toBeNull();
    expect(state.sort).toBeNull();
  });

  it("should set filters", () => {
    const filters: FindingFilters = {
      severity: [Severity.Critical, Severity.High],
      host: "10.0.0.1",
    };

    useDashboardStore.getState().setFilters(filters);

    const state = useDashboardStore.getState();
    expect(state.filters).toEqual(filters);
  });

  it("should set sort options", () => {
    const sort: SortOptions = {
      field: "severity",
      direction: "desc",
    };

    useDashboardStore.getState().setSort(sort);

    const state = useDashboardStore.getState();
    expect(state.sort).toEqual(sort);
  });

  it("should set pagination", () => {
    const pagination: Pagination = {
      page: 2,
      per_page: 50,
    };

    useDashboardStore.getState().setPagination(pagination);

    const state = useDashboardStore.getState();
    expect(state.findingsPagination).toEqual(pagination);
  });

  it("should reset state", () => {
    const filters: FindingFilters = {
      severity: [Severity.Critical],
    };
    useDashboardStore.getState().setFilters(filters);

    useDashboardStore.getState().reset();

    const state = useDashboardStore.getState();
    expect(state.filters).toBeNull();
    expect(state.stats).toBeNull();
    expect(state.findings).toEqual([]);
    expect(state.findingsPagination).toEqual({ page: 1, per_page: 25 });
  });
});
