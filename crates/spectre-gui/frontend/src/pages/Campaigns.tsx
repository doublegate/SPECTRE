import { FlagTriangleRight } from "lucide-react";

export function Campaigns() {
  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <FlagTriangleRight className="h-5 w-5 text-primary" />
          <h2 className="text-lg font-semibold">Campaign Management</h2>
        </div>
        <button className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90">
          New Campaign
        </button>
      </div>

      <div className="rounded-lg border border-border bg-card p-6 text-center">
        <FlagTriangleRight className="mx-auto h-12 w-12 text-muted" />
        <h3 className="mt-3 text-sm font-medium text-foreground">No campaigns</h3>
        <p className="mt-1 text-sm text-muted-foreground">
          Create a campaign to coordinate multi-phase security operations with target
          tracking, phase management, and artifact collection.
        </p>
      </div>
    </div>
  );
}
