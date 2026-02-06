import { Crosshair } from "lucide-react";

export function Targets() {
  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Crosshair className="h-5 w-5 text-primary" />
        <h2 className="text-lg font-semibold">Target Management</h2>
      </div>

      <div className="rounded-lg border border-border bg-card p-6">
        <p className="text-sm text-muted-foreground">
          Configure scan targets with CIDR notation, hostname lists, or file import.
          Target parsing and scope enforcement powered by spectre-core.
        </p>

        <div className="mt-4">
          <textarea
            placeholder="Enter targets (one per line)&#10;192.168.1.0/24&#10;10.0.0.1-10&#10;example.com"
            className="w-full rounded-md border border-input bg-background p-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
            rows={6}
          />
        </div>
      </div>
    </div>
  );
}
