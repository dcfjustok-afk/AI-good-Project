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
        let projects = api::fetch_trending_projects(config).await?;
        let processed = projects.len();
        let (inserted, updated) = db::upsert_projects(db_path, &projects)?;
        let used_ai = config.minimax_api_key.is_some();

        Ok(SyncDataResponse {
            processed,
            inserted,
            updated,
            used_ai,
            message: if used_ai {
                "同步完成，已使用 MiniMax 生成结构化摘要。".to_string()
            } else {
                "同步完成，当前未配置 MiniMax API Key，已使用规则摘要兜底。".to_string()
            },
        })
    }
}