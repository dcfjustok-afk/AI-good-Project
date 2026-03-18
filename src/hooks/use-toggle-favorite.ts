import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invokeCommand } from "../lib/tauri";
import type { FavoriteToggleResponse } from "../types/project";

export function useToggleFavorite() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (projectId: number) =>
      invokeCommand<FavoriteToggleResponse>("toggle_favorite", {
        projectId,
      }),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["projects"] });
      void queryClient.invalidateQueries({ queryKey: ["favorites"] });
      void queryClient.invalidateQueries({ queryKey: ["project-detail"] });
    },
  });
}