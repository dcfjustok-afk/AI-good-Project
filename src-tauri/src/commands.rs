use tauri::State;

use crate::{models::HealthCheckResponse, AppState};

#[tauri::command]
pub async fn health_check(state: State<'_, AppState>) -> Result<HealthCheckResponse, String> {
    Ok(HealthCheckResponse {
        status: "ready".to_string(),
        message: "基础设施已就绪，后续阶段会在此基础上接入数据库、抓取链路与 AI 摘要。".to_string(),
        base_url: state.config.minimax_base_url.clone(),
        model: state.config.minimax_model.clone(),
    })
}