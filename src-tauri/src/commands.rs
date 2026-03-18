use tauri::State;

use crate::{
    db,
    models::{
        FavoriteToggleResponse, HealthCheckResponse, ProjectDetail, ProjectFilters, ProjectSummary,
        SyncDataResponse,
    },
    services::sync::SyncService,
    AppState,
};

#[tauri::command]
pub async fn health_check(state: State<'_, AppState>) -> Result<HealthCheckResponse, String> {
    Ok(HealthCheckResponse {
        status: "ready".to_string(),
        message: "基础设施已就绪，后续阶段会在此基础上接入数据库、抓取链路与 AI 摘要。".to_string(),
        base_url: state.config.minimax_base_url.clone(),
        model: state.config.minimax_model.clone(),
        database_path: state.db_path.display().to_string(),
    })
}

#[tauri::command]
pub async fn get_projects(
    state: State<'_, AppState>,
    filters: Option<ProjectFilters>,
) -> Result<Vec<ProjectSummary>, String> {
    db::list_projects(&state.db_path, filters.unwrap_or_default()).map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_project_detail(
    state: State<'_, AppState>,
    owner: String,
    repo: String,
) -> Result<ProjectDetail, String> {
    db::get_project_detail_by_repo(&state.db_path, &owner, &repo).map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn toggle_favorite(
    state: State<'_, AppState>,
    project_id: i64,
) -> Result<FavoriteToggleResponse, String> {
    db::toggle_favorite(&state.db_path, project_id).map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_favorites(state: State<'_, AppState>) -> Result<Vec<ProjectSummary>, String> {
    db::list_favorites(&state.db_path).map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn sync_data(state: State<'_, AppState>) -> Result<SyncDataResponse, String> {
    SyncService::run(&state.config, &state.db_path)
        .await
        .map_err(|error| error.to_string())
}