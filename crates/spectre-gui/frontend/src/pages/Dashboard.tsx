import { useStatus } from "@/hooks/useStatus";
import { cn } from "@/lib/utils";

export function Dashboard() {
  const { version, status, loading } = useStatus();

  return (
    <div className="space-y-6">
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <StatCard title="Hosts Scanned" value="0" />
        <StatCard title="Open Ports" value="0" />
        <StatCard title="Services" value="0" />
        <StatCard title="Findings" value="0" />
      </div>

      <div className="grid gap-4 lg:grid-cols-3">
        {/* Component status */}
        <div className="rounded-lg border border-border bg-card p-4 lg:col-span-2">
          <h2 className="mb-3 text-sm font-medium text-muted-foreground">
            Component Status
          </h2>
          {loading && <p className="text-sm text-muted-foreground">Loading...</p>}
          {status?.components.map((c) => (
            <div
              key={c.name}
              className="flex items-center justify-between border-b border-border py-2 last:border-0"
            >
              <span className="text-sm text-foreground">{c.name}</span>
              <span
                className={cn(
                  "rounded-full px-2 py-0.5 text-xs font-medium",
                  c.status === "available"
                    ? "bg-success/20 text-success"
                    : "bg-warning/20 text-warning",
                )}
              >
                {c.status}
              </span>
            </div>
          ))}
        </div>

        {/* Version info */}
        <div className="rounded-lg border border-border bg-card p-4">
          <h2 className="mb-3 text-sm font-medium text-muted-foreground">
            Platform Info
          </h2>
          <dl className="space-y-2 text-sm">
            <div className="flex justify-between">
              <dt className="text-muted-foreground">Version</dt>
              <dd className="text-foreground">{version?.version ?? "..."}</dd>
            </div>
            <div className="flex justify-between">
              <dt className="text-muted-foreground">Name</dt>
              <dd className="text-foreground">{version?.name ?? "..."}</dd>
            </div>
          </dl>
        </div>
      </div>
    </div>
  );
}

function StatCard({ title, value }: { title: string; value: string }) {
  return (
    <div className="rounded-lg border border-border bg-card p-4">
      <p className="text-xs font-medium text-muted-foreground">{title}</p>
      <p className="mt-1 text-2xl font-bold text-foreground">{value}</p>
    </div>
  );
}
