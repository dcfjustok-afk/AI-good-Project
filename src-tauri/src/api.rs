use std::{collections::HashMap, time::Duration};

use anyhow::{anyhow, Context, Result};
use reqwest::{header, Client};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::json;
use tokio::time::sleep;

use crate::{config::AppConfig, models::SyncedProject};

pub const DEFAULT_GITHUB_API_BASE_URL: &str = "https://api.github.com";
pub const DEFAULT_MINIMAX_API_BASE_URL: &str = "https://api.minimaxi.com/v1";

const SEARCH_KEYWORDS: [&str; 4] = ["ai agent", "llm", "rag", "open source ai"];
const CATEGORY_AGENT: &str = "Agent Framework";
const CATEGORY_RAG: &str = "RAG / Search";
const CATEGORY_FRONTEND: &str = "AI UI / Frontend";
const CATEGORY_WORKFLOW: &str = "Workflow Automation";
const CATEGORY_MULTIMODAL: &str = "Speech / Multimodal";
const CATEGORY_DEVTOOLS: &str = "Developer Tooling";
const CATEGORY_LLM_APP: &str = "LLM Application";
const MAX_RETRIES: usize = 3;

pub struct FetchProjectsResult {
    pub projects: Vec<SyncedProject>,
    pub github_requests_failed: usize,
    pub ai_fallback_count: usize,
    pub warnings: Vec<String>,
}

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

pub async fn fetch_trending_projects(config: &AppConfig) -> Result<FetchProjectsResult> {
    let client = build_client(config)?;
    let mut repositories = HashMap::new();
    let mut warnings = Vec::new();
    let mut github_requests_failed = 0;
    let mut ai_fallback_count = 0;

    for keyword in SEARCH_KEYWORDS {
        match fetch_github_repositories(&client, config, keyword).await {
            Ok(response) => {
                for item in response.items {
                    repositories.entry(item.full_name.clone()).or_insert(item);
                }
            }
            Err(error) => {
                github_requests_failed += 1;
                warnings.push(format!("GitHub 关键词 {keyword} 抓取失败：{error}"));
            }
        }
    }

    if repositories.is_empty() {
        return Err(anyhow!("所有 GitHub 抓取请求都失败了，请检查网络连通性或 GITHUB_TOKEN 配置"));
    }

    let mut projects = Vec::new();
    for repository in repositories.into_values() {
        let summary = match try_generate_summary(config, &client, &repository).await {
            Ok(summary) => summary,
            Err(error) => {
                ai_fallback_count += 1;
                warnings.push(format!(
                    "仓库 {} 的 AI 摘要生成失败，已自动回退：{}",
                    repository.full_name, error
                ));
                rule_based_summary(&repository)
            }
        };
        let topics = repository.topics.clone().unwrap_or_default();
        let category = normalize_category(&summary.category, &repository);
        let score = calculate_score(&repository, summary.frontend_relevance);

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
            topics,
            category,
            score,
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
    Ok(FetchProjectsResult {
        projects,
        github_requests_failed,
        ai_fallback_count,
        warnings,
    })
}

async fn fetch_github_repositories(
    client: &Client,
    config: &AppConfig,
    keyword: &str,
) -> Result<GitHubSearchResponse> {
    let query = format!("{keyword} in:name,description,readme stars:>500 archived:false")
        .replace(' ', "%20");
    let url = format!(
        "{}/search/repositories?q={query}&sort=stars&order=desc&per_page=6",
        config.github_api_base_url.trim_end_matches('/')
    );

    get_json_with_retry(client, &url, &format!("GitHub repositories for keyword: {keyword}")).await
}

async fn try_generate_summary(
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

    let response: OpenAiChatResponse = post_json_with_retry(
        client,
        &format!("{}/chat/completions", config.minimax_base_url.trim_end_matches('/')),
        config.minimax_api_key.as_deref().unwrap_or_default(),
        &json!({
            "model": config.minimax_model,
            "temperature": config.minimax_temperature,
            "response_format": { "type": "json_object" },
            "messages": [
                { "role": "system", "content": "你输出的必须是合法 JSON，且适合中文产品情报站直接展示。" },
                { "role": "user", "content": prompt }
            ]
        }),
        &format!("AI summary for {}", repository.full_name),
    )
    .await?;

    let content = response
        .choices
        .first()
        .map(|choice| choice.message.content.trim().to_string())
        .filter(|content| !content.is_empty())
        .context("AI provider response did not contain summary content")?;

    serde_json::from_str::<GeneratedSummary>(&content).or_else(|_| Ok(rule_based_summary(repository)))
}

async fn get_json_with_retry<T>(client: &Client, url: &str, context_label: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    for attempt in 1..=MAX_RETRIES {
        match client.get(url).send().await {
            Ok(response) => {
                let status = response.status();

                if status.is_success() {
                    return response
                        .json::<T>()
                        .await
                        .with_context(|| format!("failed to parse response for {context_label}"));
                }

                let body = response.text().await.unwrap_or_default();
                if attempt < MAX_RETRIES && should_retry_status(status.as_u16()) {
                    sleep(retry_delay(attempt)).await;
                    continue;
                }

                return Err(anyhow!(
                    "request for {context_label} failed with status {}: {}",
                    status,
                    summarize_error_body(&body)
                ));
            }
            Err(error) => {
                if attempt < MAX_RETRIES && should_retry_error(&error) {
                    sleep(retry_delay(attempt)).await;
                    continue;
                }

                return Err(error).with_context(|| format!("failed to request {context_label}"));
            }
        }
    }

    Err(anyhow!("request retry loop unexpectedly exhausted for {context_label}"))
}

async fn post_json_with_retry<T>(
    client: &Client,
    url: &str,
    bearer_token: &str,
    payload: &serde_json::Value,
    context_label: &str,
) -> Result<T>
where
    T: DeserializeOwned,
{
    for attempt in 1..=MAX_RETRIES {
        match client
            .post(url)
            .bearer_auth(bearer_token)
            .json(payload)
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();

                if status.is_success() {
                    return response
                        .json::<T>()
                        .await
                        .with_context(|| format!("failed to parse response for {context_label}"));
                }

                let body = response.text().await.unwrap_or_default();
                if attempt < MAX_RETRIES && should_retry_status(status.as_u16()) {
                    sleep(retry_delay(attempt)).await;
                    continue;
                }

                return Err(anyhow!(
                    "request for {context_label} failed with status {}: {}",
                    status,
                    summarize_error_body(&body)
                ));
            }
            Err(error) => {
                if attempt < MAX_RETRIES && should_retry_error(&error) {
                    sleep(retry_delay(attempt)).await;
                    continue;
                }

                return Err(error).with_context(|| format!("failed to request {context_label}"));
            }
        }
    }

    Err(anyhow!("request retry loop unexpectedly exhausted for {context_label}"))
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

