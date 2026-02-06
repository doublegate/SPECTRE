import { useState, useMemo } from "react";
import { ChevronUp, ChevronDown, Download } from "lucide-react";
import type { Host, Severity } from "@/types/scan";

interface ResultsTableProps {
  hosts: Host[];
  onHostClick: (host: Host) => void;
}

type SortField = "ip" | "hostname" | "ports" | "severity";
type SortDirection = "asc" | "desc";

const SEVERITY_ORDER: Record<Severity, number> = {
  critical: 0,
  high: 1,
  medium: 2,
  low: 3,
  info: 4,
};

const SEVERITY_COLORS = {
  critical: "text-red-500 bg-red-500/10",
  high: "text-orange-500 bg-orange-500/10",
  medium: "text-yellow-500 bg-yellow-500/10",
  low: "text-green-500 bg-green-500/10",
  info: "text-gray-500 bg-gray-500/10",
};

export function ResultsTable({ hosts, onHostClick }: ResultsTableProps) {
  const [sortField, setSortField] = useState<SortField>("severity");
  const [sortDirection, setSortDirection] = useState<SortDirection>("asc");
  const [filterSeverity, setFilterSeverity] = useState<Severity | "all">("all");
  const [filterService, setFilterService] = useState("");
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(10);

  const handleSort = (field: SortField) => {
    if (sortField === field) {
      setSortDirection(sortDirection === "asc" ? "desc" : "asc");
    } else {
      setSortField(field);
      setSortDirection("asc");
    }
  };

  const filteredAndSortedHosts = useMemo(() => {
    let filtered = hosts;

    // Filter by severity
    if (filterSeverity !== "all") {
      filtered = filtered.filter((h) => h.severity === filterSeverity);
    }

    // Filter by service
    if (filterService) {
      const searchLower = filterService.toLowerCase();
      filtered = filtered.filter((h) =>
        h.ports.some((p) => p.service?.toLowerCase().includes(searchLower))
      );
    }

    // Sort
    const sorted = [...filtered].sort((a, b) => {
      let comparison = 0;

      switch (sortField) {
        case "ip":
          comparison = a.ip.localeCompare(b.ip);
          break;
        case "hostname":
          comparison = (a.hostname || "").localeCompare(b.hostname || "");
          break;
        case "ports":
          comparison = a.ports.length - b.ports.length;
          break;
        case "severity":
          comparison = SEVERITY_ORDER[a.severity] - SEVERITY_ORDER[b.severity];
          break;
      }

      return sortDirection === "asc" ? comparison : -comparison;
    });

    return sorted;
  }, [hosts, sortField, sortDirection, filterSeverity, filterService]);

  const paginatedHosts = useMemo(() => {
    const start = page * pageSize;
    return filteredAndSortedHosts.slice(start, start + pageSize);
  }, [filteredAndSortedHosts, page, pageSize]);

  const totalPages = Math.ceil(filteredAndSortedHosts.length / pageSize);

  const handleExportAll = (format: "csv" | "json") => {
    if (format === "json") {
      const json = JSON.stringify(filteredAndSortedHosts, null, 2);
      const blob = new Blob([json], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `scan_results_${Date.now()}.json`;
      a.click();
      URL.revokeObjectURL(url);
    } else {
      // CSV export
      const headers = ["IP", "Hostname", "OS", "Open Ports", "Services", "Severity"];
      const rows = filteredAndSortedHosts.map((h) => [
        h.ip,
        h.hostname || "",
        h.os ? `${h.os.name} ${h.os.version || ""}` : "",
        h.ports.filter((p) => p.state === "open").length.toString(),
        h.ports
          .filter((p) => p.service)
          .map((p) => p.service)
          .join("; "),
        h.severity,
      ]);

      const csv = [headers, ...rows].map((row) => row.join(",")).join("\n");
      const blob = new Blob([csv], { type: "text/csv" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `scan_results_${Date.now()}.csv`;
      a.click();
      URL.revokeObjectURL(url);
    }
  };

  const SortIcon = ({ field }: { field: SortField }) => {
    if (sortField !== field) return null;
    return sortDirection === "asc" ? (
      <ChevronUp className="h-3 w-3" />
    ) : (
      <ChevronDown className="h-3 w-3" />
    );
  };

  return (
    <div className="rounded-lg border border-border bg-card">
      {/* Header with filters */}
      <div className="flex items-center justify-between border-b border-border p-4">
        <div className="flex items-center gap-3">
          <select
            value={filterSeverity}
            onChange={(e) => setFilterSeverity(e.target.value as Severity | "all")}
            className="rounded-md border border-border bg-background px-3 py-1.5 text-xs text-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
          >
            <option value="all">All Severities</option>
            <option value="critical">Critical</option>
            <option value="high">High</option>
            <option value="medium">Medium</option>
            <option value="low">Low</option>
            <option value="info">Info</option>
          </select>

          <input
            type="text"
            value={filterService}
            onChange={(e) => setFilterService(e.target.value)}
            placeholder="Filter by service..."
            className="w-40 rounded-md border border-border bg-background px-3 py-1.5 text-xs text-foreground placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
          />

          <span className="text-xs text-muted-foreground">
            {filteredAndSortedHosts.length} host{filteredAndSortedHosts.length !== 1 ? "s" : ""}
          </span>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={() => handleExportAll("csv")}
            className="flex items-center gap-1.5 rounded-md border border-border bg-background px-3 py-1.5 text-xs font-medium text-foreground hover:bg-muted focus:outline-none focus:ring-2 focus:ring-primary"
          >
            <Download className="h-3.5 w-3.5" />
            CSV
          </button>
          <button
            onClick={() => handleExportAll("json")}
            className="flex items-center gap-1.5 rounded-md border border-border bg-background px-3 py-1.5 text-xs font-medium text-foreground hover:bg-muted focus:outline-none focus:ring-2 focus:ring-primary"
          >
            <Download className="h-3.5 w-3.5" />
            JSON
          </button>
        </div>
      </div>

      {/* Table */}
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead className="border-b border-border bg-muted/50">
            <tr className="text-left text-xs text-muted-foreground">
              <th
                onClick={() => handleSort("ip")}
                className="cursor-pointer p-3 font-medium hover:text-foreground"
              >
                <div className="flex items-center gap-1">
                  Host <SortIcon field="ip" />
                </div>
              </th>
              <th
                onClick={() => handleSort("hostname")}
                className="cursor-pointer p-3 font-medium hover:text-foreground"
              >
                <div className="flex items-center gap-1">
                  Hostname <SortIcon field="hostname" />
                </div>
              </th>
              <th className="p-3 font-medium">OS</th>
              <th
                onClick={() => handleSort("ports")}
                className="cursor-pointer p-3 font-medium hover:text-foreground"
              >
                <div className="flex items-center gap-1">
                  Open Ports <SortIcon field="ports" />
                </div>
              </th>
              <th className="p-3 font-medium">Services</th>
              <th
                onClick={() => handleSort("severity")}
                className="cursor-pointer p-3 font-medium hover:text-foreground"
              >
                <div className="flex items-center gap-1">
                  Severity <SortIcon field="severity" />
                </div>
              </th>
            </tr>
          </thead>
          <tbody>
            {paginatedHosts.length === 0 ? (
              <tr>
                <td colSpan={6} className="p-8 text-center text-muted-foreground">
                  No hosts found matching filters
                </td>
              </tr>
            ) : (
              paginatedHosts.map((host, idx) => (
                <tr
                  key={idx}
                  onClick={() => onHostClick(host)}
                  className="cursor-pointer border-b border-border/50 last:border-0 hover:bg-muted/30"
                >
                  <td className="p-3 font-mono text-foreground">{host.ip}</td>
                  <td className="p-3 text-foreground">{host.hostname || "—"}</td>
                  <td className="p-3 text-muted-foreground">
                    {host.os ? `${host.os.name} ${host.os.version || ""}`.trim() : "—"}
                  </td>
                  <td className="p-3 text-foreground">
                    {host.ports.filter((p) => p.state === "open").length}
                  </td>
                  <td className="max-w-xs truncate p-3 text-muted-foreground">
                    {host.ports
                      .filter((p) => p.service)
                      .map((p) => p.service)
                      .join(", ") || "—"}
                  </td>
                  <td className="p-3">
                    <span
                      className={`inline-block rounded-md px-2 py-1 text-xs font-medium uppercase ${SEVERITY_COLORS[host.severity]}`}
                    >
                      {host.severity}
                    </span>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      {totalPages > 1 && (
        <div className="flex items-center justify-between border-t border-border p-3">
          <div className="flex items-center gap-2">
            <span className="text-xs text-muted-foreground">Rows per page:</span>
            <select
              value={pageSize}
              onChange={(e) => {
                setPageSize(Number(e.target.value));
                setPage(0);
              }}
              className="rounded-md border border-border bg-background px-2 py-1 text-xs text-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
            >
              <option value={10}>10</option>
              <option value={25}>25</option>
              <option value={50}>50</option>
              <option value={100}>100</option>
            </select>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setPage(Math.max(0, page - 1))}
              disabled={page === 0}
              className="rounded-md border border-border bg-background px-3 py-1 text-xs font-medium text-foreground hover:bg-muted focus:outline-none focus:ring-2 focus:ring-primary disabled:cursor-not-allowed disabled:opacity-50"
            >
              Previous
            </button>
            <span className="text-xs text-muted-foreground">
              Page {page + 1} of {totalPages}
            </span>
            <button
              onClick={() => setPage(Math.min(totalPages - 1, page + 1))}
              disabled={page >= totalPages - 1}
              className="rounded-md border border-border bg-background px-3 py-1 text-xs font-medium text-foreground hover:bg-muted focus:outline-none focus:ring-2 focus:ring-primary disabled:cursor-not-allowed disabled:opacity-50"
            >
              Next
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
