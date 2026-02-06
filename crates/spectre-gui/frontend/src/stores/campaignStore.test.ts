import { describe, it, expect, beforeEach } from "vitest";
import { useCampaignStore } from "./campaignStore";
import type { CampaignSummary } from "@/types/campaign";

const mockCampaign: CampaignSummary = {
  id: "test-id",
  name: "test-campaign",
  description: "Test description",
  status: "planning",
  phase: "recon",
  target_count: 5,
  objectives: ["Objective 1"],
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
    useCampaignStore.getState().addCampaign({ ...mockCampaign, id: "test-id-2", name: "second" });
    expect(useCampaignStore.getState().campaigns).toHaveLength(2);
  });

  it("updateCampaign updates matching campaign by id", () => {
    useCampaignStore.getState().setCampaigns([mockCampaign]);
    useCampaignStore.getState().updateCampaign("test-id", { status: "active" });
    expect(useCampaignStore.getState().campaigns[0].status).toBe("active");
  });

  it("updateCampaign ignores non-matching ids", () => {
    useCampaignStore.getState().setCampaigns([mockCampaign]);
    useCampaignStore.getState().updateCampaign("nonexistent", { status: "active" });
    expect(useCampaignStore.getState().campaigns[0].status).toBe("planning");
  });

  it("updateCampaign updates active campaign if it matches", () => {
    useCampaignStore.getState().setCampaigns([mockCampaign]);
    useCampaignStore.getState().setActiveCampaign(mockCampaign);
    useCampaignStore.getState().updateCampaign("test-id", { phase: "analysis" });
    expect(useCampaignStore.getState().activeCampaign?.phase).toBe("analysis");
  });

  it("removeCampaign removes campaign by id", () => {
    useCampaignStore.getState().setCampaigns([mockCampaign]);
    useCampaignStore.getState().removeCampaign("test-id");
    expect(useCampaignStore.getState().campaigns).toHaveLength(0);
  });

  it("removeCampaign clears active campaign if it matches", () => {
    useCampaignStore.getState().setCampaigns([mockCampaign]);
    useCampaignStore.getState().setActiveCampaign(mockCampaign);
    useCampaignStore.getState().removeCampaign("test-id");
    expect(useCampaignStore.getState().activeCampaign).toBeNull();
  });
});
