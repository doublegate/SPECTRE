import { describe, it, expect, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter } from "react-router";
import { Sidebar } from "@/layouts/Sidebar";
import { Header } from "@/layouts/Header";
import { StatusBar } from "@/layouts/StatusBar";

// Mock @tauri-apps/api/core
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn((cmd: string) => {
    if (cmd === "get_version") {
      return Promise.resolve({
        version: "0.5.0-alpha.2",
        name: "SPECTRE",
        description:
          "Security Platform for Encrypted Comms, Testing, Enumeration, Recon",
      });
    }
    if (cmd === "get_status") {
      return Promise.resolve({
        components: [
          {
            name: "ProRT-IP",
            status: "available",
            version: "1.0.0",
            details: "Network reconnaissance engine",
          },
          {
            name: "CyberChef-MCP",
            status: "available",
            version: "1.9.0",
            details: "Data analysis via MCP protocol",
          },
          {
            name: "WRAITH-Protocol",
            status: "available",
            version: "2.3.7",
            details: "Secure communications",
          },
        ],
        config_loaded: true,
      });
    }
    return Promise.reject("Unknown command");
  }),
}));

// Mock @tauri-apps/api/event
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
}));

describe("Sidebar", () => {
  it("renders SPECTRE branding", () => {
    render(
      <MemoryRouter initialEntries={["/dashboard"]}>
        <Sidebar />
      </MemoryRouter>,
    );
    expect(screen.getByText("SPECTRE")).toBeInTheDocument();
  });

  it("renders all 8 navigation items", () => {
    render(
      <MemoryRouter initialEntries={["/dashboard"]}>
        <Sidebar />
      </MemoryRouter>,
    );
    expect(screen.getByText("Dashboard")).toBeInTheDocument();
    expect(screen.getByText("Targets")).toBeInTheDocument();
    expect(screen.getByText("Recon")).toBeInTheDocument();
    expect(screen.getByText("Analysis")).toBeInTheDocument();
    expect(screen.getByText("Comms")).toBeInTheDocument();
    expect(screen.getByText("Campaigns")).toBeInTheDocument();
    expect(screen.getByText("Reports")).toBeInTheDocument();
    expect(screen.getByText("Settings")).toBeInTheDocument();
  });
});

describe("Header", () => {
  it("renders page title from route", () => {
    render(
      <MemoryRouter initialEntries={["/dashboard"]}>
        <Header />
      </MemoryRouter>,
    );
    expect(screen.getByText("Dashboard")).toBeInTheDocument();
  });

  it("renders theme picker button", () => {
    render(
      <MemoryRouter initialEntries={["/dashboard"]}>
        <Header />
      </MemoryRouter>,
    );
    // Default theme is "dark"
    expect(screen.getByText("dark")).toBeInTheDocument();
  });
});

describe("StatusBar", () => {
  it("renders with version info after IPC resolves", async () => {
    render(
      <MemoryRouter>
        <StatusBar />
      </MemoryRouter>,
    );
    await waitFor(() => {
      expect(screen.getByText(/0\.5\.0-alpha\.2/)).toBeInTheDocument();
    });
  });

  it("shows component count", async () => {
    render(
      <MemoryRouter>
        <StatusBar />
      </MemoryRouter>,
    );
    await waitFor(() => {
      expect(screen.getByText(/3\/3/)).toBeInTheDocument();
    });
  });

  it("shows config loaded status", async () => {
    render(
      <MemoryRouter>
        <StatusBar />
      </MemoryRouter>,
    );
    await waitFor(() => {
      expect(screen.getByText("Config loaded")).toBeInTheDocument();
    });
  });
});
