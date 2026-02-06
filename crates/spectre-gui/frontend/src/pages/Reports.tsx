import { FileText } from "lucide-react";

export function Reports() {
  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <FileText className="h-5 w-5 text-primary" />
        <h2 className="text-lg font-semibold">Reports & Findings</h2>
      </div>

      <div className="grid gap-4 lg:grid-cols-2">
        {/* Findings */}
        <div className="rounded-lg border border-border bg-card p-4">
          <h3 className="mb-2 text-sm font-medium text-muted-foreground">Findings</h3>
          <p className="text-sm text-muted-foreground">
            No findings to display. Run a scan to generate findings.
          </p>
        </div>

        {/* Export */}
        <div className="rounded-lg border border-border bg-card p-4">
          <h3 className="mb-2 text-sm font-medium text-muted-foreground">Export</h3>
          <div className="space-y-2">
            {["HTML", "Markdown", "CSV", "JSON"].map((fmt) => (
              <button
                key={fmt}
                className="mr-2 rounded-md border border-border px-3 py-1.5 text-sm text-foreground hover:bg-muted/30"
              >
                {fmt}
              </button>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
