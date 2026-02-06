import { useIpc } from "./useIpc";
import type {
  CampaignSummary,
  CreateCampaignRequest,
  ExportRequest,
  ImportRequest,
} from "@/types/campaign";

export function useCampaign() {
  const create = useIpc<string, { request: CreateCampaignRequest }>("create_campaign");
  const list = useIpc<CampaignSummary[]>("list_campaigns");
  const get = useIpc<CampaignSummary | null, { id: string }>("get_campaign");
  const advance = useIpc<string, { id: string }>("advance_campaign");
  const exportCmd = useIpc<void, { request: ExportRequest }>("export_campaign");
  const importCmd = useIpc<string, { request: ImportRequest }>("import_campaign");
  const archive = useIpc<void, { id: string }>("archive_campaign");

  return {
    createCampaign: create.execute,
    listCampaigns: list.execute,
    getCampaign: get.execute,
    advanceCampaign: advance.execute,
    exportCampaign: exportCmd.execute,
    importCampaign: importCmd.execute,
    archiveCampaign: archive.execute,
    campaigns: list.data ?? [],
    loading:
      create.loading ||
      list.loading ||
      get.loading ||
      advance.loading ||
      exportCmd.loading ||
      importCmd.loading ||
      archive.loading,
    error:
      create.error ||
      list.error ||
      get.error ||
      advance.error ||
      exportCmd.error ||
      importCmd.error ||
      archive.error,
  };
}