fn calculate_score(repository: &GitHubRepositoryItem, frontend_relevance: i64) -> i64 {
    let topics = repository.topics.clone().unwrap_or_default();
    let stars_score = (repository.stargazers_count / 250).min(400);
    let forks_score = (repository.forks_count / 80).min(180);
    let issue_penalty = (repository.open_issues_count / 30).min(20);
    let recency_score = recency_score(&repository.updated_at);
    let topic_score = topics
        .iter()
        .map(|topic| match topic.as_str() {
            "agent" | "multi-agent" | "workflow" => 8,
            "rag" | "retrieval" | "search" => 7,
            "frontend" | "chat-ui" | "ui" => 7,
            "multimodal" | "speech" | "voice" => 6,
            _ => 3,
        })
        .sum::<i64>()
        .min(28);
    let language_score = match repository.language.as_deref() {
        Some("TypeScript") | Some("JavaScript") => 16,
        Some("Python") => 10,
        Some("Rust") => 12,
        _ => 6,
    };
    let demo_bonus = if repository.homepage.as_ref().is_some_and(|value| !value.trim().is_empty()) {
        10
    } else {
        0
    };

    (stars_score + forks_score + recency_score + topic_score + language_score + demo_bonus
        + frontend_relevance * 14
        - issue_penalty)
        .max(1)
}

fn should_retry_status(status: u16) -> bool {
    status == 408 || status == 425 || status == 429 || status >= 500
}

fn should_retry_error(error: &reqwest::Error) -> bool {
    error.is_timeout() || error.is_connect() || error.is_request()
}

fn retry_delay(attempt: usize) -> Duration {
    Duration::from_millis(350 * attempt as u64)
}

fn recency_score(updated_at: &str) -> i64 {
    let updated_at = chrono::DateTime::parse_from_rfc3339(updated_at)
        .map(|value| value.with_timezone(&chrono::Utc))
        .ok();

    let Some(updated_at) = updated_at else {
        return 8;
    };

    let days = (chrono::Utc::now() - updated_at).num_days();
    if days <= 7 {
        26
    } else if days <= 30 {
        20
    } else if days <= 90 {
        12
    } else {
        5
    }
}

