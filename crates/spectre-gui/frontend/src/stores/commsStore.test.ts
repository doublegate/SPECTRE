import { describe, it, expect, beforeEach } from "vitest";
import { useCommsStore } from "./commsStore";
import type { PeerInfo } from "@/types/comms";

const mockPeer: PeerInfo = {
  name: "alice",
  fingerprint: "abc123",
  status: "online",
};

describe("commsStore", () => {
  beforeEach(() => {
    useCommsStore.setState({ identity: null, peers: [] });
  });

  it("has correct default state", () => {
    const state = useCommsStore.getState();
    expect(state.identity).toBeNull();
    expect(state.peers).toEqual([]);
  });

  it("setIdentity sets identity", () => {
    useCommsStore.getState().setIdentity({ name: "me", fingerprint: "xyz", created: "2026-01-01" });
    expect(useCommsStore.getState().identity?.name).toBe("me");
  });

  it("setPeers replaces peer list", () => {
    useCommsStore.getState().setPeers([mockPeer]);
    expect(useCommsStore.getState().peers).toHaveLength(1);
  });

  it("addPeer appends to peer list", () => {
    useCommsStore.getState().addPeer(mockPeer);
    useCommsStore.getState().addPeer({ ...mockPeer, name: "bob", fingerprint: "def456" });
    expect(useCommsStore.getState().peers).toHaveLength(2);
  });

  it("removePeer removes by fingerprint", () => {
    useCommsStore.getState().setPeers([mockPeer, { name: "bob", fingerprint: "def456", status: "online" }]);
    useCommsStore.getState().removePeer("abc123");
    expect(useCommsStore.getState().peers).toHaveLength(1);
    expect(useCommsStore.getState().peers[0].name).toBe("bob");
  });

  it("removePeer ignores non-matching fingerprint", () => {
    useCommsStore.getState().setPeers([mockPeer]);
    useCommsStore.getState().removePeer("nonexistent");
    expect(useCommsStore.getState().peers).toHaveLength(1);
  });
});
