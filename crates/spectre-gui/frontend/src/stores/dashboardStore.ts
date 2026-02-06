import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import type {
  DashboardStats,
  FindingSummary,
  FindingFilters,
  SortOptions,
  Pagination,
  FindingsResponse,
} from "@/types/dashboard";

interface DashboardState {
  // Dashboard statistics
  stats: DashboardStats | null;
  statsLoading: boolean;
  statsError: string | null;

  // Findings
  findings: FindingSummary[];
  findingsTotal: number;
  findingsPagination: Pagination;
  findingsLoading: boolean;
  findingsError: string | null;

  // Filters and sorting
  filters: FindingFilters | undefined;
  sort: SortOptions | undefined;

  // Actions
  fetchStats: () => Promise<void>;
  fetchFindings: (
    filters?: FindingFilters,
    sort?: SortOptions,
    pagination?: Pagination
  ) => Promise<void>;
  setFilters: (filters: FindingFilters | undefined) => void;
  setSort: (sort: SortOptions | undefined) => void;
  setPagination: (pagination: Pagination) => void;
  reset: () => void;
}

export const useDashboardStore = create<DashboardState>((set, get) => ({
  stats: null,
  statsLoading: false,
  statsError: null,

  findings: [],
  findingsTotal: 0,
  findingsPagination: { page: 1, per_page: 25 },
  findingsLoading: false,
  findingsError: null,

  filters: undefined,
  sort: undefined,

  fetchStats: async () => {
    set({ statsLoading: true, statsError: null });

    try {
      const stats = await invoke<DashboardStats>("get_dashboard_stats");
      set({ stats, statsLoading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      set({ statsError: message, statsLoading: false });
      console.error("Failed to fetch dashboard stats:", error);
    }
  },

  fetchFindings: async (filters, sort, pagination) => {
    set({ findingsLoading: true, findingsError: null });

    // Update filters/sort/pagination if provided
    const currentFilters = filters ?? get().filters;
    const currentSort = sort ?? get().sort;
    const currentPagination = pagination ?? get().findingsPagination;

    if (filters) set({ filters });
    if (sort) set({ sort });
    if (pagination) set({ findingsPagination: pagination });

    try {
      const response = await invoke<FindingsResponse>("get_findings", {
        filters: currentFilters,
        sort: currentSort,
        pagination: currentPagination,
      });

      set({
        findings: response.findings,
        findingsTotal: response.total,
        findingsPagination: {
          page: response.page,
          per_page: response.per_page,
        },
        findingsLoading: false,
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      set({ findingsError: message, findingsLoading: false });
      console.error("Failed to fetch findings:", error);
    }
  },

  setFilters: (filters) => {
    set({ filters });
    // Reset to page 1 when filters change
    get().fetchFindings(filters, get().sort, { page: 1, per_page: get().findingsPagination.per_page });
  },

  setSort: (sort) => {
    set({ sort });
    // Keep same pagination when sorting
    get().fetchFindings(get().filters, sort, get().findingsPagination);
  },

  setPagination: (pagination) => {
    set({ findingsPagination: pagination });
    get().fetchFindings(get().filters, get().sort, pagination);
  },

  reset: () =>
    set({
      stats: null,
      statsLoading: false,
      statsError: null,
      findings: [],
      findingsTotal: 0,
      findingsPagination: { page: 1, per_page: 25 },
      findingsLoading: false,
      findingsError: null,
      filters: undefined,
      sort: undefined,
    }),
}));
