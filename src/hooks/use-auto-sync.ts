import { useEffect, useEffectEvent } from "react";
import { useSyncPreferencesStore } from "../store/use-sync-preferences";
import { useSyncData } from "./use-sync-data";

export function useAutoSync() {
  const syncMutation = useSyncData();
  const autoSyncEnabled = useSyncPreferencesStore((state) => state.autoSyncEnabled);
  const intervalMinutes = useSyncPreferencesStore((state) => state.intervalMinutes);
  const markSyncTriggered = useSyncPreferencesStore((state) => state.markSyncTriggered);
  const markSyncCompleted = useSyncPreferencesStore((state) => state.markSyncCompleted);

  const runAutoSync = useEffectEvent(() => {
    if (!autoSyncEnabled || syncMutation.isPending) {
      return;
    }

    const timestamp = new Date().toISOString();
    markSyncTriggered(timestamp);
    syncMutation.mutate(undefined, {
      onSuccess: () => {
        markSyncCompleted(new Date().toISOString());
      },
    });
  });

  useEffect(() => {
    if (!autoSyncEnabled) {
      return undefined;
    }

    runAutoSync();
    const timer = window.setInterval(runAutoSync, intervalMinutes * 60 * 1000);
    return () => window.clearInterval(timer);
  }, [autoSyncEnabled, intervalMinutes, runAutoSync]);

  return {
    isSyncing: syncMutation.isPending,
  };
}