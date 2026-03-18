import { useQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/tauri";
import type { ProjectFilters, ProjectSummary } from "../types/project";

export function useProjects(filters: ProjectFilters) {
  return useQuery({
    queryKey: ["projects", filters],
    queryFn: () => invokeCommand<ProjectSummary[]>("get_projects", { filters }),
  });
}