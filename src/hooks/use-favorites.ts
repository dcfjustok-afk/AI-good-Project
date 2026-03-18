import { useQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/tauri";
import type { ProjectSummary } from "../types/project";

export function useFavorites() {
  return useQuery({
    queryKey: ["favorites"],
    queryFn: () => invokeCommand<ProjectSummary[]>("get_favorites"),
  });
}