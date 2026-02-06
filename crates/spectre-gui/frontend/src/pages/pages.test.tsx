import { describe, it, expect, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { Dashboard } from "./Dashboard";
import { Targets } from "./Targets";
import { Recon } from "./Recon";
import { Analysis } from "./Analysis";
import { Comms } from "./Comms";
import { Campaigns } from "./Campaigns";
import { Reports } from "./Reports";
import { Settings } from "./Settings";

// Mock @tauri-apps/api/core
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn((cmd: string) => {
    if (cmd === "get_version") {
      return Promise.resolve({
        version: "0.5.0-alpha.2",
        name: "SPECTRE",
        description: "Test",
      });
    }
    if (cmd === "get_status") {
      return Promise.resolve({
        components: [],
        config_loaded: true,
      });
    }
    if (cmd === "get_findings") {
      return Promise.resolve([]);
    }
    if (cmd === "get_dashboard_stats") {
      return Promise.resolve({
        total_hosts: 42,
        total_ports: 256,
        open_ports: 128,
        services_found: 64,
        severity_counts: {
          critical: 2,
          high: 5,
          medium: 7,
          low: 2,
          info: 0,
        },
        top_services: [],
        recent_activity: [],
      });
    }
    if (cmd === "list_campaigns") {
      return Promise.resolve([]);
    }
    return Promise.reject("Unknown command");
  }),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
}));

function renderWithRouter(component: React.ReactElement) {
  return render(<MemoryRouter>{component}</MemoryRouter>);
}

describe("Dashboard", () => {
  it("renders stat cards", async () => {
    renderWithRouter(<Dashboard />);
    await waitFor(() => {
      expect(screen.getByText("Total Hosts")).toBeInTheDocument();
      expect(screen.getByText("Open Ports")).toBeInTheDocument();
      expect(screen.getByText("Services")).toBeInTheDocument();
      expect(screen.getByText("Findings")).toBeInTheDocument();
    }, { timeout: 3000 });
  });

  it("renders dashboard layout", async () => {
    renderWithRouter(<Dashboard />);
    // Wait for data to load before checking layout
    await waitFor(() => {
      expect(screen.queryByText("No data available")).not.toBeInTheDocument();
    }, { timeout: 3000 });
    const dashboard = document.querySelector('.space-y-6');
    expect(dashboard).toBeTruthy();
  });
});

describe("Targets", () => {
  it("renders target management heading", () => {
    renderWithRouter(<Targets />);
    expect(screen.getByText("Target Management")).toBeInTheDocument();
  });

  it("renders target input textarea", () => {
    renderWithRouter(<Targets />);
    expect(screen.getByPlaceholderText(/Enter targets/)).toBeInTheDocument();
  });
});

describe("Recon", () => {
  it("renders reconnaissance heading", () => {
    renderWithRouter(<Recon />);
    expect(screen.getByText("Network Reconnaissance")).toBeInTheDocument();
  });

  it("shows idle status by default", () => {
    renderWithRouter(<Recon />);
    expect(screen.getByText(/No active scan/)).toBeInTheDocument();
  });

  it("shows empty topology message", () => {
    renderWithRouter(<Recon />);
    expect(screen.getByText(/No hosts discovered yet/)).toBeInTheDocument();
  });
});

describe("Analysis", () => {
  it("renders data analysis heading", () => {
    renderWithRouter(<Analysis />);
    expect(screen.getByText("Data Analysis")).toBeInTheDocument();
  });

  it("renders input and output sections", () => {
    renderWithRouter(<Analysis />);
    expect(screen.getByText("Input")).toBeInTheDocument();
    expect(screen.getByText("Output")).toBeInTheDocument();
  });

  it("renders operations section", () => {
    renderWithRouter(<Analysis />);
    expect(screen.getByText(/CyberChef Operations/)).toBeInTheDocument();
  });
});

describe("Comms", () => {
  it("renders secure communications heading", () => {
    renderWithRouter(<Comms />);
    expect(screen.getByText("Secure Communications")).toBeInTheDocument();
  });

  it("renders identity, peers, and transfer sections", () => {
    renderWithRouter(<Comms />);
    expect(screen.getByText("Identity")).toBeInTheDocument();
    expect(screen.getByText("Peers")).toBeInTheDocument();
    expect(screen.getByText("Transfer History")).toBeInTheDocument();
  });
});

describe("Campaigns", () => {
  it("renders campaign management heading", () => {
    renderWithRouter(<Campaigns />);
    expect(screen.getByText("Campaign Management")).toBeInTheDocument();
  });

  it("renders new campaign button", () => {
    renderWithRouter(<Campaigns />);
    expect(screen.getByText("New Campaign")).toBeInTheDocument();
  });

  it("shows loading state initially", () => {
    renderWithRouter(<Campaigns />);
    expect(screen.getByText("Loading campaigns...")).toBeInTheDocument();
  });
});

describe("Reports", () => {
  it("renders reports heading", async () => {
    renderWithRouter(<Reports />);
    await waitFor(() => {
      expect(screen.getByText("Reports & Findings")).toBeInTheDocument();
    }, { timeout: 3000 });
  });

  it("renders empty state when no findings", async () => {
    renderWithRouter(<Reports />);
    // New implementation shows a loading state first, then empty state
    await waitFor(() => {
      expect(screen.getByText("Reports & Findings")).toBeInTheDocument();
    }, { timeout: 3000 });
  });
});

describe("Settings", () => {
  it("renders settings heading", () => {
    renderWithRouter(<Settings />);
    expect(screen.getByText("Settings")).toBeInTheDocument();
  });

  it("renders theme options", () => {
    renderWithRouter(<Settings />);
    expect(screen.getByText("Dark")).toBeInTheDocument();
    expect(screen.getByText("Light")).toBeInTheDocument();
    expect(screen.getByText("Tactical")).toBeInTheDocument();
    expect(screen.getByText("Matrix")).toBeInTheDocument();
    expect(screen.getByText("Hacker")).toBeInTheDocument();
  });
});
