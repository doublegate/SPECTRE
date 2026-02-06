import { StopCircle, Activity } from "lucide-react";
import type { ScanProgressEvent } from "@/types/scan";

interface ScanProgressProps {
  progress: ScanProgressEvent | null;
  onStop: () => void;
  scanning: boolean;
}

function formatRate(pps?: number): string {
  if (!pps) return "—";
  if (pps >= 1000000) return `${(pps / 1000000).toFixed(1)}M pps`;
  if (pps >= 1000) return `${(pps / 1000).toFixed(1)}K pps`;
  return `${pps} pps`;
}

export function ScanProgress({ progress, onStop, scanning }: ScanProgressProps) {
  if (!scanning && !progress) {
    return (
      <div className="rounded-lg border border-border bg-card p-4">
        <div className="flex items-center gap-3 text-muted-foreground">
          <Activity className="h-5 w-5" />
          <span className="text-sm">No active scan</span>
        </div>
      </div>
    );
  }

  return (
    <div className="rounded-lg border border-border bg-card p-4">
      <div className="mb-3 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Activity className="h-5 w-5 animate-pulse text-primary" />
          <span className="text-sm font-medium text-foreground">
            {scanning ? "Scanning..." : "Scan Complete"}
          </span>
        </div>
        {scanning && (
          <button
            onClick={onStop}
            className="flex items-center gap-1.5 rounded-md border border-border bg-background px-3 py-1.5 text-xs font-medium text-foreground hover:bg-muted focus:outline-none focus:ring-2 focus:ring-primary"
          >
            <StopCircle className="h-3.5 w-3.5" />
            Stop
          </button>
        )}
      </div>

      {progress && (
        <>
          <div className="mb-2 overflow-hidden rounded-full bg-muted">
            <div
              className="h-2 bg-primary transition-all duration-300"
              style={{ width: `${progress.percent}%` }}
            />
          </div>

          <div className="grid grid-cols-2 gap-3 text-xs">
            <div>
              <span className="text-muted-foreground">Progress:</span>
              <span className="ml-2 font-medium text-foreground">
                {progress.completed}/{progress.total} ({progress.percent.toFixed(1)}%)
              </span>
            </div>
            <div>
              <span className="text-muted-foreground">Rate:</span>
              <span className="ml-2 font-medium text-foreground">
                {formatRate(progress.rate_pps)}
              </span>
            </div>
            {progress.current_target && (
              <div className="col-span-2">
                <span className="text-muted-foreground">Current:</span>
                <span className="ml-2 font-mono text-foreground">
                  {progress.current_target}
                </span>
              </div>
            )}
          </div>
        </>
      )}
    </div>
  );
}
