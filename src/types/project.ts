export type HealthCheckResponse = {
  status: string;
  message: string;
  baseUrl: string;
  model: string;
  databasePath: string;
  githubTokenConfigured: boolean;
  minimaxApiKeyConfigured: boolean;
};

export type ProjectFilters = {
  language?: string;
  category?: string;
  frontendOnly?: boolean;
  favoritesOnly?: boolean;
  hasDemo?: boolean;
  sortBy?: "score" | "stars" | "updatedAt" | "frontendRelevance" | "favoritedAt";
  limit?: number;
};

export type ProjectSummary = {
  id: number;
  owner: string;
  repo: string;
  repoName: string;
  description: string;
  language: string | null;
  stars: number;
  forks: number;
  updatedAt: string;
  category: string;
  summary: string;
  topics: string[];
  demoUrl: string | null;
  frontendRelevance: number;
  isFavorite: boolean;
  favoriteCreatedAt: string | null;
};

export type ProjectDetail = {
  id: number;
  owner: string;
  repo: string;
  repoName: string;
  description: string;
  githubUrl: string;
  homepageUrl: string | null;
  demoUrl: string | null;
  language: string | null;
  stars: number;
  forks: number;
  openIssues: number;
  updatedAt: string;
  category: string;
  frontendRelevance: number;
  summary: string;
  highlights: string[];
  useCases: string[];
  frontendValue: string;
  learningCost: string;
  topics: string[];
  license: string | null;
  isFavorite: boolean;
};

export type FavoriteToggleResponse = {
  projectId: number;
  isFavorite: boolean;
};

export type SyncDataResponse = {
  processed: number;
  inserted: number;
  updated: number;
  usedAi: boolean;
  usedFallback: boolean;
  githubRequestsFailed: number;
  aiFallbackCount: number;
  message: string;
  warnings: string[];
};