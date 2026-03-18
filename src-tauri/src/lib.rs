mod api;
mod commands;
mod config;
mod db;
mod models;
mod services;

use config::AppConfig;
use db::initialize_schema;

pub struct AppState {
    pub config: AppConfig,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = AppConfig::from_env();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { config })
        .setup(|_app| {
            initialize_schema().map_err(|error| error.to_string())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::health_check])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
