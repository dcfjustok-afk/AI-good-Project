import { useQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/tauri";
import type { ProjectDetail } from "../types/project";

export function useProjectDetail(owner?: string, repo?: string) {
  return useQuery({
    queryKey: ["project-detail", owner, repo],
    enabled: Boolean(owner && repo),
    queryFn: () =>
      invokeCommand<ProjectDetail>("get_project_detail", {
        owner,
        repo,
      }),
  });
}