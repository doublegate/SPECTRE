import { FlaskConical } from "lucide-react";

export function Analysis() {
  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <FlaskConical className="h-5 w-5 text-primary" />
        <h2 className="text-lg font-semibold">Data Analysis</h2>
      </div>

      <div className="grid gap-4 lg:grid-cols-2">
        {/* Input */}
        <div className="rounded-lg border border-border bg-card p-4">
          <h3 className="mb-2 text-sm font-medium text-muted-foreground">Input</h3>
          <textarea
            placeholder="Enter data to analyze..."
            className="w-full rounded-md border border-input bg-background p-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
            rows={8}
          />
        </div>

        {/* Output */}
        <div className="rounded-lg border border-border bg-card p-4">
          <h3 className="mb-2 text-sm font-medium text-muted-foreground">Output</h3>
          <div className="min-h-[200px] rounded-md border border-input bg-background p-3 text-sm text-muted-foreground">
            Analysis results will appear here.
          </div>
        </div>
      </div>

      {/* Operations */}
      <div className="rounded-lg border border-border bg-card p-4">
        <h3 className="mb-2 text-sm font-medium text-muted-foreground">
          CyberChef Operations (463 available)
        </h3>
        <p className="text-sm text-muted-foreground">
          Search and execute CyberChef operations via MCP. Supports recipe building,
          batch processing, and result chaining.
        </p>
      </div>
    </div>
  );
}
