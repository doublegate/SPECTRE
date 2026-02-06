import { useState } from "react";
import { ChevronLeft, ChevronRight, X } from "lucide-react";
import { ObjectiveList } from "./ObjectiveList";
import { TargetInput } from "./TargetInput";
import type { CreateCampaignRequest } from "@/types/campaign";

interface CampaignCreateWizardProps {
  open: boolean;
  onClose: () => void;
  onCreate: (campaign: CreateCampaignRequest) => Promise<void>;
}

type Step = "name" | "objectives" | "targets" | "review";

export function CampaignCreateWizard({ open, onClose, onCreate }: CampaignCreateWizardProps) {
  const [step, setStep] = useState<Step>("name");
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [objectives, setObjectives] = useState<string[]>([]);
  const [targets, setTargets] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);

  if (!open) return null;

  const steps: Step[] = ["name", "objectives", "targets", "review"];
  const currentStepIndex = steps.indexOf(step);

  const canProceed = (): boolean => {
    switch (step) {
      case "name":
        return name.trim().length > 0;
      case "objectives":
        return objectives.length > 0;
      case "targets":
        return targets.length > 0;
      case "review":
        return true;
      default:
        return false;
    }
  };

  const handleNext = () => {
    if (currentStepIndex < steps.length - 1) {
      setStep(steps[currentStepIndex + 1]);
    }
  };

  const handleBack = () => {
    if (currentStepIndex > 0) {
      setStep(steps[currentStepIndex - 1]);
    }
  };

  const handleCreate = async () => {
    setLoading(true);
    try {
      await onCreate({
        name,
        description: description || undefined,
        objectives: objectives.length > 0 ? objectives : undefined,
        targets: targets.length > 0 ? targets : undefined,
      });
      handleReset();
    } catch (err) {
      console.error("Failed to create campaign:", err);
    } finally {
      setLoading(false);
    }
  };

  const handleReset = () => {
    setStep("name");
    setName("");
    setDescription("");
    setObjectives([]);
    setTargets([]);
    setLoading(false);
    onClose();
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="relative w-full max-w-2xl rounded-lg border border-border bg-background p-6 shadow-lg">
        {/* Header */}
        <div className="mb-6 flex items-center justify-between">
          <h2 className="text-xl font-semibold text-foreground">Create New Campaign</h2>
          <button
            onClick={handleReset}
            className="rounded p-1 text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        {/* Progress bar */}
        <div className="mb-6 flex items-center gap-2">
          {steps.map((s, index) => (
            <div key={s} className="flex flex-1 items-center gap-2">
              <div
                className={`h-2 flex-1 rounded ${
                  index <= currentStepIndex ? "bg-primary" : "bg-muted"
                }`}
              />
              {index < steps.length - 1 && <div className="h-2 w-2 rounded-full bg-muted" />}
            </div>
          ))}
        </div>

        {/* Step content */}
        <div className="min-h-[300px]">
          {/* Step 1: Name & Description */}
          {step === "name" && (
            <div className="space-y-4">
              <div>
                <label htmlFor="campaign-name" className="mb-2 block text-sm font-medium">
                  Campaign Name *
                </label>
                <input
                  id="campaign-name"
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="e.g., Operation NIGHTFALL"
                  className="w-full rounded-md border border-input bg-background px-3 py-2 text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
                  autoFocus
                />
              </div>

              <div>
                <label htmlFor="campaign-desc" className="mb-2 block text-sm font-medium">
                  Description (Optional)
                </label>
                <textarea
                  id="campaign-desc"
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  placeholder="Brief description of campaign goals and scope"
                  rows={4}
                  className="w-full rounded-md border border-input bg-background px-3 py-2 text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
                />
              </div>
            </div>
          )}

          {/* Step 2: Objectives */}
          {step === "objectives" && (
            <ObjectiveList objectives={objectives} editable onChange={setObjectives} />
          )}

          {/* Step 3: Targets */}
          {step === "targets" && <TargetInput onTargetsChange={setTargets} />}

          {/* Step 4: Review */}
          {step === "review" && (
            <div className="space-y-4">
              <h3 className="text-lg font-medium">Review Campaign Details</h3>

              <div className="rounded-md border border-border bg-card p-4 space-y-3">
                <div>
                  <h4 className="text-sm font-medium text-muted-foreground">Name</h4>
                  <p className="text-foreground">{name}</p>
                </div>

                {description && (
                  <div>
                    <h4 className="text-sm font-medium text-muted-foreground">Description</h4>
                    <p className="text-foreground">{description}</p>
                  </div>
                )}

                <div>
                  <h4 className="text-sm font-medium text-muted-foreground">Objectives</h4>
                  <ul className="mt-1 list-inside list-disc space-y-1 text-sm text-foreground">
                    {objectives.map((obj, i) => (
                      <li key={i}>{obj}</li>
                    ))}
                  </ul>
                </div>

                <div>
                  <h4 className="text-sm font-medium text-muted-foreground">Targets</h4>
                  <p className="text-foreground">
                    {targets.length} target{targets.length !== 1 ? "s" : ""} configured
                  </p>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Footer buttons */}
        <div className="mt-6 flex items-center justify-between">
          <button
            onClick={handleBack}
            disabled={currentStepIndex === 0}
            className="flex items-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium text-foreground transition-colors hover:bg-accent disabled:opacity-50"
          >
            <ChevronLeft className="h-4 w-4" />
            Back
          </button>

          <div className="flex gap-2">
            <button
              onClick={handleReset}
              className="rounded-md border border-input bg-background px-4 py-2 text-sm font-medium text-foreground transition-colors hover:bg-accent"
            >
              Cancel
            </button>

            {step === "review" ? (
              <button
                onClick={handleCreate}
                disabled={loading}
                className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-opacity hover:opacity-90 disabled:opacity-50"
              >
                {loading ? "Creating..." : "Create Campaign"}
              </button>
            ) : (
              <button
                onClick={handleNext}
                disabled={!canProceed()}
                className="flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-opacity hover:opacity-90 disabled:opacity-50"
              >
                Next
                <ChevronRight className="h-4 w-4" />
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
