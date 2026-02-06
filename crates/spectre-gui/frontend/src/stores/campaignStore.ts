import { create } from "zustand";
import type { CampaignSummary } from "@/types/campaign";

interface CampaignState {
  campaigns: CampaignSummary[];
  activeCampaign: CampaignSummary | null;

  setCampaigns: (campaigns: CampaignSummary[]) => void;
  setActiveCampaign: (campaign: CampaignSummary | null) => void;
  addCampaign: (campaign: CampaignSummary) => void;
  updateCampaign: (id: string, updates: Partial<CampaignSummary>) => void;
  removeCampaign: (id: string) => void;
}

export const useCampaignStore = create<CampaignState>((set) => ({
  campaigns: [],
  activeCampaign: null,

  setCampaigns: (campaigns) => set({ campaigns }),

  setActiveCampaign: (campaign) => set({ activeCampaign: campaign }),

  addCampaign: (campaign) =>
    set((state) => ({ campaigns: [...state.campaigns, campaign] })),

  updateCampaign: (id, updates) =>
    set((state) => ({
      campaigns: state.campaigns.map((c) => (c.id === id ? { ...c, ...updates } : c)),
      activeCampaign:
        state.activeCampaign?.id === id
          ? { ...state.activeCampaign, ...updates }
          : state.activeCampaign,
    })),

  removeCampaign: (id) =>
    set((state) => ({
      campaigns: state.campaigns.filter((c) => c.id !== id),
      activeCampaign: state.activeCampaign?.id === id ? null : state.activeCampaign,
    })),
}));
