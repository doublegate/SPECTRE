import { useIpc } from "./useIpc";
import type { ChefRequest, ChefResult } from "@/types/chef";

export function useChef() {
  const execute = useIpc<ChefResult, { request: ChefRequest }>("execute_chef");
  const listOps = useIpc<string[]>("list_chef_operations");

  return {
    executeChef: execute.execute,
    listOperations: listOps.execute,
    result: execute.data,
    operations: listOps.data ?? [],
    loading: execute.loading || listOps.loading,
    error: execute.error || listOps.error,
  };
}
