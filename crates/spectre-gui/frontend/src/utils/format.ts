/** Format byte count to human-readable string */
export function formatBytes(bytes: number, decimals = 1): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(decimals))} ${sizes[i]}`;
}

/** Format duration in milliseconds to human-readable string */
export function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`;
  if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`;
  const min = Math.floor(ms / 60000);
  const sec = Math.round((ms % 60000) / 1000);
  return `${min}m ${sec}s`;
}

/** Format a number with comma separators */
export function formatNumber(n: number): string {
  return n.toLocaleString();
}

/** Format an ISO date string to a short display format */
export function formatDate(iso: string): string {
  const d = new Date(iso);
  return d.toLocaleDateString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

/** Format packets per second */
export function formatRate(pps: number): string {
  if (pps >= 1_000_000) return `${(pps / 1_000_000).toFixed(1)}M pps`;
  if (pps >= 1_000) return `${(pps / 1_000).toFixed(1)}K pps`;
  return `${pps} pps`;
}
