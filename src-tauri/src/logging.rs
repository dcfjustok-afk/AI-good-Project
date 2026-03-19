use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use chrono::Utc;
use tauri::{AppHandle, Manager};

pub const LOG_FILE_NAME: &str = "ai-good-project.log";

pub fn initialize_log_file(app: AppHandle) -> Result<PathBuf> {
    let log_dir = app
        .path()
        .app_data_dir()
        .context("failed to resolve app data directory for logs")?
        .join("logs");

    fs::create_dir_all(&log_dir).context("failed to create log directory")?;

    let log_path = log_dir.join(LOG_FILE_NAME);
    if !log_path.exists() {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .with_context(|| format!("failed to create log file at {}", log_path.display()))?;
    }

    Ok(log_path)
}

pub fn info(log_path: &Path, scope: &str, message: &str) {
    let _ = append(log_path, "INFO", scope, message);
}

pub fn warn(log_path: &Path, scope: &str, message: &str) {
    let _ = append(log_path, "WARN", scope, message);
}

pub fn error(log_path: &Path, scope: &str, message: &str) {
    let _ = append(log_path, "ERROR", scope, message);
}

fn append(log_path: &Path, level: &str, scope: &str, message: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .with_context(|| format!("failed to open log file at {}", log_path.display()))?;

    writeln!(
        file,
        "{} [{}] [{}] {}",
        Utc::now().to_rfc3339(),
        level,
        scope,
        message.replace('\n', " ")
    )
    .context("failed to append application log")?;

    Ok(())
}