fn summarize_error_body(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        "empty response body".to_string()
    } else {
        trimmed.chars().take(180).collect()
    }
}

fn rule_based_summary(repository: &GitHubRepositoryItem) -> GeneratedSummary {
    let topics = repository.topics.clone().unwrap_or_default();
    let category = classify_repository(repository);

    let frontend_relevance = if category == CATEGORY_FRONTEND {
        3
    } else if matches!(category, CATEGORY_AGENT | CATEGORY_WORKFLOW | CATEGORY_RAG) {
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

fn normalize_category(input: &str, repository: &GitHubRepositoryItem) -> String {
    let normalized = input.trim().to_lowercase();

    if normalized.contains("agent") {
        CATEGORY_AGENT.to_string()
    } else if normalized.contains("rag") || normalized.contains("search") || normalized.contains("retrieval") {
        CATEGORY_RAG.to_string()
    } else if normalized.contains("front") || normalized.contains("ui") || normalized.contains("chat") {
        CATEGORY_FRONTEND.to_string()
    } else if normalized.contains("workflow") || normalized.contains("automation") {
        CATEGORY_WORKFLOW.to_string()
    } else if normalized.contains("multimodal") || normalized.contains("speech") || normalized.contains("voice") {
        CATEGORY_MULTIMODAL.to_string()
    } else if normalized.contains("tool") || normalized.contains("sdk") || normalized.contains("framework") {
        CATEGORY_DEVTOOLS.to_string()
    } else {
        classify_repository(repository).to_string()
    }
}

fn classify_repository(repository: &GitHubRepositoryItem) -> &'static str {
    let description = repository
        .description
        .as_deref()
        .unwrap_or_default()
        .to_lowercase();
    let topics = repository.topics.clone().unwrap_or_default();
    let has_topic = |keywords: &[&str]| {
        topics.iter().any(|topic| {
            let topic = topic.to_lowercase();
            keywords.iter().any(|keyword| topic.contains(keyword))
        })
    };

    if has_topic(&["frontend", "chat-ui", "ui", "webui"]) || description.contains("chat interface") {
        CATEGORY_FRONTEND
    } else if has_topic(&["agent", "multi-agent", "autonomous"]) || description.contains("agent") {
        CATEGORY_AGENT
    } else if has_topic(&["rag", "retrieval", "search"]) || description.contains("retrieval") {
        CATEGORY_RAG
    } else if has_topic(&["workflow", "automation", "orchestrator"]) || description.contains("workflow") {
        CATEGORY_WORKFLOW
    } else if has_topic(&["speech", "voice", "multimodal", "audio"]) || description.contains("voice") {
        CATEGORY_MULTIMODAL
    } else if has_topic(&["sdk", "framework", "tooling", "developer-tools"]) || description.contains("framework") {
        CATEGORY_DEVTOOLS
    } else {
        CATEGORY_LLM_APP
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_repo(topics: &[&str], language: Option<&str>, description: &str, updated_at: &str) -> GitHubRepositoryItem {
        GitHubRepositoryItem {
            full_name: "demo/repo".to_string(),
            name: "repo".to_string(),
            description: Some(description.to_string()),
            html_url: "https://github.com/demo/repo".to_string(),
            homepage: Some("https://demo.example.com".to_string()),
            language: language.map(str::to_string),
            stargazers_count: 12000,
            forks_count: 1600,
            open_issues_count: 32,
            topics: Some(topics.iter().map(|value| (*value).to_string()).collect()),
            updated_at: updated_at.to_string(),
            owner: GitHubOwner {
                login: "demo".to_string(),
            },
            license: None,
        }
    }

    #[test]
    fn classify_repository_prefers_frontend_category() {
        let repository = sample_repo(
            &["frontend", "chat-ui", "rag"],
            Some("TypeScript"),
            "Open source chat interface for local models",
            "2026-03-15T10:00:00Z",
        );

        assert_eq!(classify_repository(&repository), CATEGORY_FRONTEND);
    }

    #[test]
    fn calculate_score_rewards_recency_and_frontend_relevance() {
        let recent = sample_repo(
            &["agent", "workflow"],
            Some("TypeScript"),
            "Agent workflow orchestration",
            "2026-03-18T10:00:00Z",
        );
        let stale = sample_repo(
            &["agent", "workflow"],
            Some("TypeScript"),
            "Agent workflow orchestration",
            "2025-01-01T10:00:00Z",
        );

        assert!(calculate_score(&recent, 3) > calculate_score(&stale, 1));
    }
}