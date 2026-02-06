import { useEffect, useState } from "react";
import { FlagTriangleRight, Plus, Upload } from "lucide-react";
import { useCampaign } from "@/hooks/useCampaign";
import { useCampaignStore } from "@/stores/campaignStore";
import { CampaignCard, CampaignCreateWizard } from "@/components/campaign";
import type { CreateCampaignRequest } from "@/types/campaign";

export function Campaigns() {
  const [wizardOpen, setWizardOpen] = useState(false);
  const { createCampaign, listCampaigns, exportCampaign, archiveCampaign, loading, error } =
    useCampaign();
  const { campaigns, setCampaigns, updateCampaign } = useCampaignStore();

  useEffect(() => {
    // Load campaigns on mount
    const loadCampaigns = async () => {
      try {
        const result = await listCampaigns();
        if (result) {
          setCampaigns(result);
        }
      } catch (err) {
        console.error("Failed to load campaigns:", err);
      }
    };

    loadCampaigns();
  }, [listCampaigns, setCampaigns]);

  const handleCreate = async (request: CreateCampaignRequest) => {
    try {
      const id = await createCampaign({ request });
      if (id) {
        // Reload campaigns list
        const result = await listCampaigns();
        if (result) {
          setCampaigns(result);
        }
        setWizardOpen(false);
      }
    } catch (err) {
      console.error("Failed to create campaign:", err);
      throw err;
    }
  };

  const handleExport = async (id: string) => {
    try {
      // For now, use a default export path (future: file picker)
      const path = `/tmp/campaign-${id}.json`;
      await exportCampaign({ request: { id, path } });
      alert(`Campaign exported to ${path}`);
    } catch (err) {
      console.error("Failed to export campaign:", err);
      alert("Failed to export campaign");
    }
  };

  const handleArchive = async (id: string) => {
    try {
      await archiveCampaign({ id });

      // Update the campaign in the store
      updateCampaign(id, { phase: "archived", status: "complete" });
    } catch (err) {
      console.error("Failed to archive campaign:", err);
      alert("Failed to archive campaign");
    }
  };

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <FlagTriangleRight className="h-5 w-5 text-primary" />
          <h2 className="text-lg font-semibold">Campaign Management</h2>
        </div>
        <div className="flex gap-2">
          <button
            disabled
            className="flex items-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium text-muted-foreground opacity-50"
            title="Import campaign (coming soon)"
          >
            <Upload className="h-4 w-4" />
            Import
          </button>
          <button
            onClick={() => setWizardOpen(true)}
            className="flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90"
          >
            <Plus className="h-4 w-4" />
            New Campaign
          </button>
        </div>
      </div>

      {/* Error state */}
      {error && (
        <div className="rounded-lg border border-destructive/20 bg-destructive/10 p-4">
          <p className="text-sm text-destructive">{error}</p>
        </div>
      )}

      {/* Loading state */}
      {loading && campaigns.length === 0 && (
        <div className="flex items-center justify-center rounded-lg border border-border bg-card p-12">
          <p className="text-sm text-muted-foreground">Loading campaigns...</p>
        </div>
      )}

      {/* Empty state */}
      {!loading && campaigns.length === 0 && (
        <div className="rounded-lg border border-border bg-card p-12 text-center">
          <FlagTriangleRight className="mx-auto h-12 w-12 text-muted" />
          <h3 className="mt-3 text-sm font-medium text-foreground">No campaigns</h3>
          <p className="mt-1 text-sm text-muted-foreground">
            Create a campaign to coordinate multi-phase security operations with target tracking,
            phase management, and artifact collection.
          </p>
          <button
            onClick={() => setWizardOpen(true)}
            className="mt-4 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90"
          >
            Create Your First Campaign
          </button>
        </div>
      )}

      {/* Campaign grid */}
      {campaigns.length > 0 && (
        <div className="grid gap-4 md:grid-cols-2">
          {campaigns.map((campaign) => (
            <CampaignCard
              key={campaign.id}
              campaign={campaign}
              onArchive={handleArchive}
              onExport={handleExport}
            />
          ))}
        </div>
      )}

      {/* Create wizard */}
      <CampaignCreateWizard open={wizardOpen} onClose={() => setWizardOpen(false)} onCreate={handleCreate} />
    </div>
  );
}

// Default export for lazy loading
export default Campaigns;
