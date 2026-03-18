import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invokeCommand } from "../lib/tauri";
import type { SyncDataResponse } from "../types/project";

export function useSyncData() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () => invokeCommand<SyncDataResponse>("sync_data"),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["projects"] });
      void queryClient.invalidateQueries({ queryKey: ["favorites"] });
      void queryClient.invalidateQueries({ queryKey: ["project-detail"] });
    },
  });
}