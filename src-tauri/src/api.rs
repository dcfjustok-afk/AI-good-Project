#![allow(dead_code)]

pub const DEFAULT_GITHUB_API_BASE_URL: &str = "https://api.github.com";
pub const DEFAULT_MINIMAX_API_BASE_URL: &str = "https://api.minimaxi.com/v1";
pub const DEFAULT_MINIMAX_ANTHROPIC_BASE_URL: &str = "https://api.minimaxi.com/anthropic";

pub fn build_user_agent() -> String {
    format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}