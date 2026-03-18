import { create } from "zustand";

import type { ProjectFilters } from "../types/project";

type ProjectFiltersState = {
  language: string;
  category: string;
  frontendOnly: boolean;
  hasDemo: boolean;
  sortBy: NonNullable<ProjectFilters["sortBy"]>;
  limit: number;
  setLanguage: (language: string) => void;
  setCategory: (category: string) => void;
  toggleFrontendOnly: () => void;
  toggleHasDemo: () => void;
  setSortBy: (sortBy: NonNullable<ProjectFilters["sortBy"]>) => void;
  setLimit: (limit: number) => void;
  resetFilters: () => void;
};

const defaultFilters = {
  language: "",
  category: "",
  frontendOnly: false,
  hasDemo: false,
  sortBy: "score" as const,
  limit: 12,
};

export const useProjectFiltersStore = create<ProjectFiltersState>((set) => ({
  ...defaultFilters,
  setLanguage: (language) => set({ language }),
  setCategory: (category) => set({ category }),
  toggleFrontendOnly: () =>
    set((state) => ({
      frontendOnly: !state.frontendOnly,
    })),
  toggleHasDemo: () =>
    set((state) => ({
      hasDemo: !state.hasDemo,
    })),
  setSortBy: (sortBy) => set({ sortBy }),
  setLimit: (limit) => set({ limit }),
  resetFilters: () => set(defaultFilters),
}));