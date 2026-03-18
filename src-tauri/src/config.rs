#![allow(dead_code)]

use std::env;

use crate::api::{DEFAULT_GITHUB_API_BASE_URL, DEFAULT_MINIMAX_ANTHROPIC_BASE_URL, DEFAULT_MINIMAX_API_BASE_URL};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub github_api_base_url: String,
    pub github_token: Option<String>,
    pub minimax_api_key: Option<String>,
    pub minimax_base_url: String,
    pub minimax_anthropic_base_url: String,
    pub minimax_model: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            github_api_base_url: env::var("GITHUB_API_BASE_URL")
                .unwrap_or_else(|_| DEFAULT_GITHUB_API_BASE_URL.to_string()),
            github_token: env::var("GITHUB_TOKEN").ok(),
            minimax_api_key: env::var("MINIMAX_API_KEY").ok(),
            minimax_base_url: env::var("MINIMAX_BASE_URL")
                .unwrap_or_else(|_| DEFAULT_MINIMAX_API_BASE_URL.to_string()),
            minimax_anthropic_base_url: env::var("MINIMAX_ANTHROPIC_BASE_URL")
                .unwrap_or_else(|_| DEFAULT_MINIMAX_ANTHROPIC_BASE_URL.to_string()),
            minimax_model: env::var("MINIMAX_MODEL")
                .unwrap_or_else(|_| "MiniMax-M2.5".to_string()),
        }
    }
}