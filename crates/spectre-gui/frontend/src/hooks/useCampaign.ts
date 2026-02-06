import { useIpc } from "./useIpc";
import type { CampaignSummary, CreateCampaignRequest } from "@/types/campaign";

export function useCampaign() {
  const create = useIpc<string, { request: CreateCampaignRequest }>("create_campaign");
  const list = useIpc<CampaignSummary[]>("list_campaigns");
  const get = useIpc<CampaignSummary | null, { name: string }>("get_campaign");
  const advance = useIpc<string, { name: string }>("advance_campaign");

  return {
    createCampaign: create.execute,
    listCampaigns: list.execute,
    getCampaign: get.execute,
    advanceCampaign: advance.execute,
    campaigns: list.data ?? [],
    loading: create.loading || list.loading,
    error: create.error || list.error,
  };
}
