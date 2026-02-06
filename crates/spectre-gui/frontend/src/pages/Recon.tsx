import { useState } from "react";
import { Radar, Network, Table2, ChevronDown, ChevronUp } from "lucide-react";
import { useScan } from "@/hooks/useScan";
import { useScanStore, useActiveHosts, useScanProgress } from "@/stores/scanStore";
import { NetworkTopology } from "@/components/scan/NetworkTopology";
import { ScanConfigForm } from "@/components/scan/ScanConfigForm";
import { ScanProgress } from "@/components/scan/ScanProgress";
import { HostCard } from "@/components/scan/HostCard";
import { ResultsTable } from "@/components/scan/ResultsTable";
import type { Host, ScanRequest } from "@/types/scan";

type ViewMode = "topology" | "table";

export function Recon() {
  const { startScan, stopScan, loading } = useScan();
  const { scanning, progress } = useScanProgress();
  const hosts = useActiveHosts();
  const { activeScanId } = useScanStore();

  const [selectedHost, setSelectedHost] = useState<Host | null>(null);
  const [viewMode, setViewMode] = useState<ViewMode>("topology");
  const [configCollapsed, setConfigCollapsed] = useState(false);
  const [tableCollapsed, setTableCollapsed] = useState(false);

  const handleScanStart = async (request: ScanRequest) => {
    try {
      await startScan(request);
    } catch (error) {
      console.error("Failed to start scan:", error);
    }
  };

  const handleScanStop = async () => {
    if (activeScanId) {
      try {
        await stopScan(activeScanId);
      } catch (error) {
        console.error("Failed to stop scan:", error);
      }
    }
  };

  const handleHostClick = (host: Host) => {
    setSelectedHost(host);
  };

  return (
    <div className="flex h-full flex-col gap-4">
      {/* Header */}
      <div className="flex items-center gap-3">
        <Radar className="h-5 w-5 text-primary" />
        <h2 className="text-lg font-semibold">Network Reconnaissance</h2>
        {hosts.length > 0 && (
          <span className="rounded-md bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">
            {hosts.length} host{hosts.length !== 1 ? "s" : ""} discovered
          </span>
        )}
      </div>

      {/* Main content area with 2-column layout */}
      <div className="grid flex-1 grid-cols-1 gap-4 overflow-hidden lg:grid-cols-3">
        {/* Left column - Configuration and Progress */}
        <div className="flex flex-col gap-4 overflow-y-auto lg:col-span-1">
          {/* Scan Configuration */}
          <div className="rounded-lg border border-border bg-card">
            <button
              onClick={() => setConfigCollapsed(!configCollapsed)}
              className="flex w-full items-center justify-between p-4 text-left hover:bg-muted/30"
            >
              <span className="text-sm font-medium text-foreground">Scan Configuration</span>
              {configCollapsed ? (
                <ChevronDown className="h-4 w-4 text-muted-foreground" />
              ) : (
                <ChevronUp className="h-4 w-4 text-muted-foreground" />
              )}
            </button>
            {!configCollapsed && (
              <div className="border-t border-border p-4">
                <ScanConfigForm onScanStart={handleScanStart} loading={loading || scanning} />
              </div>
            )}
          </div>

          {/* Scan Progress */}
          <ScanProgress progress={progress} onStop={handleScanStop} scanning={scanning} />
        </div>

        {/* Right column - Visualization */}
        <div className="flex flex-col gap-4 overflow-hidden lg:col-span-2">
          {/* View mode toggle */}
          <div className="flex items-center gap-2 rounded-lg border border-border bg-card p-2">
            <button
              onClick={() => setViewMode("topology")}
              className={`flex flex-1 items-center justify-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-colors ${
                viewMode === "topology"
                  ? "bg-primary text-primary-foreground"
                  : "text-muted-foreground hover:bg-muted hover:text-foreground"
              }`}
            >
              <Network className="h-4 w-4" />
              Network Topology
            </button>
            <button
              onClick={() => setViewMode("table")}
              className={`flex flex-1 items-center justify-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-colors ${
                viewMode === "table"
                  ? "bg-primary text-primary-foreground"
                  : "text-muted-foreground hover:bg-muted hover:text-foreground"
              }`}
            >
              <Table2 className="h-4 w-4" />
              Table View
            </button>
          </div>

          {/* Main visualization area */}
          <div className="min-h-0 flex-1">
            {viewMode === "topology" ? (
              <NetworkTopology hosts={hosts} onHostClick={handleHostClick} />
            ) : (
              <div className="h-full overflow-auto">
                <ResultsTable hosts={hosts} onHostClick={handleHostClick} />
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Bottom results table (collapsible) when in topology mode */}
      {viewMode === "topology" && hosts.length > 0 && (
        <div className="rounded-lg border border-border bg-card">
          <button
            onClick={() => setTableCollapsed(!tableCollapsed)}
            className="flex w-full items-center justify-between p-3 text-left hover:bg-muted/30"
          >
            <div className="flex items-center gap-2">
              <Table2 className="h-4 w-4 text-muted-foreground" />
              <span className="text-sm font-medium text-foreground">Detailed Results</span>
              <span className="text-xs text-muted-foreground">
                ({hosts.length} host{hosts.length !== 1 ? "s" : ""})
              </span>
            </div>
            {tableCollapsed ? (
              <ChevronUp className="h-4 w-4 text-muted-foreground" />
            ) : (
              <ChevronDown className="h-4 w-4 text-muted-foreground" />
            )}
          </button>
          {!tableCollapsed && (
            <div className="border-t border-border">
              <ResultsTable hosts={hosts} onHostClick={handleHostClick} />
            </div>
          )}
        </div>
      )}

      {/* Host detail modal */}
      {selectedHost && <HostCard host={selectedHost} onClose={() => setSelectedHost(null)} />}
    </div>
  );
}

// Default export for lazy loading
export default Recon;
