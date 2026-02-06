import { useEffect } from "react";
import { useDashboardStore } from "@/stores/dashboardStore";

/**
 * Hook for accessing dashboard statistics with auto-refresh.
 *
 * Automatically fetches dashboard stats on mount and refreshes every 30 seconds.
 * Cleans up on unmount.
 *
 * @param refreshInterval - Refresh interval in ms (default: 30000 = 30s)
 * @returns Dashboard stats, loading state, and error
 */
export function useDashboard(refreshInterval: number = 30000) {
  const { stats, statsLoading, statsError, fetchStats } = useDashboardStore();

  useEffect(() => {
    // Fetch immediately on mount
    fetchStats();

    // Set up auto-refresh if interval > 0
    if (refreshInterval > 0) {
      const interval = setInterval(fetchStats, refreshInterval);
      return () => clearInterval(interval);
    }

    return undefined;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [refreshInterval]);

  return {
    stats,
    loading: statsLoading,
    isLoading: statsLoading,  // Alias for convenience
    error: statsError,
    refresh: fetchStats,
  };
}

/**
 * Hook for accessing findings with filtering, sorting, and pagination.
 *
 * Does NOT auto-fetch on mount. Call `fetchFindings()` explicitly or use
 * `setFilters()`, `setSort()`, or `setPagination()` which trigger fetches automatically.
 *
 * @returns Findings data, filters, loading state, and actions
 */
export function useFindings() {
  const {
    findings,
    findingsTotal,
    findingsPagination,
    findingsLoading,
    findingsError,
    filters,
    sort,
    fetchFindings,
    setFilters,
    setSort,
    setPagination,
  } = useDashboardStore();

  return {
    // Data
    findings,
    total: findingsTotal,
    pagination: findingsPagination,
    filters,
    sort,

    // State
    loading: findingsLoading,
    error: findingsError,

    // Actions
    fetch: fetchFindings,
    setFilters,
    setSort,
    setPagination,
  };
}
