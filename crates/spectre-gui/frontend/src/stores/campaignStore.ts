import { create } from "zustand";
import type { CampaignSummary } from "@/types/campaign";

interface CampaignState {
  campaigns: CampaignSummary[];
  activeCampaign: CampaignSummary | null;

  setCampaigns: (campaigns: CampaignSummary[]) => void;
  setActiveCampaign: (campaign: CampaignSummary | null) => void;
  addCampaign: (campaign: CampaignSummary) => void;
  updateCampaign: (name: string, updates: Partial<CampaignSummary>) => void;
}

export const useCampaignStore = create<CampaignState>((set) => ({
  campaigns: [],
  activeCampaign: null,

  setCampaigns: (campaigns) => set({ campaigns }),

  setActiveCampaign: (campaign) => set({ activeCampaign: campaign }),

  addCampaign: (campaign) =>
    set((state) => ({ campaigns: [...state.campaigns, campaign] })),

  updateCampaign: (name, updates) =>
    set((state) => ({
      campaigns: state.campaigns.map((c) =>
        c.name === name ? { ...c, ...updates } : c,
      ),
    })),
}));
