import { create } from "zustand";
import type { IdentityInfo, PeerInfo } from "@/types/comms";

interface CommsState {
  identity: IdentityInfo | null;
  peers: PeerInfo[];

  setIdentity: (identity: IdentityInfo | null) => void;
  setPeers: (peers: PeerInfo[]) => void;
  addPeer: (peer: PeerInfo) => void;
  removePeer: (fingerprint: string) => void;
}

export const useCommsStore = create<CommsState>((set) => ({
  identity: null,
  peers: [],

  setIdentity: (identity) => set({ identity }),
  setPeers: (peers) => set({ peers }),
  addPeer: (peer) => set((state) => ({ peers: [...state.peers, peer] })),
  removePeer: (fingerprint) =>
    set((state) => ({
      peers: state.peers.filter((p) => p.fingerprint !== fingerprint),
    })),
}));
