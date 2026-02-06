import { CheckCircle2, Circle } from "lucide-react";
import type { CampaignPhase, CampaignSummary } from "@/types/campaign";

interface PhaseTimelineProps {
  campaign: CampaignSummary;
  onAdvance: () => void;
}

const PHASES: Array<{ key: CampaignPhase; label: string; description: string }> = [
  { key: "planning", label: "Planning", description: "Define objectives and scope" },
  { key: "recon", label: "Reconnaissance", description: "Gather intelligence" },
  { key: "analysis", label: "Analysis", description: "Process gathered data" },
  { key: "exploitation", label: "Exploitation", description: "Active testing" },
  { key: "reporting", label: "Reporting", description: "Document findings" },
  { key: "complete", label: "Complete", description: "Campaign finished" },
];

export function PhaseTimeline({ campaign, onAdvance }: PhaseTimelineProps) {
  const currentPhaseIndex = PHASES.findIndex((p) => p.key === campaign.phase);
  const canAdvance =
    currentPhaseIndex < PHASES.length - 1 &&
    campaign.phase !== "complete" &&
    campaign.phase !== "archived";

  // Check prerequisites for advancing
  const hasPrerequisites = (): { valid: boolean; message?: string } => {
    if (campaign.phase === "planning") {
      if (campaign.objectives.length === 0) {
        return { valid: false, message: "Add at least one objective before advancing" };
      }
      if (campaign.target_count === 0) {
        return { valid: false, message: "Add at least one target before advancing" };
      }
    }
    return { valid: true };
  };

  const prerequisites = hasPrerequisites();

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium text-foreground">Campaign Progress</h3>
        {canAdvance && (
          <button
            onClick={onAdvance}
            disabled={!prerequisites.valid}
            className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-opacity hover:opacity-90 disabled:opacity-50"
            title={prerequisites.message || "Advance to next phase"}
          >
            Advance Phase
          </button>
        )}
      </div>

      {!prerequisites.valid && (
        <div className="rounded-md border border-yellow-500/20 bg-yellow-500/10 p-3">
          <p className="text-sm text-yellow-500">{prerequisites.message}</p>
        </div>
      )}

      <div className="relative">
        {/* Timeline line */}
        <div className="absolute left-4 top-5 h-[calc(100%-1.25rem)] w-0.5 bg-border" />

        {/* Phase steps */}
        <div className="space-y-4">
          {PHASES.map((phase, index) => {
            const isComplete = index < currentPhaseIndex;
            const isCurrent = index === currentPhaseIndex;
            const isNext = index === currentPhaseIndex + 1;

            return (
              <div key={phase.key} className="relative flex items-start gap-4">
                {/* Phase indicator */}
                <div className="relative z-10 flex-shrink-0">
                  {isComplete ? (
                    <CheckCircle2 className="h-8 w-8 text-green-500" />
                  ) : isCurrent ? (
                    <div className="flex h-8 w-8 items-center justify-center rounded-full border-2 border-primary bg-background">
                      <div className="h-3 w-3 animate-pulse rounded-full bg-primary" />
                    </div>
                  ) : (
                    <Circle
                      className={`h-8 w-8 ${isNext ? "text-muted-foreground" : "text-border"}`}
                    />
                  )}
                </div>

                {/* Phase info */}
                <div className="flex-1 pt-0.5">
                  <div className="flex items-center gap-2">
                    <h4
                      className={`font-medium ${
                        isCurrent
                          ? "text-primary"
                          : isComplete
                            ? "text-green-500"
                            : "text-muted-foreground"
                      }`}
                    >
                      {phase.label}
                    </h4>
                    {isCurrent && (
                      <span className="rounded bg-primary/10 px-2 py-0.5 text-xs font-medium text-primary">
                        Current
                      </span>
                    )}
                    {isComplete && (
                      <span className="rounded bg-green-500/10 px-2 py-0.5 text-xs font-medium text-green-500">
                        Complete
                      </span>
                    )}
                  </div>
                  <p
                    className={`mt-1 text-sm ${isCurrent || isComplete ? "text-foreground" : "text-muted-foreground"}`}
                  >
                    {phase.description}
                  </p>
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {campaign.phase === "archived" && (
        <div className="rounded-md border border-border bg-muted p-3">
          <p className="text-sm text-muted-foreground">
            This campaign has been archived and is no longer active.
          </p>
        </div>
      )}
    </div>
  );
}
