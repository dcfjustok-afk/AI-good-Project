#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    pub status: String,
    pub message: String,
    pub base_url: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSummary {
    pub id: i64,
    pub repo_name: String,
    pub description: String,
    pub language: Option<String>,
    pub stars: i64,
    pub forks: i64,
    pub updated_at: String,
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFilters {
    pub language: Option<String>,
    pub category: Option<String>,
    pub frontend_only: Option<bool>,
    pub has_demo: Option<bool>,
    pub sort_by: Option<String>,
}