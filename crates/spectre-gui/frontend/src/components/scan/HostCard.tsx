import { X, Download, Server, Shield } from "lucide-react";
import type { Host } from "@/types/scan";

interface HostCardProps {
  host: Host;
  onClose: () => void;
}

const SEVERITY_COLORS = {
  critical: "text-red-500 bg-red-500/10 border-red-500/20",
  high: "text-orange-500 bg-orange-500/10 border-orange-500/20",
  medium: "text-yellow-500 bg-yellow-500/10 border-yellow-500/20",
  low: "text-green-500 bg-green-500/10 border-green-500/20",
  info: "text-gray-500 bg-gray-500/10 border-gray-500/20",
};

export function HostCard({ host, onClose }: HostCardProps) {
  const handleExport = () => {
    const json = JSON.stringify(host, null, 2);
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `host_${host.ip}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const openPorts = host.ports.filter((p) => p.state === "open");

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
      <div className="w-full max-w-2xl rounded-lg border border-border bg-card shadow-lg">
        {/* Header */}
        <div className="flex items-center justify-between border-b border-border p-4">
          <div className="flex items-center gap-3">
            <Server className="h-5 w-5 text-primary" />
            <div>
              <h3 className="font-semibold text-foreground">{host.hostname || host.ip}</h3>
              {host.hostname && (
                <p className="text-xs text-muted-foreground">{host.ip}</p>
              )}
            </div>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={handleExport}
              className="flex items-center gap-1.5 rounded-md border border-border bg-background px-3 py-1.5 text-xs font-medium text-foreground hover:bg-muted focus:outline-none focus:ring-2 focus:ring-primary"
            >
              <Download className="h-3.5 w-3.5" />
              Export
            </button>
            <button
              onClick={onClose}
              className="rounded-md p-1.5 text-muted-foreground hover:bg-muted hover:text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
            >
              <X className="h-4 w-4" />
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="max-h-[70vh] overflow-y-auto p-4">
          <div className="mb-4 grid grid-cols-2 gap-4">
            <div className="rounded-md border border-border bg-background p-3">
              <div className="mb-1 text-xs text-muted-foreground">Severity</div>
              <div
                className={`inline-flex items-center gap-1.5 rounded-md border px-2 py-1 text-xs font-medium uppercase ${SEVERITY_COLORS[host.severity]}`}
              >
                <Shield className="h-3 w-3" />
                {host.severity}
              </div>
            </div>
            <div className="rounded-md border border-border bg-background p-3">
              <div className="mb-1 text-xs text-muted-foreground">Open Ports</div>
              <div className="text-lg font-semibold text-foreground">{openPorts.length}</div>
            </div>
          </div>

          {host.os && (
            <div className="mb-4 rounded-md border border-border bg-background p-3">
              <div className="mb-2 text-xs font-medium text-muted-foreground">
                Operating System
              </div>
              <div className="text-sm text-foreground">
                {host.os.name}
                {host.os.version && ` ${host.os.version}`}
              </div>
              <div className="mt-1 text-xs text-muted-foreground">
                Confidence: {host.os.confidence}%
              </div>
            </div>
          )}

          <div className="rounded-md border border-border bg-background">
            <div className="border-b border-border p-3">
              <div className="text-xs font-medium text-muted-foreground">
                Open Ports ({openPorts.length})
              </div>
            </div>
            <div className="max-h-96 overflow-y-auto">
              <table className="w-full text-sm">
                <thead className="sticky top-0 border-b border-border bg-muted/50">
                  <tr className="text-left text-xs text-muted-foreground">
                    <th className="p-2 font-medium">Port</th>
                    <th className="p-2 font-medium">State</th>
                    <th className="p-2 font-medium">Service</th>
                    <th className="p-2 font-medium">Version</th>
                  </tr>
                </thead>
                <tbody>
                  {openPorts.length === 0 ? (
                    <tr>
                      <td colSpan={4} className="p-4 text-center text-muted-foreground">
                        No open ports found
                      </td>
                    </tr>
                  ) : (
                    openPorts.map((port, idx) => (
                      <tr
                        key={idx}
                        className="border-b border-border/50 last:border-0 hover:bg-muted/30"
                      >
                        <td className="p-2 font-mono text-foreground">
                          {port.port}/{port.protocol}
                        </td>
                        <td className="p-2">
                          <span className="rounded-md bg-green-500/10 px-1.5 py-0.5 text-xs font-medium text-green-500">
                            {port.state}
                          </span>
                        </td>
                        <td className="p-2 text-foreground">{port.service || "—"}</td>
                        <td className="p-2 text-muted-foreground">{port.version || "—"}</td>
                      </tr>
                    ))
                  )}
                </tbody>
              </table>
            </div>
          </div>

          {host.ports.some((p) => p.banner) && (
            <div className="mt-4 rounded-md border border-border bg-background p-3">
              <div className="mb-2 text-xs font-medium text-muted-foreground">Banners</div>
              <div className="space-y-2">
                {host.ports
                  .filter((p) => p.banner)
                  .map((port, idx) => (
                    <div key={idx} className="rounded-md bg-muted p-2 font-mono text-xs">
                      <div className="mb-1 text-muted-foreground">
                        {port.port}/{port.protocol}
                      </div>
                      <div className="text-foreground">{port.banner}</div>
                    </div>
                  ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
