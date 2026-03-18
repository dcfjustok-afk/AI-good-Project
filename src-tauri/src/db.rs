use std::{fs, path::{Path, PathBuf}};

use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use tauri::{AppHandle, Manager};

use crate::models::{
    FavoriteToggleResponse, ProjectDetail, ProjectFilters, ProjectSummary, SyncedProject,
};

pub const DATABASE_FILE_NAME: &str = "ai-good-project.db";

type SeedProject<'a> = (
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    i64,
    i64,
    i64,
    &'a [&'a str],
    &'a str,
    i64,
    &'a str,
    &'a [&'a str],
    &'a [&'a str],
    &'a str,
    &'a str,
    Option<&'a str>,
    Option<&'a str>,
    &'a str,
);

pub fn initialize_schema(app: AppHandle) -> Result<PathBuf> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .context("failed to resolve app data directory")?;

    fs::create_dir_all(&app_data_dir).context("failed to create app data directory")?;

    let db_path = app_data_dir.join(DATABASE_FILE_NAME);
    let connection = open_connection(&db_path)?;

    create_schema(&connection)?;
    seed_sample_projects(&connection)?;

    Ok(db_path)
}

pub fn list_projects(db_path: &Path, filters: ProjectFilters) -> Result<Vec<ProjectSummary>> {
    let connection = open_connection(db_path)?;
    let limit = i64::from(filters.limit.unwrap_or(20));

    let sort_by = match filters.sort_by.as_deref() {
        Some("stars") => "p.stars DESC, p.updated_at DESC",
        Some("updatedAt") => "p.updated_at DESC, p.stars DESC",
        Some("frontendRelevance") => "COALESCE(s.frontend_relevance, 0) DESC, p.stars DESC",
        Some("favoritedAt") => "COALESCE(f.created_at, '') DESC, p.updated_at DESC",
        _ => "p.score DESC, p.stars DESC, p.updated_at DESC",
    };

    let query = format!(
        r#"
        SELECT
            p.id,
            p.owner,
            p.name,
            p.repo_name,
            p.description,
            p.language,
            p.stars,
            p.forks,
            p.updated_at,
            p.category,
            COALESCE(s.frontend_relevance, 0) AS frontend_relevance,
            COALESCE(s.summary, p.description) AS summary,
            p.topics,
            p.demo_url,
            CASE WHEN f.repo_id IS NULL THEN 0 ELSE 1 END AS is_favorite,
            f.created_at AS favorite_created_at
        FROM projects p
        LEFT JOIN summaries s ON s.repo_id = p.id
        LEFT JOIN favorites f ON f.repo_id = p.id
        WHERE (?1 IS NULL OR p.language = ?1)
          AND (?2 IS NULL OR p.category = ?2)
          AND (?3 = 0 OR COALESCE(s.frontend_relevance, 0) >= 2)
          AND (?4 = 0 OR (p.demo_url IS NOT NULL AND p.demo_url <> ''))
          AND (?5 = 0 OR f.repo_id IS NOT NULL)
        ORDER BY {}
        LIMIT ?6
        "#,
        sort_by
    );

    let mut statement = connection.prepare(&query)?;
    let rows = statement.query_map(
        params![
            filters.language,
            filters.category,
            bool_to_i64(filters.frontend_only.unwrap_or(false)),
            bool_to_i64(filters.has_demo.unwrap_or(false)),
            bool_to_i64(filters.favorites_only.unwrap_or(false)),
            limit,
        ],
        map_project_summary,
    )?;

    rows.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
}

pub fn list_favorites(db_path: &Path) -> Result<Vec<ProjectSummary>> {
    list_projects(
        db_path,
        ProjectFilters {
            favorites_only: Some(true),
            sort_by: Some("favoritedAt".to_string()),
            limit: Some(50),
            ..ProjectFilters::default()
        },
    )
}

