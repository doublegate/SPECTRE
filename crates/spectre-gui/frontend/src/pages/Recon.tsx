import { Radar } from "lucide-react";
import { useScanStore } from "@/stores/scanStore";

export function Recon() {
  const { scanning, progress, results } = useScanStore();

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Radar className="h-5 w-5 text-primary" />
        <h2 className="text-lg font-semibold">Reconnaissance</h2>
      </div>

      {/* Scan controls */}
      <div className="rounded-lg border border-border bg-card p-4">
        <div className="flex items-center gap-4">
          <span className="text-sm text-muted-foreground">
            Status: {scanning ? "Scanning..." : "Idle"}
          </span>
          {progress && (
            <div className="flex-1">
              <div className="h-2 overflow-hidden rounded-full bg-muted">
                <div
                  className="h-full bg-primary transition-all"
                  style={{ width: `${progress.percent}%` }}
                />
              </div>
              <p className="mt-1 text-xs text-muted-foreground">
                {progress.completed}/{progress.total} ({progress.percent.toFixed(1)}%)
              </p>
            </div>
          )}
        </div>
      </div>

      {/* Results */}
      <div className="rounded-lg border border-border bg-card p-4">
        <h3 className="mb-2 text-sm font-medium text-muted-foreground">
          Results ({results.length})
        </h3>
        {results.length === 0 ? (
          <p className="text-sm text-muted-foreground">
            No scan results yet. Configure targets and start a scan.
          </p>
        ) : (
          <div className="max-h-96 overflow-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-border text-left text-muted-foreground">
                  <th className="pb-2">Host</th>
                  <th className="pb-2">Port</th>
                  <th className="pb-2">State</th>
                  <th className="pb-2">Service</th>
                </tr>
              </thead>
              <tbody>
                {results.map((r, i) => (
                  <tr key={i} className="border-b border-border/50">
                    <td className="py-1.5">{r.host}</td>
                    <td className="py-1.5">{r.port}/{r.protocol}</td>
                    <td className="py-1.5">
                      <span className={r.state === "open" ? "text-success" : "text-muted-foreground"}>
                        {r.state}
                      </span>
                    </td>
                    <td className="py-1.5 text-muted-foreground">{r.service ?? "-"}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
}
