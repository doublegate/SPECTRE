import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import App from "./App";

// Mock @tauri-apps/api/core
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn((cmd: string) => {
    if (cmd === "get_version") {
      return Promise.resolve({
        version: "0.5.0-alpha.1",
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

describe("App", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders SPECTRE heading", async () => {
    render(<App />);
    await waitFor(() => {
      expect(screen.getByText("SPECTRE")).toBeInTheDocument();
    });
  });

  it("displays version from IPC", async () => {
    render(<App />);
    await waitFor(() => {
      expect(screen.getByText("v0.5.0-alpha.1")).toBeInTheDocument();
    });
  });

  it("renders all three components", async () => {
    render(<App />);
    await waitFor(() => {
      expect(screen.getByText("ProRT-IP")).toBeInTheDocument();
      expect(screen.getByText("CyberChef-MCP")).toBeInTheDocument();
      expect(screen.getByText("WRAITH-Protocol")).toBeInTheDocument();
    });
  });

  it("shows component versions", async () => {
    render(<App />);
    await waitFor(() => {
      expect(screen.getByText("v1.0.0")).toBeInTheDocument();
      expect(screen.getByText("v1.9.0")).toBeInTheDocument();
      expect(screen.getByText("v2.3.7")).toBeInTheDocument();
    });
  });

  it("shows available status badges", async () => {
    render(<App />);
    await waitFor(() => {
      const badges = screen.getAllByText("available");
      expect(badges).toHaveLength(3);
    });
  });
});