pub fn get_project_detail_by_repo(db_path: &Path, owner: &str, repo: &str) -> Result<ProjectDetail> {
    let connection = open_connection(db_path)?;
    let mut statement = connection.prepare(
        r#"
        SELECT
            p.id,
            p.owner,
            p.name,
            p.repo_name,
            p.description,
            p.github_url,
            p.homepage_url,
            p.demo_url,
            p.language,
            p.stars,
            p.forks,
            p.open_issues,
            p.updated_at,
            p.category,
            COALESCE(s.frontend_relevance, 0) AS frontend_relevance,
            COALESCE(s.summary, p.description) AS summary,
            COALESCE(s.highlights, '[]') AS highlights,
            COALESCE(s.use_cases, '[]') AS use_cases,
            COALESCE(s.frontend_value, '') AS frontend_value,
            COALESCE(s.learning_cost, '中') AS learning_cost,
            p.topics,
            p.license,
            CASE WHEN f.repo_id IS NULL THEN 0 ELSE 1 END AS is_favorite
        FROM projects p
        LEFT JOIN summaries s ON s.repo_id = p.id
        LEFT JOIN favorites f ON f.repo_id = p.id
        WHERE p.owner = ?1 AND p.name = ?2
        LIMIT 1
        "#,
    )?;

    let detail = statement
        .query_row(params![owner, repo], |row| {
            Ok(ProjectDetail {
                id: row.get(0)?,
                owner: row.get(1)?,
                repo: row.get(2)?,
                repo_name: row.get(3)?,
                description: row.get(4)?,
                github_url: row.get(5)?,
                homepage_url: row.get(6)?,
                demo_url: row.get(7)?,
                language: row.get(8)?,
                stars: row.get(9)?,
                forks: row.get(10)?,
                open_issues: row.get(11)?,
                updated_at: row.get(12)?,
                category: row.get(13)?,
                frontend_relevance: row.get(14)?,
                summary: row.get(15)?,
                highlights: parse_json_vec(&row.get::<_, String>(16)?),
                use_cases: parse_json_vec(&row.get::<_, String>(17)?),
                frontend_value: row.get(18)?,
                learning_cost: row.get(19)?,
                topics: parse_json_vec(&row.get::<_, String>(20)?),
                license: row.get(21)?,
                is_favorite: row.get::<_, i64>(22)? == 1,
            })
        })
        .optional()?
        .ok_or_else(|| anyhow!("project not found: {owner}/{repo}"))?;

    Ok(detail)
}

pub fn toggle_favorite(db_path: &Path, project_id: i64) -> Result<FavoriteToggleResponse> {
    let connection = open_connection(db_path)?;
    let transaction = connection.unchecked_transaction()?;

    let is_favorite = transaction
        .query_row(
            "SELECT 1 FROM favorites WHERE repo_id = ?1 LIMIT 1",
            params![project_id],
            |_| Ok(true),
        )
        .optional()?
        .unwrap_or(false);

    if is_favorite {
        transaction.execute("DELETE FROM favorites WHERE repo_id = ?1", params![project_id])?;
    } else {
        transaction.execute(
            "INSERT INTO favorites (repo_id, created_at) VALUES (?1, datetime('now'))",
            params![project_id],
        )?;
    }

    transaction.execute(
        "UPDATE projects SET is_favorite = ?2 WHERE id = ?1",
        params![project_id, bool_to_i64(!is_favorite)],
    )?;

    transaction.commit()?;

    Ok(FavoriteToggleResponse {
        project_id,
        is_favorite: !is_favorite,
    })
}

