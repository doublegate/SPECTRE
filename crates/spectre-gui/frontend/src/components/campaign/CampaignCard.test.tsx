import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { BrowserRouter } from "react-router";
import { CampaignCard } from "./CampaignCard";
import type { CampaignSummary } from "@/types/campaign";

const mockCampaign: CampaignSummary = {
  id: "test-id",
  name: "Operation Test",
  description: "Test campaign",
  status: "planning",
  phase: "recon",
  target_count: 5,
  objectives: ["Find vulns"],
  created: "2026-01-01T00:00:00Z",
};

describe("CampaignCard", () => {
  it("renders campaign name and description", () => {
    render(
      <BrowserRouter>
        <CampaignCard campaign={mockCampaign} onArchive={vi.fn()} onExport={vi.fn()} />
      </BrowserRouter>
    );

    expect(screen.getByText("Operation Test")).toBeInTheDocument();
    expect(screen.getByText("Test campaign")).toBeInTheDocument();
  });

  it("displays target count", () => {
    render(
      <BrowserRouter>
        <CampaignCard campaign={mockCampaign} onArchive={vi.fn()} onExport={vi.fn()} />
      </BrowserRouter>
    );

    expect(screen.getByText("5 targets")).toBeInTheDocument();
  });

  it("displays objectives count", () => {
    render(
      <BrowserRouter>
        <CampaignCard campaign={mockCampaign} onArchive={vi.fn()} onExport={vi.fn()} />
      </BrowserRouter>
    );

    expect(screen.getByText(/1 objective/)).toBeInTheDocument();
  });
});
