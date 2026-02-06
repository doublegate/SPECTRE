import { Radio } from "lucide-react";

export function Comms() {
  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Radio className="h-5 w-5 text-primary" />
        <h2 className="text-lg font-semibold">Secure Communications</h2>
      </div>

      <div className="grid gap-4 lg:grid-cols-3">
        {/* Identity */}
        <div className="rounded-lg border border-border bg-card p-4">
          <h3 className="mb-2 text-sm font-medium text-muted-foreground">Identity</h3>
          <p className="text-sm text-muted-foreground">
            No identity configured. Generate or import a cryptographic identity to
            enable secure communications.
          </p>
        </div>

        {/* Peers */}
        <div className="rounded-lg border border-border bg-card p-4">
          <h3 className="mb-2 text-sm font-medium text-muted-foreground">Peers</h3>
          <p className="text-sm text-muted-foreground">No peers configured.</p>
        </div>

        {/* Transfer */}
        <div className="rounded-lg border border-border bg-card p-4">
          <h3 className="mb-2 text-sm font-medium text-muted-foreground">
            Transfer History
          </h3>
          <p className="text-sm text-muted-foreground">No transfers yet.</p>
        </div>
      </div>
    </div>
  );
}
