use std::path::Path;

use anyhow::Result;

use crate::{
    api,
    config::AppConfig,
    db,
    models::SyncDataResponse,
};

pub struct SyncService;

impl SyncService {
    pub async fn run(config: &AppConfig, db_path: &Path) -> Result<SyncDataResponse> {
        let sync_result = api::fetch_trending_projects(config).await?;
        let processed = sync_result.projects.len();
        let (inserted, updated) = db::upsert_projects(db_path, &sync_result.projects)?;
        let used_ai = config.minimax_api_key.is_some();
        let used_fallback = !used_ai
            || sync_result.ai_fallback_count > 0
            || sync_result.github_requests_failed > 0;

        let message = if sync_result.github_requests_failed > 0 {
            format!(
                "同步完成，但有 {} 个 GitHub 查询失败；其余结果已写入本地数据库。",
                sync_result.github_requests_failed
            )
        } else if sync_result.ai_fallback_count > 0 {
            format!(
                "同步完成，其中 {} 个仓库的 AI 摘要失败，已自动回退到规则摘要。",
                sync_result.ai_fallback_count
            )
        } else if used_ai {
            "同步完成，已使用 MiniMax 生成结构化摘要。".to_string()
        } else {
            "同步完成，当前未配置 MiniMax API Key，已使用规则摘要兜底。".to_string()
        };

        Ok(SyncDataResponse {
            processed,
            inserted,
            updated,
            used_ai,
            used_fallback,
            github_requests_failed: sync_result.github_requests_failed,
            ai_fallback_count: sync_result.ai_fallback_count,
            message,
            warnings: sync_result.warnings,
        })
    }
}