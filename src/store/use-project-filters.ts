import { create } from "zustand";

import type { ProjectFilters } from "../types/project";

type ProjectFiltersState = {
  search: string;
  topic: string;
  language: string;
  category: string;
  frontendOnly: boolean;
  hasDemo: boolean;
  aiOnly: boolean;
  era: "" | "classic" | "latest";
  sortBy: NonNullable<ProjectFilters["sortBy"]>;
  page: number;
  limit: number;
  setSearch: (search: string) => void;
  setTopic: (topic: string) => void;
  setLanguage: (language: string) => void;
  setCategory: (category: string) => void;
  toggleFrontendOnly: () => void;
  toggleHasDemo: () => void;
  toggleAiOnly: () => void;
  setEra: (era: "" | "classic" | "latest") => void;
  setSortBy: (sortBy: NonNullable<ProjectFilters["sortBy"]>) => void;
  previousPage: () => void;
  nextPage: () => void;
  setLimit: (limit: number) => void;
  resetFilters: () => void;
};

const defaultFilters: Pick<
  ProjectFiltersState,
  "search" | "topic" | "language" | "category" | "frontendOnly" | "hasDemo" | "aiOnly" | "era" | "sortBy" | "page" | "limit"
> = {
  search: "",
  topic: "",
  language: "",
  category: "",
  frontendOnly: false,
  hasDemo: false,
  aiOnly: true,
  era: "",
  sortBy: "score" as const,
  page: 1,
  limit: 12,
};

export const useProjectFiltersStore = create<ProjectFiltersState>((set) => ({
  ...defaultFilters,
  setSearch: (search) => set({ search, page: 1 }),
  setTopic: (topic) => set({ topic, page: 1 }),
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
  toggleAiOnly: () =>
    set((state) => ({
      aiOnly: !state.aiOnly,
      page: 1,
    })),
  setEra: (era) => set({ era, page: 1 }),
  setSortBy: (sortBy) => set({ sortBy, page: 1 }),
  previousPage: () => set((state) => ({ page: Math.max(1, state.page - 1) })),
  nextPage: () => set((state) => ({ page: state.page + 1 })),
  setLimit: (limit) => set({ limit, page: 1 }),
  resetFilters: () => set(defaultFilters),
}));
