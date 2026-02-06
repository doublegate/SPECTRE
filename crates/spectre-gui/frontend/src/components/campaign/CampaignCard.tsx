import { useNavigate } from "react-router";
import { Archive, Download, FlagTriangleRight } from "lucide-react";
import type { CampaignSummary } from "@/types/campaign";

interface CampaignCardProps {
  campaign: CampaignSummary;
  onArchive: (id: string) => void;
  onExport: (id: string) => void;
}

export function CampaignCard({ campaign, onArchive, onExport }: CampaignCardProps) {
  const navigate = useNavigate();

  const getPhaseColor = (phase: string) => {
    switch (phase) {
      case "planning":
        return "text-blue-500";
      case "recon":
        return "text-yellow-500";
      case "analysis":
        return "text-orange-500";
      case "exploitation":
        return "text-red-500";
      case "reporting":
        return "text-purple-500";
      case "complete":
        return "text-green-500";
      case "archived":
        return "text-gray-500";
      default:
        return "text-muted-foreground";
    }
  };

  const getStatusBadge = (status: string) => {
    const colors: Record<string, string> = {
      planning: "bg-blue-500/10 text-blue-500",
      active: "bg-green-500/10 text-green-500",
      complete: "bg-gray-500/10 text-gray-500",
    };

    return (
      <span
        className={`rounded px-2 py-0.5 text-xs font-medium ${colors[status] || "bg-muted text-muted-foreground"}`}
      >
        {status}
      </span>
    );
  };

  return (
    <div
      className="group cursor-pointer rounded-lg border border-border bg-card p-4 transition-colors hover:bg-accent"
      onClick={() => navigate(`/campaigns/${campaign.id}`)}
    >
      <div className="mb-3 flex items-start justify-between">
        <div className="flex items-center gap-2">
          <FlagTriangleRight className={`h-5 w-5 ${getPhaseColor(campaign.phase)}`} />
          <h3 className="font-semibold text-foreground">{campaign.name}</h3>
        </div>
        <div className="flex items-center gap-2">
          {getStatusBadge(campaign.status)}
          <button
            onClick={(e) => {
              e.stopPropagation();
              onExport(campaign.id);
            }}
            className="rounded p-1 text-muted-foreground opacity-0 transition-all hover:bg-background hover:text-foreground group-hover:opacity-100"
            title="Export campaign"
          >
            <Download className="h-4 w-4" />
          </button>
          <button
            onClick={(e) => {
              e.stopPropagation();
              if (
                window.confirm(
                  `Archive campaign "${campaign.name}"? This will mark it as inactive.`
                )
              ) {
                onArchive(campaign.id);
              }
            }}
            className="rounded p-1 text-muted-foreground opacity-0 transition-all hover:bg-background hover:text-foreground group-hover:opacity-100"
            title="Archive campaign"
          >
            <Archive className="h-4 w-4" />
          </button>
        </div>
      </div>

      <p className="mb-3 text-sm text-muted-foreground">{campaign.description}</p>

      <div className="flex items-center justify-between text-sm">
        <div className="flex items-center gap-4">
          <span className={`font-medium ${getPhaseColor(campaign.phase)}`}>
            Phase: {campaign.phase}
          </span>
          <span className="text-muted-foreground">
            {campaign.target_count} target{campaign.target_count !== 1 ? "s" : ""}
          </span>
          <span className="text-muted-foreground">
            {campaign.objectives.length} objective{campaign.objectives.length !== 1 ? "s" : ""}
          </span>
        </div>
        <span className="text-xs text-muted-foreground">
          {new Date(campaign.created).toLocaleDateString()}
        </span>
      </div>
    </div>
  );
}
