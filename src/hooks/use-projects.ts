import { useQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/tauri";
import type { ProjectFilters, ProjectListResponse } from "../types/project";

export function useProjects(filters: ProjectFilters) {
  return useQuery({
    queryKey: ["projects", filters],
    queryFn: () => invokeCommand<ProjectListResponse>("get_projects", { filters }),
    placeholderData: (previousData) => previousData,
  });
}