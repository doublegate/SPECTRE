import { useEffect } from "react";
import { useIpc } from "./useIpc";
import type { VersionInfo, SystemStatus } from "@/types/config";

export function useStatus() {
  const version = useIpc<VersionInfo>("get_version");
  const status = useIpc<SystemStatus>("get_status");

  useEffect(() => {
    version.execute();
    status.execute();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return {
    version: version.data,
    status: status.data,
    loading: version.loading || status.loading,
    error: version.error || status.error,
    refresh: async () => {
      await Promise.all([version.execute(), status.execute()]);
    },
  };
}
