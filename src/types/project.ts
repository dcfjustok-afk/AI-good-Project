export type HealthCheckResponse = {
  status: string;
  message: string;
  baseUrl: string;
  model: string;
  databasePath: string;
  logPath: string;
  projectCount: number;
  favoriteCount: number;
  lastSyncedAt: string | null;
  githubTokenConfigured: boolean;
  minimaxApiKeyConfigured: boolean;
};

export type ProjectFilters = {
  search?: string;
  topic?: string;
  language?: string;
  category?: string;
  frontendOnly?: boolean;
  favoritesOnly?: boolean;
  hasDemo?: boolean;
  aiOnly?: boolean;
  era?: "classic" | "latest";
  sortBy?: "score" | "stars" | "updatedAt" | "frontendRelevance" | "favoritedAt" | "impactRank";
  page?: number;
  limit?: number;
};

export type ProjectListResponse = {
  items: ProjectSummary[];
  total: number;
  page: number;
  pageSize: number;
  hasMore: boolean;
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
  score: number;
  summary: string;
  descriptionLong: string;
  topics: string[];
  demoUrl: string | null;
  isAI: boolean;
  era: "classic" | "latest";
  impactRank: number;
  frontendRelevance: number;
  isFavorite: boolean;
  favoriteCreatedAt: string | null;
};

export type AiProjectSectionsResponse = {
  classic: ProjectSummary[];
  latest: ProjectSummary[];
  classicTotal: number;
  latestTotal: number;
  invalidEraCount: number;
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
  score: number;
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
