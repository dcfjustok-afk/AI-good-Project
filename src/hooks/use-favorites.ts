import { useQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/tauri";
import type { ProjectFilters, ProjectListResponse } from "../types/project";

export function useFavorites(sortBy: NonNullable<ProjectFilters["sortBy"]> = "favoritedAt") {
  const filters: ProjectFilters = {
    favoritesOnly: true,
    sortBy,
    page: 1,
    limit: 50,
  };

  return useQuery({
    queryKey: ["favorites", filters],
    queryFn: () => invokeCommand<ProjectListResponse>("get_projects", { filters }),
    placeholderData: (previousData) => previousData,
  });
}