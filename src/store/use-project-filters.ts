import { create } from "zustand";

type ProjectFiltersState = {
  frontendOnly: boolean;
  toggleFrontendOnly: () => void;
};

export const useProjectFiltersStore = create<ProjectFiltersState>((set) => ({
  frontendOnly: false,
  toggleFrontendOnly: () =>
    set((state) => ({
      frontendOnly: !state.frontendOnly,
    })),
}));