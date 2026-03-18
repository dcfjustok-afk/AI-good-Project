import { useQuery } from "@tanstack/react-query";
import { invokeCommand } from "../lib/tauri";
import type { HealthCheckResponse } from "../types/project";

export function useAppHealth() {
  return useQuery({
    queryKey: ["app-health"],
    queryFn: () => invokeCommand<HealthCheckResponse>("health_check"),
  });
}