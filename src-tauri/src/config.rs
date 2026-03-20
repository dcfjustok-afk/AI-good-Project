use std::env;

use dotenvy::dotenv;

use crate::api::{DEFAULT_GITHUB_API_BASE_URL, DEFAULT_MINIMAX_API_BASE_URL};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub github_api_base_url: String,
    pub github_token: Option<String>,
    pub minimax_api_key: Option<String>,
    pub minimax_base_url: String,
    pub minimax_model: String,
    pub minimax_temperature: f32,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let _ = dotenv();

        Self {
            github_api_base_url: env::var("GITHUB_API_BASE_URL")
                .unwrap_or_else(|_| DEFAULT_GITHUB_API_BASE_URL.to_string()),
            github_token: read_optional_env("GITHUB_TOKEN"),
            minimax_api_key: read_optional_env("AI_API_KEY")
                .or_else(|| read_optional_env("MINIMAX_API_KEY")),
            minimax_base_url: read_env_with_fallback(&["AI_BASE_URL", "MINIMAX_BASE_URL"])
                .unwrap_or_else(|| DEFAULT_MINIMAX_API_BASE_URL.to_string()),
            minimax_model: read_env_with_fallback(&["AI_MODEL", "MINIMAX_MODEL"])
                .unwrap_or_else(|| "MiniMax-M2.5".to_string()),
            minimax_temperature: read_float_env_with_fallback(&["AI_TEMPERATURE", "MINIMAX_TEMPERATURE"])
                .unwrap_or(0.3),
        }
    }
}

fn read_optional_env(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn read_env_with_fallback(keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| read_optional_env(key))
}

fn read_float_env_with_fallback(keys: &[&str]) -> Option<f32> {
    keys.iter()
        .find_map(|key| read_optional_env(key))
        .and_then(|value| value.parse::<f32>().ok())
}