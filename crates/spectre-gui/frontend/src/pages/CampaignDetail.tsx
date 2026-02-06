import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router";
import { ArrowLeft, Archive, Download } from "lucide-react";
import { useCampaign } from "@/hooks/useCampaign";
import { useCampaignStore } from "@/stores/campaignStore";
import { PhaseTimeline, ObjectiveList } from "@/components/campaign";
import type { CampaignSummary } from "@/types/campaign";

export function CampaignDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { getCampaign, advanceCampaign, exportCampaign, archiveCampaign, loading, error } =
    useCampaign();
  const { updateCampaign } = useCampaignStore();
  const [campaign, setCampaign] = useState<CampaignSummary | null>(null);

  useEffect(() => {
    if (!id) return;

    const loadCampaign = async () => {
      try {
        const result = await getCampaign({ id });
        if (result) {
          setCampaign(result);
        } else {
          // Campaign not found
          navigate("/campaigns");
        }
      } catch (err) {
        console.error("Failed to load campaign:", err);
      }
    };

    loadCampaign();
  }, [id, getCampaign, navigate]);

  const handleAdvance = async () => {
    if (!campaign) return;

    try {
      const newPhase = await advanceCampaign({ id: campaign.id });
      if (newPhase) {
        // Update local state
        const updatedCampaign = { ...campaign, phase: newPhase as CampaignSummary["phase"] };
        setCampaign(updatedCampaign);
        updateCampaign(campaign.id, { phase: newPhase as CampaignSummary["phase"] });
      }
    } catch (err) {
      console.error("Failed to advance campaign:", err);
      alert("Failed to advance campaign phase");
    }
  };

  const handleExport = async () => {
    if (!campaign) return;

    try {
      const path = `/tmp/campaign-${campaign.id}.json`;
      await exportCampaign({ request: { id: campaign.id, path } });
      alert(`Campaign exported to ${path}`);
    } catch (err) {
      console.error("Failed to export campaign:", err);
      alert("Failed to export campaign");
    }
  };

  const handleArchive = async () => {
    if (!campaign) return;

    if (!window.confirm(`Archive campaign "${campaign.name}"? This will mark it as inactive.`)) {
      return;
    }

    try {
      await archiveCampaign({ id: campaign.id });
      updateCampaign(campaign.id, { phase: "archived", status: "complete" });
      navigate("/campaigns");
    } catch (err) {
      console.error("Failed to archive campaign:", err);
      alert("Failed to archive campaign");
    }
  };

  if (loading && !campaign) {
    return (
      <div className="flex items-center justify-center p-12">
        <p className="text-sm text-muted-foreground">Loading campaign...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="rounded-lg border border-destructive/20 bg-destructive/10 p-4">
        <p className="text-sm text-destructive">{error}</p>
      </div>
    );
  }

  if (!campaign) {
    return null;
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <button
            onClick={() => navigate("/campaigns")}
            className="rounded p-1 text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
          >
            <ArrowLeft className="h-5 w-5" />
          </button>
          <div>
            <h2 className="text-lg font-semibold text-foreground">{campaign.name}</h2>
            <p className="text-sm text-muted-foreground">{campaign.description}</p>
          </div>
        </div>

        <div className="flex gap-2">
          <button
            onClick={handleExport}
            className="flex items-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium text-foreground transition-colors hover:bg-accent"
          >
            <Download className="h-4 w-4" />
            Export
          </button>
          <button
            onClick={handleArchive}
            disabled={campaign.phase === "archived"}
            className="flex items-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium text-foreground transition-colors hover:bg-accent disabled:opacity-50"
          >
            <Archive className="h-4 w-4" />
            Archive
          </button>
        </div>
      </div>

      {/* Campaign metadata */}
      <div className="grid gap-4 md:grid-cols-3">
        <div className="rounded-lg border border-border bg-card p-4">
          <p className="text-sm font-medium text-muted-foreground">Status</p>
          <p className="mt-1 text-lg font-semibold capitalize text-foreground">
            {campaign.status}
          </p>
        </div>
        <div className="rounded-lg border border-border bg-card p-4">
          <p className="text-sm font-medium text-muted-foreground">Targets</p>
          <p className="mt-1 text-lg font-semibold text-foreground">{campaign.target_count}</p>
        </div>
        <div className="rounded-lg border border-border bg-card p-4">
          <p className="text-sm font-medium text-muted-foreground">Created</p>
          <p className="mt-1 text-lg font-semibold text-foreground">
            {new Date(campaign.created).toLocaleDateString()}
          </p>
        </div>
      </div>

      {/* Main content */}
      <div className="grid gap-6 lg:grid-cols-2">
        {/* Left column */}
        <div className="space-y-6">
          <div className="rounded-lg border border-border bg-card p-6">
            <PhaseTimeline campaign={campaign} onAdvance={handleAdvance} />
          </div>
        </div>

        {/* Right column */}
        <div className="space-y-6">
          <div className="rounded-lg border border-border bg-card p-6">
            <ObjectiveList objectives={campaign.objectives} editable={false} />
          </div>

          <div className="rounded-lg border border-border bg-card p-6">
            <h3 className="text-sm font-medium text-foreground">Campaign Targets</h3>
            <p className="mt-2 text-muted-foreground">
              {campaign.target_count} target{campaign.target_count !== 1 ? "s" : ""} configured
            </p>
            {/* Future: Display actual target list */}
          </div>

          <div className="rounded-lg border border-border bg-card p-6">
            <h3 className="text-sm font-medium text-foreground">Artifacts</h3>
            <p className="mt-2 text-muted-foreground">
              No artifacts collected yet.
            </p>
            {/* Future: Display artifacts (scan results, findings, etc.) */}
          </div>
        </div>
      </div>
    </div>
  );
}

// Default export for lazy loading
export default CampaignDetail;
