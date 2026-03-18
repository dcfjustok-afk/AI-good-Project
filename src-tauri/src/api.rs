use std::{collections::HashMap, time::Duration};

use anyhow::{Context, Result};
use reqwest::{header, Client};
use serde::Deserialize;
use serde_json::json;

use crate::{config::AppConfig, models::SyncedProject};

pub const DEFAULT_GITHUB_API_BASE_URL: &str = "https://api.github.com";
pub const DEFAULT_MINIMAX_API_BASE_URL: &str = "https://api.minimaxi.com/v1";
pub const DEFAULT_MINIMAX_ANTHROPIC_BASE_URL: &str = "https://api.minimaxi.com/anthropic";

const SEARCH_KEYWORDS: [&str; 4] = ["ai agent", "llm", "rag", "open source ai"];

pub fn build_user_agent() -> String {
    format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[derive(Debug, Deserialize)]
struct GitHubSearchResponse {
    items: Vec<GitHubRepositoryItem>,
}

#[derive(Debug, Clone, Deserialize)]
struct GitHubRepositoryItem {
    full_name: String,
    name: String,
    description: Option<String>,
    html_url: String,
    homepage: Option<String>,
    language: Option<String>,
    stargazers_count: i64,
    forks_count: i64,
    open_issues_count: i64,
    topics: Option<Vec<String>>,
    updated_at: String,
    owner: GitHubOwner,
    license: Option<GitHubLicense>,
}

#[derive(Debug, Clone, Deserialize)]
struct GitHubOwner {
    login: String,
}

#[derive(Debug, Clone, Deserialize)]
struct GitHubLicense {
    spdx_id: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChatResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAiMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeneratedSummary {
    summary: String,
    highlights: Vec<String>,
    use_cases: Vec<String>,
    frontend_value: String,
    learning_cost: String,
    frontend_relevance: i64,
    category: String,
}

pub async fn fetch_trending_projects(config: &AppConfig) -> Result<Vec<SyncedProject>> {
    let client = build_client(config)?;
    let mut repositories = HashMap::new();

    for keyword in SEARCH_KEYWORDS {
        let query = format!("{keyword} in:name,description,readme stars:>500 archived:false")
            .replace(' ', "%20");
        let url = format!(
            "{}/search/repositories?q={query}&sort=stars&order=desc&per_page=6",
            config.github_api_base_url.trim_end_matches('/')
        );

        let response = client
            .get(url)
            .send()
            .await
            .with_context(|| format!("failed to fetch GitHub repositories for keyword: {keyword}"))?
            .error_for_status()
            .with_context(|| format!("GitHub search request failed for keyword: {keyword}"))?
            .json::<GitHubSearchResponse>()
            .await
            .context("failed to parse GitHub search response")?;

        for item in response.items {
            repositories.entry(item.full_name.clone()).or_insert(item);
        }
    }

    let mut projects = Vec::new();
    for repository in repositories.into_values() {
        let summary = summarize_project(config, &client, &repository).await?;
        projects.push(SyncedProject {
            owner: repository.owner.login,
            repo: repository.name,
            repo_name: repository.full_name,
            description: repository.description.unwrap_or_else(|| "暂无仓库描述".to_string()),
            github_url: repository.html_url,
            homepage_url: repository.homepage.clone(),
            demo_url: repository.homepage,
            language: repository.language,
            stars: repository.stargazers_count,
            forks: repository.forks_count,
            open_issues: repository.open_issues_count,
            topics: repository.topics.unwrap_or_default(),
            category: summary.category,
            score: calculate_score(repository.stargazers_count, repository.forks_count, summary.frontend_relevance),
            license: repository.license.and_then(|license| license.spdx_id.or(license.name)),
            updated_at: repository.updated_at,
            summary: summary.summary,
            highlights: summary.highlights,
            use_cases: summary.use_cases,
            frontend_value: summary.frontend_value,
            learning_cost: summary.learning_cost,
            frontend_relevance: summary.frontend_relevance.clamp(1, 3),
        });
    }

    projects.sort_by(|left, right| right.score.cmp(&left.score));
    Ok(projects)
}

async fn summarize_project(
    config: &AppConfig,
    client: &Client,
    repository: &GitHubRepositoryItem,
) -> Result<GeneratedSummary> {
    if config.minimax_api_key.is_none() {
        return Ok(rule_based_summary(repository));
    }

    let prompt = format!(
        "你是 AI 开源项目情报站的数据摘要器。请基于以下仓库信息输出 JSON，不要带 markdown。\n仓库：{}\n描述：{}\n语言：{}\n主题：{}\n要求字段：summary, highlights(3项), useCases(2项), frontendValue, learningCost, frontendRelevance(1-3), category。",
        repository.full_name,
        repository.description.clone().unwrap_or_else(|| "暂无描述".to_string()),
        repository.language.clone().unwrap_or_else(|| "未知".to_string()),
        repository.topics.clone().unwrap_or_default().join(", ")
    );

    let response = client
        .post(format!("{}/chat/completions", config.minimax_base_url.trim_end_matches('/')))
        .bearer_auth(config.minimax_api_key.clone().unwrap_or_default())
        .json(&json!({
            "model": config.minimax_model,
            "temperature": 0.3,
            "response_format": { "type": "json_object" },
            "messages": [
                { "role": "system", "content": "你输出的必须是合法 JSON，且适合中文产品情报站直接展示。" },
                { "role": "user", "content": prompt }
            ]
        }))
        .send()
        .await
        .context("failed to request MiniMax summary")?
        .error_for_status()
        .context("MiniMax summary request returned error status")?
        .json::<OpenAiChatResponse>()
        .await
        .context("failed to parse MiniMax response")?;

    let content = response
        .choices
        .first()
        .map(|choice| choice.message.content.trim().to_string())
        .filter(|content| !content.is_empty())
        .context("MiniMax response did not contain summary content")?;

    serde_json::from_str::<GeneratedSummary>(&content).or_else(|_| Ok(rule_based_summary(repository)))
}

fn build_client(config: &AppConfig) -> Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_str(&build_user_agent()).context("invalid user agent header")?,
    );
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github+json"),
    );

    if let Some(token) = &config.github_token {
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {token}"))
                .context("invalid GitHub authorization header")?,
        );
    }

    Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(20))
        .build()
        .context("failed to build HTTP client")
}