pub fn upsert_projects(db_path: &Path, projects: &[SyncedProject]) -> Result<(usize, usize)> {
    let mut connection = open_connection(db_path)?;
    let transaction = connection.transaction()?;
    let mut inserted = 0;
    let mut updated = 0;

    for project in projects {
        let existing_id = transaction
            .query_row(
                "SELECT id FROM projects WHERE repo_name = ?1 LIMIT 1",
                params![project.repo_name],
                |row| row.get::<_, i64>(0),
            )
            .optional()?;

        let repo_id = if let Some(repo_id) = existing_id {
            updated += 1;
            transaction.execute(
                r#"
                UPDATE projects
                SET description = ?2,
                    github_url = ?3,
                    homepage_url = ?4,
                    demo_url = ?5,
                    language = ?6,
                    stars = ?7,
                    forks = ?8,
                    open_issues = ?9,
                    topics = ?10,
                    category = ?11,
                    score = ?12,
                    license = ?13,
                    updated_at = ?14,
                    synced_at = datetime('now')
                WHERE id = ?1
                "#,
                params![
                    repo_id,
                    project.description,
                    project.github_url,
                    project.homepage_url,
                    project.demo_url,
                    project.language,
                    project.stars,
                    project.forks,
                    project.open_issues,
                    serde_json::to_string(&project.topics)?,
                    project.category,
                    project.score,
                    project.license,
                    project.updated_at,
                ],
            )?;
            repo_id
        } else {
            inserted += 1;
            transaction.execute(
                r#"
                INSERT INTO projects (
                    repo_name, owner, name, description, github_url, homepage_url, demo_url,
                    language, stars, forks, open_issues, topics, category, score, license,
                    pushed_at, created_at, updated_at, synced_at, is_favorite
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, datetime('now'), ?16, datetime('now'), 0)
                "#,
                params![
                    project.repo_name,
                    project.owner,
                    project.repo,
                    project.description,
                    project.github_url,
                    project.homepage_url,
                    project.demo_url,
                    project.language,
                    project.stars,
                    project.forks,
                    project.open_issues,
                    serde_json::to_string(&project.topics)?,
                    project.category,
                    project.score,
                    project.license,
                    project.updated_at,
                ],
            )?;
            transaction.last_insert_rowid()
        };

        transaction.execute(
            r#"
            INSERT INTO summaries (
                repo_id, summary, highlights, use_cases, frontend_value,
                learning_cost, frontend_relevance, generated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'))
            ON CONFLICT(repo_id) DO UPDATE SET
                summary = excluded.summary,
                highlights = excluded.highlights,
                use_cases = excluded.use_cases,
                frontend_value = excluded.frontend_value,
                learning_cost = excluded.learning_cost,
                frontend_relevance = excluded.frontend_relevance,
                generated_at = datetime('now')
            "#,
            params![
                repo_id,
                project.summary,
                serde_json::to_string(&project.highlights)?,
                serde_json::to_string(&project.use_cases)?,
                project.frontend_value,
                project.learning_cost,
                project.frontend_relevance,
            ],
        )?;
    }

    transaction.commit()?;
    Ok((inserted, updated))
}

fn open_connection(db_path: &Path) -> Result<Connection> {
    Connection::open(db_path).with_context(|| format!("failed to open database at {}", db_path.display()))
}

fn create_schema(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_name TEXT NOT NULL UNIQUE,
            owner TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            github_url TEXT NOT NULL,
            homepage_url TEXT,
            demo_url TEXT,
            language TEXT,
            stars INTEGER NOT NULL DEFAULT 0,
            forks INTEGER NOT NULL DEFAULT 0,
            open_issues INTEGER NOT NULL DEFAULT 0,
            topics TEXT NOT NULL DEFAULT '[]',
            category TEXT NOT NULL DEFAULT 'General',
            score REAL NOT NULL DEFAULT 0,
            license TEXT,
            pushed_at TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            synced_at TEXT,
            is_favorite INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS summaries (
            repo_id INTEGER PRIMARY KEY,
            summary TEXT NOT NULL,
            highlights TEXT NOT NULL DEFAULT '[]',
            use_cases TEXT NOT NULL DEFAULT '[]',
            frontend_value TEXT NOT NULL DEFAULT '',
            learning_cost TEXT NOT NULL DEFAULT '中',
            frontend_relevance INTEGER NOT NULL DEFAULT 0,
            generated_at TEXT NOT NULL,
            FOREIGN KEY(repo_id) REFERENCES projects(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS favorites (
            repo_id INTEGER PRIMARY KEY,
            created_at TEXT NOT NULL,
            FOREIGN KEY(repo_id) REFERENCES projects(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_projects_repo_name ON projects(repo_name);
        CREATE INDEX IF NOT EXISTS idx_projects_language ON projects(language);
        CREATE INDEX IF NOT EXISTS idx_projects_category ON projects(category);
        CREATE INDEX IF NOT EXISTS idx_projects_updated_at ON projects(updated_at DESC);
        CREATE INDEX IF NOT EXISTS idx_projects_is_favorite ON projects(is_favorite);
        CREATE INDEX IF NOT EXISTS idx_summaries_frontend_relevance ON summaries(frontend_relevance DESC);
        "#,
    )?;

    Ok(())
}

fn seed_sample_projects(connection: &Connection) -> Result<()> {
    let existing_count: i64 = connection.query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))?;

    if existing_count > 0 {
        return Ok(());
    }

    let projects: [SeedProject<'_>; 3] = [
        (
            "langgenius",
            "dify",
            "langgenius/dify",
            "开源 LLM 应用开发平台，提供工作流、知识库和可视化运营后台。",
            "https://github.com/langgenius/dify",
            "TypeScript",
            92000,
            13000,
            982,
            &["ai", "workflow", "agent", "llm"],
            "LLM 应用",
            92,
            "对前端团队而言，Dify 的控制台信息架构、节点编排和状态反馈都很有参考价值。",
            &["工作流编排可视化", "知识库与应用发布一体化", "运营后台成熟"],
            &["快速搭建团队内部 AI 工具", "研究 AI SaaS 控制台设计"],
            "前端可以直接借鉴其表单流、编排视图和状态反馈设计。",
            "中",
            Some("https://dify.ai"),
            Some("https://cloud.dify.ai"),
            "Apache-2.0",
        ),
        (
            "microsoft",
            "autogen",
            "microsoft/autogen",
            "多智能体协作框架，适合构建复杂的自动化任务流和对话式系统。",
            "https://github.com/microsoft/autogen",
            "Python",
            41000,
            6000,
            514,
            &["agent", "multi-agent", "automation"],
            "AI Agent",
            81,
            "多智能体消息流设计成熟，适合作为前端可视化调试台和编排界面的参考样本。",
            &["多角色协作机制", "适合长任务链编排", "生态热度高"],
            &["构建协作型 Agent 系统", "研究可视化调度界面"],
            "前端可以围绕任务追踪、消息时间线和可视化调试做体验设计。",
            "中",
            Some("https://microsoft.github.io/autogen/"),
            None,
            "MIT",
        ),
        (
            "open-webui",
            "open-webui",
            "open-webui/open-webui",
            "围绕本地和云端大模型的统一聊天界面，强调安装便利性和多模型切换体验。",
            "https://github.com/open-webui/open-webui",
            "TypeScript",
            68000,
            8000,
            267,
            &["chat-ui", "ollama", "rag", "frontend"],
            "AI UI / Frontend",
            95,
            "聊天界面、模型切换、消息流体验成熟，是前端侧最直接的竞品参考。",
            &["多模型切换体验完整", "聊天界面成熟", "适配本地模型部署"],
            &["搭建企业内部聊天助手", "研究流式输出与消息布局"],
            "前端团队可直接对照其消息结构、设置面板和对话状态设计。",
            "低",
            Some("https://openwebui.com"),
            Some("https://demo.openwebui.com"),
            "BSD-3-Clause",
        ),
    ];

    let transaction = connection.unchecked_transaction()?;

    for (
        owner,
        name,
        repo_name,
        description,
        github_url,
        language,
        stars,
        forks,
        open_issues,
        topics,
        category,
        score,
        summary,
        highlights,
        use_cases,
        frontend_value,
        learning_cost,
        homepage_url,
        demo_url,
        license,
    ) in projects
    {
        transaction.execute(
            r#"
            INSERT INTO projects (
                repo_name, owner, name, description, github_url, homepage_url, demo_url,
                language, stars, forks, open_issues, topics, category, score, license,
                pushed_at, created_at, updated_at, synced_at, is_favorite
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, datetime('now', '-1 day'), datetime('now', '-30 day'), datetime('now'), datetime('now'), 0)
            "#,
            params![
                repo_name,
                owner,
                name,
                description,
                github_url,
                homepage_url,
                demo_url,
                language,
                stars,
                forks,
                open_issues,
                serialize_json(topics),
                category,
                score,
                license,
            ],
        )?;

        let repo_id = transaction.last_insert_rowid();
        let frontend_relevance = if score >= 90 { 3 } else if score >= 80 { 2 } else { 1 };

        transaction.execute(
            r#"
            INSERT INTO summaries (
                repo_id, summary, highlights, use_cases, frontend_value,
                learning_cost, frontend_relevance, generated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'))
            "#,
            params![
                repo_id,
                summary,
                serialize_json(highlights),
                serialize_json(use_cases),
                frontend_value,
                learning_cost,
                frontend_relevance,
            ],
        )?;
    }

    transaction.commit()?;

    Ok(())
}

fn map_project_summary(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProjectSummary> {
    let owner: String = row.get(1)?;
    let repo: String = row.get(2)?;

    Ok(ProjectSummary {
        id: row.get(0)?,
        owner,
        repo,
        repo_name: row.get(3)?,
        description: row.get(4)?,
        language: row.get(5)?,
        stars: row.get(6)?,
        forks: row.get(7)?,
        updated_at: row.get(8)?,
        category: row.get(9)?,
        frontend_relevance: row.get(10)?,
        summary: row.get(11)?,
        topics: parse_json_vec(&row.get::<_, String>(12)?),
        demo_url: row.get(13)?,
        is_favorite: row.get::<_, i64>(14)? == 1,
        favorite_created_at: row.get(15)?,
    })
}

fn serialize_json(items: &[&str]) -> String {
    serde_json::to_string(items).unwrap_or_else(|_| "[]".to_string())
}

fn parse_json_vec(value: &str) -> Vec<String> {
    serde_json::from_str(value).unwrap_or_default()
}

fn bool_to_i64(value: bool) -> i64 {
    if value { 1 } else { 0 }
}