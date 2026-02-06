import { useIpc } from "./useIpc";
import type { IdentityInfo, PeerInfo, SendRequest } from "@/types/comms";

export function useComms() {
  const identity = useIpc<IdentityInfo | null>("get_identity");
  const peers = useIpc<PeerInfo[]>("list_peers");
  const send = useIpc<string, { request: SendRequest }>("send_data");

  return {
    getIdentity: identity.execute,
    listPeers: peers.execute,
    sendData: send.execute,
    identity: identity.data,
    peers: peers.data ?? [],
    loading: identity.loading || peers.loading || send.loading,
    error: identity.error || peers.error || send.error,
  };
}