fn calculate_score(stars: i64, forks: i64, frontend_relevance: i64) -> i64 {
    stars / 200 + forks / 100 + frontend_relevance * 10
}

fn rule_based_summary(repository: &GitHubRepositoryItem) -> GeneratedSummary {
    let topics = repository.topics.clone().unwrap_or_default();
    let category = if topics.iter().any(|topic| topic.contains("agent")) {
        "AI Agent"
    } else if topics.iter().any(|topic| topic.contains("rag")) {
        "RAG"
    } else if repository
        .language
        .as_deref()
        .is_some_and(|language| matches!(language, "TypeScript" | "JavaScript"))
    {
        "AI UI / Frontend"
    } else {
        "LLM 应用"
    };

    let frontend_relevance = if category == "AI UI / Frontend" {
        3
    } else if topics.iter().any(|topic| topic.contains("agent") || topic.contains("workflow")) {
        2
    } else {
        1
    };

    GeneratedSummary {
        summary: repository
            .description
            .clone()
            .unwrap_or_else(|| format!("{} 是近期值得跟踪的 AI 开源项目。", repository.full_name)),
        highlights: vec![
            format!("仓库星标达到 {}，社区关注度较高。", repository.stargazers_count),
            format!("主要语言为 {}。", repository.language.clone().unwrap_or_else(|| "未知".to_string())),
            if topics.is_empty() {
                "适合后续在同步链路中补充更细的主题标签。".to_string()
            } else {
                format!("主题覆盖 {}。", topics.join(" / "))
            },
        ],
        use_cases: vec![
            "用于追踪 AI 开源趋势与竞品动向。".to_string(),
            "作为团队内部技术调研与分享素材。".to_string(),
        ],
        frontend_value: if frontend_relevance >= 2 {
            "具备明显的前端界面、工作流或交互设计参考价值。".to_string()
        } else {
            "更偏后端能力，但仍可作为产品能力和接口形态参考。".to_string()
        },
        learning_cost: if repository.language.as_deref() == Some("TypeScript") {
            "低".to_string()
        } else {
            "中".to_string()
        },
        frontend_relevance,
        category: category.to_string(),
    }
}