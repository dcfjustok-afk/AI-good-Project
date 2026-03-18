import { useQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/tauri";
import type { ProjectFilters, ProjectSummary } from "../types/project";

export function useFavorites(sortBy: NonNullable<ProjectFilters["sortBy"]> = "favoritedAt") {
  const filters: ProjectFilters = {
    favoritesOnly: true,
    sortBy,
    limit: 50,
  };

  return useQuery({
    queryKey: ["favorites", filters],
    queryFn: () => invokeCommand<ProjectSummary[]>("get_projects", { filters }),
    placeholderData: (previousData) => previousData,
  });
}