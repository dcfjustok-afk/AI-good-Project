import { create } from "zustand";
import { persist } from "zustand/middleware";

type SyncPreferencesState = {
  autoSyncEnabled: boolean;
  intervalMinutes: number;
  lastTriggeredAt: string | null;
  lastCompletedAt: string | null;
  setAutoSyncEnabled: (value: boolean) => void;
  setIntervalMinutes: (value: number) => void;
  markSyncTriggered: (value: string) => void;
  markSyncCompleted: (value: string) => void;
};

export const useSyncPreferencesStore = create<SyncPreferencesState>()(
  persist(
    (set) => ({
      autoSyncEnabled: false,
      intervalMinutes: 30,
      lastTriggeredAt: null,
      lastCompletedAt: null,
      setAutoSyncEnabled: (value) => set({ autoSyncEnabled: value }),
      setIntervalMinutes: (value) => set({ intervalMinutes: value }),
      markSyncTriggered: (value) => set({ lastTriggeredAt: value }),
      markSyncCompleted: (value) => set({ lastCompletedAt: value }),
    }),
    {
      name: "ai-good-project-sync-preferences",
    },
  ),
);