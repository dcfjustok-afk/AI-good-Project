import { create } from "zustand";

import type { ProjectFilters } from "../types/project";

type ProjectFiltersState = {
  language: string;
  category: string;
  frontendOnly: boolean;
  hasDemo: boolean;
  sortBy: NonNullable<ProjectFilters["sortBy"]>;
  page: number;
  limit: number;
  setLanguage: (language: string) => void;
  setCategory: (category: string) => void;
  toggleFrontendOnly: () => void;
  toggleHasDemo: () => void;
  setSortBy: (sortBy: NonNullable<ProjectFilters["sortBy"]>) => void;
  previousPage: () => void;
  nextPage: () => void;
  setLimit: (limit: number) => void;
  resetFilters: () => void;
};

const defaultFilters = {
  language: "",
  category: "",
  frontendOnly: false,
  hasDemo: false,
  sortBy: "score" as const,
  page: 1,
  limit: 12,
};

export const useProjectFiltersStore = create<ProjectFiltersState>((set) => ({
  ...defaultFilters,
  setLanguage: (language) => set({ language, page: 1 }),
  setCategory: (category) => set({ category, page: 1 }),
  toggleFrontendOnly: () =>
    set((state) => ({
      frontendOnly: !state.frontendOnly,
      page: 1,
    })),
  toggleHasDemo: () =>
    set((state) => ({
      hasDemo: !state.hasDemo,
      page: 1,
    })),
  setSortBy: (sortBy) => set({ sortBy, page: 1 }),
  previousPage: () => set((state) => ({ page: Math.max(1, state.page - 1) })),
  nextPage: () => set((state) => ({ page: state.page + 1 })),
  setLimit: (limit) => set({ limit, page: 1 }),
  resetFilters: () => set(defaultFilters),
}));