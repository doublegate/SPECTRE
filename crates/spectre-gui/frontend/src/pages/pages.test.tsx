import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router";
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
  it("renders stat cards", () => {
    renderWithRouter(<Dashboard />);
    expect(screen.getByText("Hosts Scanned")).toBeInTheDocument();
    expect(screen.getByText("Open Ports")).toBeInTheDocument();
    expect(screen.getByText("Services")).toBeInTheDocument();
    expect(screen.getByText("Findings")).toBeInTheDocument();
  });

  it("renders component status section", () => {
    renderWithRouter(<Dashboard />);
    expect(screen.getByText("Component Status")).toBeInTheDocument();
  });

  it("renders platform info section", () => {
    renderWithRouter(<Dashboard />);
    expect(screen.getByText("Platform Info")).toBeInTheDocument();
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
    expect(screen.getByText("Reconnaissance")).toBeInTheDocument();
  });

  it("shows idle status by default", () => {
    renderWithRouter(<Recon />);
    expect(screen.getByText(/Idle/)).toBeInTheDocument();
  });

  it("shows empty results message", () => {
    renderWithRouter(<Recon />);
    expect(screen.getByText(/No scan results yet/)).toBeInTheDocument();
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

  it("shows empty state message", () => {
    renderWithRouter(<Campaigns />);
    expect(screen.getByText("No campaigns")).toBeInTheDocument();
  });
});

describe("Reports", () => {
  it("renders reports heading", () => {
    renderWithRouter(<Reports />);
    expect(screen.getByText("Reports & Findings")).toBeInTheDocument();
  });

  it("renders export format buttons", () => {
    renderWithRouter(<Reports />);
    expect(screen.getByText("HTML")).toBeInTheDocument();
    expect(screen.getByText("Markdown")).toBeInTheDocument();
    expect(screen.getByText("CSV")).toBeInTheDocument();
    expect(screen.getByText("JSON")).toBeInTheDocument();
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
