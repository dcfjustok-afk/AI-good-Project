import { useQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/tauri";
import type { AiProjectSectionsResponse } from "../types/project";

export function useAiProjectSections(limit: number) {
  return useQuery({
    queryKey: ["ai-project-sections", limit],
    queryFn: () =>
      invokeCommand<AiProjectSectionsResponse>("get_ai_project_sections", {
        limit,
      }),
    placeholderData: (previousData) => previousData,
  });
}
