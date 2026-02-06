import { describe, it, expect, beforeEach } from "vitest";
import { useCampaignStore } from "./campaignStore";
import type { CampaignSummary } from "@/types/campaign";

const mockCampaign: CampaignSummary = {
  name: "test-campaign",
  status: "planning",
  phase: "recon",
  target_count: 5,
  created: "2026-01-01T00:00:00Z",
};

describe("campaignStore", () => {
  beforeEach(() => {
    useCampaignStore.setState({ campaigns: [], activeCampaign: null });
  });

  it("has correct default state", () => {
    const state = useCampaignStore.getState();
    expect(state.campaigns).toEqual([]);
    expect(state.activeCampaign).toBeNull();
  });

  it("setCampaigns replaces campaign list", () => {
    useCampaignStore.getState().setCampaigns([mockCampaign]);
    expect(useCampaignStore.getState().campaigns).toHaveLength(1);
  });

  it("setActiveCampaign sets active campaign", () => {
    useCampaignStore.getState().setActiveCampaign(mockCampaign);
    expect(useCampaignStore.getState().activeCampaign?.name).toBe("test-campaign");
  });

  it("addCampaign appends to list", () => {
    useCampaignStore.getState().addCampaign(mockCampaign);
    useCampaignStore.getState().addCampaign({ ...mockCampaign, name: "second" });
    expect(useCampaignStore.getState().campaigns).toHaveLength(2);
  });

  it("updateCampaign updates matching campaign", () => {
    useCampaignStore.getState().setCampaigns([mockCampaign]);
    useCampaignStore.getState().updateCampaign("test-campaign", { status: "active" });
    expect(useCampaignStore.getState().campaigns[0].status).toBe("active");
  });

  it("updateCampaign ignores non-matching names", () => {
    useCampaignStore.getState().setCampaigns([mockCampaign]);
    useCampaignStore.getState().updateCampaign("nonexistent", { status: "active" });
    expect(useCampaignStore.getState().campaigns[0].status).toBe("planning");
  });
});
