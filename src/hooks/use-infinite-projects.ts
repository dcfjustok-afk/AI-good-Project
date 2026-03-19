import { useInfiniteQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/tauri";
import type { ProjectFilters, ProjectListResponse } from "../types/project";

type InfiniteProjectFilters = Omit<ProjectFilters, "page">;

export function useInfiniteProjects(filters: InfiniteProjectFilters) {
  return useInfiniteQuery({
    queryKey: ["projects-infinite", filters],
    queryFn: ({ pageParam }) =>
      invokeCommand<ProjectListResponse>("get_projects", {
        filters: {
          ...filters,
          page: pageParam,
        },
      }),
    initialPageParam: 1,
    getNextPageParam: (lastPage, allPages) => (lastPage.hasMore ? allPages.length + 1 : undefined),
  });
}