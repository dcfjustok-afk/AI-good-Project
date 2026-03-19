use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    pub status: String,
    pub message: String,
    pub base_url: String,
    pub model: String,
    pub database_path: String,
    pub log_path: String,
    pub github_token_configured: bool,
    pub minimax_api_key_configured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectListResponse {
    pub items: Vec<ProjectSummary>,
    pub total: usize,
    pub page: u32,
    pub page_size: u32,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSummary {
    pub id: i64,
    pub owner: String,
    pub repo: String,
    pub repo_name: String,
    pub description: String,
    pub language: Option<String>,
    pub stars: i64,
    pub forks: i64,
    pub updated_at: String,
    pub category: String,
    pub frontend_relevance: i64,
    pub summary: String,
    pub topics: Vec<String>,
    pub demo_url: Option<String>,
    pub is_favorite: bool,
    pub favorite_created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDetail {
    pub id: i64,
    pub owner: String,
    pub repo: String,
    pub repo_name: String,
    pub description: String,
    pub github_url: String,
    pub homepage_url: Option<String>,
    pub demo_url: Option<String>,
    pub language: Option<String>,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub updated_at: String,
    pub category: String,
    pub frontend_relevance: i64,
    pub summary: String,
    pub highlights: Vec<String>,
    pub use_cases: Vec<String>,
    pub frontend_value: String,
    pub learning_cost: String,
    pub topics: Vec<String>,
    pub license: Option<String>,
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteToggleResponse {
    pub project_id: i64,
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncDataResponse {
    pub processed: usize,
    pub inserted: usize,
    pub updated: usize,
    pub used_ai: bool,
    pub used_fallback: bool,
    pub github_requests_failed: usize,
    pub ai_fallback_count: usize,
    pub message: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SyncedProject {
    pub owner: String,
    pub repo: String,
    pub repo_name: String,
    pub description: String,
    pub github_url: String,
    pub homepage_url: Option<String>,
    pub demo_url: Option<String>,
    pub language: Option<String>,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub topics: Vec<String>,
    pub category: String,
    pub score: i64,
    pub license: Option<String>,
    pub updated_at: String,
    pub summary: String,
    pub highlights: Vec<String>,
    pub use_cases: Vec<String>,
    pub frontend_value: String,
    pub learning_cost: String,
    pub frontend_relevance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFilters {
    pub language: Option<String>,
    pub category: Option<String>,
    pub frontend_only: Option<bool>,
    pub favorites_only: Option<bool>,
    pub has_demo: Option<bool>,
    pub sort_by: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}