export type HealthCheckResponse = {
  status: string;
  message: string;
  baseUrl: string;
  model: string;
};

export type ProjectCardViewModel = {
  name: string;
  owner: string;
  repo: string;
  category: string;
  frontendFitLabel: string;
  summary: string;
  tags: string[];
  stars: string;
  forks: string;
  language: string;
  updatedAt: string;
};