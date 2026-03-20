use tauri::State;

use crate::{
    db,
    logging,
    models::{
        AiProjectSectionsResponse, FavoriteToggleResponse, HealthCheckResponse, ProjectDetail,
        ProjectFilters, ProjectListResponse, ProjectSummary, SyncDataResponse,
    },
    services::sync::SyncService,
    AppState,
};

#[tauri::command]
pub async fn health_check(state: State<'_, AppState>) -> Result<HealthCheckResponse, String> {
    let snapshot = db::get_health_snapshot(&state.db_path).map_err(|error| {
        logging::error(&state.log_path, "health_check", &error.to_string());
        error.to_string()
    })?;

    Ok(HealthCheckResponse {
        status: "ready".to_string(),
        message: if state.config.minimax_api_key.is_some() {
            "基础设施已就绪，当前已检测到 OpenAI 兼容 AI Key，可直接尝试真实同步。".to_string()
        } else {
            "基础设施已就绪，当前未检测到 OpenAI 兼容 AI Key，同步时会自动回退到规则摘要。".to_string()
        },
        base_url: state.config.minimax_base_url.clone(),
        model: state.config.minimax_model.clone(),
        database_path: state.db_path.display().to_string(),
        log_path: state.log_path.display().to_string(),
        project_count: snapshot.project_count,
        favorite_count: snapshot.favorite_count,
        last_synced_at: snapshot.last_synced_at,
        github_token_configured: state.config.github_token.is_some(),
        minimax_api_key_configured: state.config.minimax_api_key.is_some(),
    })
}

#[tauri::command]
pub async fn get_projects(
    state: State<'_, AppState>,
    filters: Option<ProjectFilters>,
) -> Result<ProjectListResponse, String> {
    db::list_projects(&state.db_path, filters.unwrap_or_default()).map_err(|error| {
        logging::error(&state.log_path, "get_projects", &error.to_string());
        error.to_string()
    })
}

#[tauri::command]
pub async fn get_ai_project_sections(
    state: State<'_, AppState>,
    limit: Option<u32>,
) -> Result<AiProjectSectionsResponse, String> {
    db::list_ai_project_sections(&state.db_path, limit.unwrap_or(24)).map_err(|error| {
        logging::error(&state.log_path, "get_ai_project_sections", &error.to_string());
        error.to_string()
    })
}

#[tauri::command]
pub async fn get_project_detail(
    state: State<'_, AppState>,
    owner: String,
    repo: String,
) -> Result<ProjectDetail, String> {
    db::get_project_detail_by_repo(&state.db_path, &owner, &repo).map_err(|error| {
        logging::error(
            &state.log_path,
            "get_project_detail",
            &format!("failed to load {owner}/{repo}: {error}"),
        );
        error.to_string()
    })
}

#[tauri::command]
pub async fn toggle_favorite(
    state: State<'_, AppState>,
    project_id: i64,
) -> Result<FavoriteToggleResponse, String> {
    db::toggle_favorite(&state.db_path, project_id).map_err(|error| {
        logging::error(
            &state.log_path,
            "toggle_favorite",
            &format!("failed to toggle favorite for project {project_id}: {error}"),
        );
        error.to_string()
    })
}

#[tauri::command]
pub async fn get_favorites(state: State<'_, AppState>) -> Result<Vec<ProjectSummary>, String> {
    db::list_favorites(&state.db_path).map_err(|error| {
        logging::error(&state.log_path, "get_favorites", &error.to_string());
        error.to_string()
    })
}

#[tauri::command]
pub async fn sync_data(state: State<'_, AppState>) -> Result<SyncDataResponse, String> {
    logging::info(&state.log_path, "sync_data", "manual sync requested from frontend");

    SyncService::run(&state.config, &state.db_path)
        .await
        .map_err(|error| {
            logging::error(&state.log_path, "sync_data", &error.to_string());
            error.to_string()
        })
}
