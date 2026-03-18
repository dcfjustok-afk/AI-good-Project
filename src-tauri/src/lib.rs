mod api;
mod commands;
mod config;
mod db;
mod models;
mod services;

use config::AppConfig;
use db::initialize_schema;
use std::path::PathBuf;
use tauri::Manager;

pub struct AppState {
    pub config: AppConfig,
    pub db_path: PathBuf,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = AppConfig::from_env();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let db_path = initialize_schema(app.handle().clone()).map_err(|error| error.to_string())?;

            app.manage(AppState {
                config: config.clone(),
                db_path,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::health_check,
            commands::get_projects,
            commands::get_project_detail,
            commands::toggle_favorite,
            commands::get_favorites,
            commands::sync_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
