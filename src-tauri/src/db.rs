use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use tauri::{AppHandle, Manager};

use crate::models::{
    AiProjectSectionsResponse, FavoriteToggleResponse, ProjectDetail, ProjectFilters,
    ProjectListResponse, ProjectSummary, SyncedProject,
};

pub struct HealthSnapshot {
    pub project_count: usize,
    pub favorite_count: usize,
    pub last_synced_at: Option<String>,
}

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
    bool,
    &'a str,
    i64,
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

pub fn list_projects(db_path: &Path, filters: ProjectFilters) -> Result<ProjectListResponse> {
    let connection = open_connection(db_path)?;
    let limit = filters.limit.unwrap_or(20).max(1);
    let page = filters.page.unwrap_or(1).max(1);
    let page_size = i64::from(limit);
    let offset = i64::from(page.saturating_sub(1)) * page_size;
    let search_like = filters.search.as_deref().map(normalize_search_like);
    let topic_like = filters.topic.as_deref().map(normalize_search_like);

    let sort_by = match filters.sort_by.as_deref() {
        Some("stars") => "p.stars DESC, p.updated_at DESC",
        Some("updatedAt") => "p.updated_at DESC, p.stars DESC",
        Some("impactRank") => "p.impact_rank DESC, p.score DESC, p.stars DESC",
        Some("frontendRelevance") => "COALESCE(s.frontend_relevance, 0) DESC, p.stars DESC",
        Some("favoritedAt") => "COALESCE(f.created_at, '') DESC, p.updated_at DESC",
        _ => "p.score DESC, p.stars DESC, p.updated_at DESC",
    };

    let where_clause = r#"
        WHERE (?1 IS NULL OR p.language = ?1)
          AND (?2 IS NULL OR p.category = ?2)
          AND (?3 = 0 OR COALESCE(s.frontend_relevance, 0) >= 2)
          AND (?4 = 0 OR (p.demo_url IS NOT NULL AND p.demo_url <> ''))
          AND (?5 = 0 OR f.repo_id IS NOT NULL)
                    AND (
                        ?6 IS NULL
                        OR lower(p.repo_name) LIKE ?6
                        OR lower(p.description) LIKE ?6
                        OR lower(COALESCE(s.summary, p.description)) LIKE ?6
                        OR lower(p.topics) LIKE ?6
                    )
                    AND (?7 IS NULL OR lower(p.topics) LIKE ?7)
          AND (?8 = 0 OR p.is_ai = 1)
          AND (?9 IS NULL OR p.era = ?9)
    "#;

    let count_query = format!(
        r#"
        SELECT COUNT(*)
        FROM projects p
        LEFT JOIN summaries s ON s.repo_id = p.id
        LEFT JOIN favorites f ON f.repo_id = p.id
        {}
        "#,
        where_clause
    );

    let total: i64 = connection.query_row(
        &count_query,
        params![
            filters.language.clone(),
            filters.category.clone(),
            bool_to_i64(filters.frontend_only.unwrap_or(false)),
            bool_to_i64(filters.has_demo.unwrap_or(false)),
            bool_to_i64(filters.favorites_only.unwrap_or(false)),
            search_like.clone(),
            topic_like.clone(),
            bool_to_i64(filters.ai_only.unwrap_or(false)),
            filters.era.clone(),
        ],
        |row| row.get(0),
    )?;

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
            p.score,
            COALESCE(s.frontend_relevance, 0) AS frontend_relevance,
            COALESCE(s.summary, p.description) AS summary,
            p.description_long,
            p.topics,
            p.demo_url,
            p.is_ai,
            p.era,
            p.impact_rank,
            CASE WHEN f.repo_id IS NULL THEN 0 ELSE 1 END AS is_favorite,
            f.created_at AS favorite_created_at
        FROM projects p
        LEFT JOIN summaries s ON s.repo_id = p.id
        LEFT JOIN favorites f ON f.repo_id = p.id
                {}
        ORDER BY {}
            LIMIT ?10 OFFSET ?11
        "#,
        where_clause, sort_by
    );

    let mut statement = connection.prepare(&query)?;
    let rows = statement.query_map(
        params![
            filters.language,
            filters.category,
            bool_to_i64(filters.frontend_only.unwrap_or(false)),
            bool_to_i64(filters.has_demo.unwrap_or(false)),
            bool_to_i64(filters.favorites_only.unwrap_or(false)),
            search_like,
            topic_like,
            bool_to_i64(filters.ai_only.unwrap_or(false)),
            filters.era,
            page_size,
            offset,
        ],
        map_project_summary,
    )?;

    let items = rows.collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(ProjectListResponse {
        has_more: offset + (items.len() as i64) < total,
        items,
        page,
        page_size: limit,
        total: total.max(0) as usize,
    })
}

pub fn list_ai_project_sections(db_path: &Path, limit: u32) -> Result<AiProjectSectionsResponse> {
    let page_limit = limit.max(1);
    let classic = list_projects(
        db_path,
        ProjectFilters {
            ai_only: Some(true),
            era: Some("classic".to_string()),
            sort_by: Some("impactRank".to_string()),
            page: Some(1),
            limit: Some(page_limit),
            ..ProjectFilters::default()
        },
    )?;
    let latest = list_projects(
        db_path,
        ProjectFilters {
            ai_only: Some(true),
            era: Some("latest".to_string()),
            sort_by: Some("updatedAt".to_string()),
            page: Some(1),
            limit: Some(page_limit),
            ..ProjectFilters::default()
        },
    )?;

    let connection = open_connection(db_path)?;
    let invalid_era_count: i64 = connection.query_row(
        "SELECT COUNT(*) FROM projects WHERE is_ai = 1 AND era NOT IN ('classic', 'latest')",
        [],
        |row| row.get(0),
    )?;

    Ok(AiProjectSectionsResponse {
        classic: classic.items,
        latest: latest.items,
        classic_total: classic.total,
        latest_total: latest.total,
        invalid_era_count: invalid_era_count.max(0) as usize,
    })
}

pub fn list_favorites(db_path: &Path) -> Result<Vec<ProjectSummary>> {
    Ok(list_projects(
        db_path,
        ProjectFilters {
            favorites_only: Some(true),
            ai_only: Some(true),
            sort_by: Some("favoritedAt".to_string()),
            limit: Some(50),
            ..ProjectFilters::default()
        },
    )?
    .items)
}

pub fn get_health_snapshot(db_path: &Path) -> Result<HealthSnapshot> {
    let connection = open_connection(db_path)?;
    let project_count: i64 =
        connection.query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))?;
    let favorite_count: i64 =
        connection.query_row("SELECT COUNT(*) FROM favorites", [], |row| row.get(0))?;
    let last_synced_at =
        connection.query_row("SELECT MAX(synced_at) FROM projects", [], |row| {
            row.get::<_, Option<String>>(0)
        })?;

    Ok(HealthSnapshot {
        project_count: project_count.max(0) as usize,
        favorite_count: favorite_count.max(0) as usize,
        last_synced_at,
    })
}

pub fn get_project_detail_by_repo(
    db_path: &Path,
    owner: &str,
    repo: &str,
) -> Result<ProjectDetail> {
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
            p.score,
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
                score: row.get::<_, f64>(14)? as i64,
                frontend_relevance: row.get(15)?,
                summary: row.get(16)?,
                highlights: parse_json_vec(&row.get::<_, String>(17)?),
                use_cases: parse_json_vec(&row.get::<_, String>(18)?),
                frontend_value: row.get(19)?,
                learning_cost: row.get(20)?,
                topics: parse_json_vec(&row.get::<_, String>(21)?),
                license: row.get(22)?,
                is_favorite: row.get::<_, i64>(23)? == 1,
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
        transaction.execute(
            "DELETE FROM favorites WHERE repo_id = ?1",
            params![project_id],
        )?;
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
        validate_synced_project(project)?;
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
                    synced_at = datetime('now'),
                    is_ai = ?15,
                    era = ?16,
                    impact_rank = ?17,
                    description_long = ?18
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
                    bool_to_i64(project.is_ai),
                    project.era,
                    project.impact_rank,
                    project.description_long,
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
                    pushed_at, created_at, updated_at, synced_at, is_favorite,
                    is_ai, era, impact_rank, description_long
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, datetime('now'), ?16, datetime('now'), 0, ?17, ?18, ?19, ?20)
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
                    bool_to_i64(project.is_ai),
                    project.era,
                    project.impact_rank,
                    project.description_long,
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
    Connection::open(db_path)
        .with_context(|| format!("failed to open database at {}", db_path.display()))
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
            is_favorite INTEGER NOT NULL DEFAULT 0,
            is_ai INTEGER NOT NULL DEFAULT 1,
            era TEXT NOT NULL DEFAULT 'latest',
            impact_rank INTEGER NOT NULL DEFAULT 0,
            description_long TEXT NOT NULL DEFAULT ''
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
        CREATE INDEX IF NOT EXISTS idx_projects_is_ai ON projects(is_ai);
        CREATE INDEX IF NOT EXISTS idx_projects_era ON projects(era);
        CREATE INDEX IF NOT EXISTS idx_projects_impact_rank ON projects(impact_rank DESC);
        CREATE INDEX IF NOT EXISTS idx_summaries_frontend_relevance ON summaries(frontend_relevance DESC);
        "#,
    )?;

    ensure_project_schema_extensions(connection)?;

    Ok(())
}

fn seed_sample_projects(connection: &Connection) -> Result<()> {
    let existing_count: i64 =
        connection.query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))?;

    if existing_count > 0 {
        return Ok(());
    }

    let projects: [SeedProject<'_>; 8] = [
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
            true,
            "classic",
            99,
            "Dify 是一个面向 AI 应用构建与运营的全栈平台，支持工作流编排、知识库接入、应用发布和监控。对于产品与前端团队，它不仅是可直接部署的 AI 中台，也能作为复杂配置面板、可视化流程编辑器和状态反馈体系的高质量参考。",
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
            true,
            "classic",
            95,
            "AutoGen 聚焦多智能体协作与长任务链自动化，适合构建需要多角色对话、任务分解和工具调用的 AI 系统。它在消息编排、角色协同和流程控制方面提供了成熟范式，便于前端设计可视化调度台和任务追踪界面。",
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
            true,
            "classic",
            93,
            "Open WebUI 提供本地与云端大模型的统一交互界面，具备多模型切换、消息流展示、配置管理等核心能力。其信息架构与交互细节非常适合作为 AI 聊天产品前端的基准案例，尤其适合研究流式输出与会话状态管理。",
        ),
        (
            "langchain-ai",
            "langgraph",
            "langchain-ai/langgraph",
            "用于构建可控 Agent 工作流与图式编排的框架，强调状态持久化与人机协同。",
            "https://github.com/langchain-ai/langgraph",
            "Python",
            35000,
            4200,
            240,
            &["agent", "workflow", "graph", "llm"],
            "Agent Framework",
            90,
            "LangGraph 将 Agent 任务建模为图结构，便于实现可恢复、可追踪的复杂工作流。",
            &["图式工作流建模", "支持断点与恢复", "适合长链路任务编排"],
            &["构建企业级 Agent 流程", "实现可观测的多步骤任务"],
            "前端可借鉴其节点状态和执行路径可视化。",
            "中",
            Some("https://www.langchain.com/langgraph"),
            None,
            "MIT",
            true,
            "latest",
            88,
            "LangGraph 通过图结构组织 Agent 行为与上下文状态，特别适合需要可观察、可恢复、可插入人工审核节点的复杂自动化流程。它能帮助团队把原本不可控的对话流升级为可治理、可追踪的生产级任务系统。",
        ),
        (
            "anthropics",
            "claude-code",
            "anthropics/claude-code",
            "AI 编码代理工具链，面向终端与仓库级协作的开发自动化实践。",
            "https://github.com/anthropics/claude-code",
            "TypeScript",
            22000,
            1500,
            110,
            &["ai", "coding-agent", "developer-tools"],
            "Developer Tooling",
            86,
            "以仓库上下文为核心的 AI 编码协作工具，强调工程化落地。",
            &["仓库级上下文理解", "可编排执行开发任务", "贴近真实工程流程"],
            &["提升研发效率", "构建团队级 AI 开发流程"],
            "可用于设计面向开发者的任务流 UI 与反馈机制。",
            "中",
            None,
            None,
            "MIT",
            true,
            "latest",
            84,
            "Claude Code 展示了 AI 在真实软件工程链路中的落地方式，从理解代码库到执行任务再到反馈结果形成闭环。对于研发团队，它提供了构建 AI 原生开发流程的样板，并能帮助产品侧定义更清晰的人机协作边界。",
        ),
        (
            "crewAIInc",
            "crewAI",
            "crewAIInc/crewAI",
            "多角色 Agent 协作框架，强调角色职责分离与任务接力。",
            "https://github.com/crewAIInc/crewAI",
            "Python",
            30000,
            3900,
            200,
            &["agent", "multi-agent", "automation"],
            "Agent Framework",
            87,
            "通过角色分工与流程编排构建多智能体协作体系。",
            &["角色化 Agent 组织", "任务接力模型明确", "上手路径清晰"],
            &["客服自动化", "复杂任务分治"],
            "前端可围绕角色看板与流程状态设计协作界面。",
            "中",
            Some("https://www.crewai.com"),
            None,
            "MIT",
            true,
            "latest",
            82,
            "CrewAI 通过明确的角色职责和任务接力机制，让多 Agent 协作更容易建模与管理。它适合需要流程可视化和角色协同的业务场景，也便于前端构建可解释的任务进度与责任归属界面。",
        ),
        (
            "run-llama",
            "llama_index",
            "run-llama/llama_index",
            "面向 RAG 与知识增强应用的数据连接与检索框架。",
            "https://github.com/run-llama/llama_index",
            "Python",
            43000,
            6100,
            310,
            &["rag", "retrieval", "llm"],
            "RAG / Search",
            89,
            "连接异构数据源并构建可扩展的检索增强系统。",
            &["数据源适配广", "检索策略灵活", "社区生态活跃"],
            &["企业知识问答", "内部文档搜索增强"],
            "前端可借鉴其检索链路反馈与答案溯源展示。",
            "中",
            Some("https://www.llamaindex.ai"),
            None,
            "MIT",
            true,
            "classic",
            90,
            "LlamaIndex 是 RAG 体系中的关键基础设施，擅长把分散的数据源组织成可检索、可组合、可追踪的知识上下文。它特别适合需要高可信回答与数据溯源能力的企业场景，能显著提升问答系统的可用性。",
        ),
        (
            "cloudflare",
            "workers-ai",
            "cloudflare/workers-ai",
            "边缘环境部署与调用 AI 模型的平台能力集合。",
            "https://github.com/cloudflare/workers-ai",
            "TypeScript",
            12000,
            900,
            80,
            &["edge", "ai", "inference"],
            "LLM Application",
            80,
            "把 AI 推理能力下沉到边缘节点，提升响应与部署灵活性。",
            &["边缘推理低延迟", "与 Workers 生态衔接", "部署路径清晰"],
            &["全球化低延迟 AI 应用", "边缘智能 API"],
            "可为前端提供更近端的推理服务与实时交互体验。",
            "中",
            Some("https://developers.cloudflare.com/workers-ai/"),
            None,
            "Apache-2.0",
            true,
            "latest",
            79,
            "Workers AI 让模型推理能力能够更接近用户侧运行，适合对时延和全球覆盖有要求的 AI 产品。通过与边缘计算平台结合，团队可以更快落地实时交互型体验，并降低中心化推理瓶颈带来的风险。",
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
        is_ai,
        era,
        impact_rank,
        description_long,
    ) in projects
    {
        transaction.execute(
            r#"
            INSERT INTO projects (
                repo_name, owner, name, description, github_url, homepage_url, demo_url,
                language, stars, forks, open_issues, topics, category, score, license,
                pushed_at, created_at, updated_at, synced_at, is_favorite,
                is_ai, era, impact_rank, description_long
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, datetime('now', '-1 day'), datetime('now', '-30 day'), datetime('now'), datetime('now'), 0, ?16, ?17, ?18, ?19)
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
                bool_to_i64(is_ai),
                era,
                impact_rank,
                description_long,
            ],
        )?;

        let repo_id = transaction.last_insert_rowid();
        let frontend_relevance = if score >= 90 {
            3
        } else if score >= 80 {
            2
        } else {
            1
        };

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
        score: row.get::<_, f64>(10)? as i64,
        frontend_relevance: row.get(11)?,
        summary: row.get(12)?,
        description_long: row.get(13)?,
        topics: parse_json_vec(&row.get::<_, String>(14)?),
        demo_url: row.get(15)?,
        is_ai: row.get::<_, i64>(16)? == 1,
        era: row.get(17)?,
        impact_rank: row.get(18)?,
        is_favorite: row.get::<_, i64>(19)? == 1,
        favorite_created_at: row.get(20)?,
    })
}

fn ensure_project_schema_extensions(connection: &Connection) -> Result<()> {
    ensure_column(
        connection,
        "is_ai",
        "ALTER TABLE projects ADD COLUMN is_ai INTEGER NOT NULL DEFAULT 1",
    )?;
    ensure_column(
        connection,
        "era",
        "ALTER TABLE projects ADD COLUMN era TEXT NOT NULL DEFAULT 'latest'",
    )?;
    ensure_column(
        connection,
        "impact_rank",
        "ALTER TABLE projects ADD COLUMN impact_rank INTEGER NOT NULL DEFAULT 0",
    )?;
    ensure_column(
        connection,
        "description_long",
        "ALTER TABLE projects ADD COLUMN description_long TEXT NOT NULL DEFAULT ''",
    )?;

    connection.execute(
        "UPDATE projects SET description_long = COALESCE(NULLIF(description_long, ''), description)",
        [],
    )?;

    connection.execute(
        "UPDATE projects SET era = CASE WHEN era IN ('classic', 'latest') THEN era ELSE 'latest' END",
        [],
    )?;

    Ok(())
}

fn ensure_column(connection: &Connection, column_name: &str, statement: &str) -> Result<()> {
    let mut pragma = connection.prepare("PRAGMA table_info(projects)")?;
    let rows = pragma.query_map([], |row| row.get::<_, String>(1))?;
    let has_column = rows
        .collect::<rusqlite::Result<Vec<_>>>()?
        .into_iter()
        .any(|name| name == column_name);

    if !has_column {
        connection.execute(statement, [])?;
    }

    Ok(())
}

fn validate_synced_project(project: &SyncedProject) -> Result<()> {
    if project.is_ai && project.era != "classic" && project.era != "latest" {
        return Err(anyhow!(
            "invalid era for AI project {}: {}",
            project.repo_name,
            project.era
        ));
    }

    if project.description_long.trim().is_empty() {
        return Err(anyhow!(
            "description_long is required for project {}",
            project.repo_name
        ));
    }

    Ok(())
}

fn normalize_search_like(value: &str) -> String {
    format!("%{}%", value.trim().to_lowercase())
}

fn serialize_json(items: &[&str]) -> String {
    serde_json::to_string(items).unwrap_or_else(|_| "[]".to_string())
}

fn parse_json_vec(value: &str) -> Vec<String> {
    serde_json::from_str(value).unwrap_or_default()
}

fn bool_to_i64(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ProjectFilters, SyncedProject};
    use std::{
        env,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn test_db_path() -> PathBuf {
        let file_name = format!(
            "ai-good-project-test-{}.db",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time before unix epoch")
                .as_nanos()
        );

        env::temp_dir().join(file_name)
    }

    fn sample_project(owner: &str, repo: &str, score: i64) -> SyncedProject {
        SyncedProject {
            owner: owner.to_string(),
            repo: repo.to_string(),
            repo_name: format!("{owner}/{repo}"),
            description: format!("{repo} description"),
            github_url: format!("https://github.com/{owner}/{repo}"),
            homepage_url: Some(format!("https://{repo}.example.com")),
            demo_url: Some(format!("https://demo.example.com/{repo}")),
            language: Some("TypeScript".to_string()),
            stars: score * 100,
            forks: score * 10,
            open_issues: 3,
            topics: vec!["ai".to_string(), "frontend".to_string()],
            category: "AI UI / Frontend".to_string(),
            score,
            license: Some("MIT".to_string()),
            updated_at: "2026-03-19T10:00:00Z".to_string(),
            summary: format!("summary for {repo}"),
            description_long: format!("{repo} long description with AI context"),
            highlights: vec!["highlight-a".to_string(), "highlight-b".to_string()],
            use_cases: vec!["use-case-a".to_string()],
            frontend_value: "high".to_string(),
            learning_cost: "低".to_string(),
            frontend_relevance: 3,
            is_ai: true,
            era: if score >= 85 {
                "classic".to_string()
            } else {
                "latest".to_string()
            },
            impact_rank: score,
        }
    }

    fn create_test_database() -> PathBuf {
        let db_path = test_db_path();
        let connection = open_connection(&db_path).expect("open test database");
        create_schema(&connection).expect("create schema");
        upsert_projects(
            &db_path,
            &[
                sample_project("owner", "repo-one", 95),
                sample_project("owner", "repo-two", 85),
                sample_project("owner", "repo-three", 75),
            ],
        )
        .expect("seed test projects");
        db_path
    }

    #[test]
    fn list_projects_supports_pagination() {
        let db_path = create_test_database();

        let page_one = list_projects(
            &db_path,
            ProjectFilters {
                page: Some(1),
                limit: Some(2),
                ..ProjectFilters::default()
            },
        )
        .expect("page one query should succeed");

        assert_eq!(page_one.total, 3);
        assert_eq!(page_one.items.len(), 2);
        assert_eq!(page_one.page, 1);
        assert!(page_one.has_more);

        let page_two = list_projects(
            &db_path,
            ProjectFilters {
                page: Some(2),
                limit: Some(2),
                ..ProjectFilters::default()
            },
        )
        .expect("page two query should succeed");

        assert_eq!(page_two.items.len(), 1);
        assert!(!page_two.has_more);

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn list_projects_supports_keyword_and_topic_search() {
        let db_path = create_test_database();

        let result = list_projects(
            &db_path,
            ProjectFilters {
                search: Some("repo-two".to_string()),
                ..ProjectFilters::default()
            },
        )
        .expect("keyword search should succeed");
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].repo, "repo-two");

        let result = list_projects(
            &db_path,
            ProjectFilters {
                topic: Some("frontend".to_string()),
                ..ProjectFilters::default()
            },
        )
        .expect("topic search should succeed");
        assert_eq!(result.total, 3);

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn toggle_favorite_persists_and_is_queryable() {
        let db_path = create_test_database();
        let first_page = list_projects(&db_path, ProjectFilters::default()).expect("list projects");
        let project_id = first_page.items.first().expect("first project").id;

        let response = toggle_favorite(&db_path, project_id).expect("toggle favorite on");
        assert!(response.is_favorite);

        let favorites = list_favorites(&db_path).expect("list favorites");
        assert_eq!(favorites.len(), 1);
        assert_eq!(favorites[0].id, project_id);
        assert!(favorites[0].is_favorite);

        let response = toggle_favorite(&db_path, project_id).expect("toggle favorite off");
        assert!(!response.is_favorite);

        let favorites = list_favorites(&db_path).expect("list favorites after removal");
        assert!(favorites.is_empty());

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn list_projects_applies_ai_filter_before_limit() {
        let db_path = create_test_database();
        let mut non_ai = sample_project("owner", "non-ai-top", 999);
        non_ai.is_ai = false;
        non_ai.era = "latest".to_string();
        non_ai.category = "General Tooling".to_string();
        upsert_projects(&db_path, &[non_ai]).expect("insert non-ai project");

        let result = list_projects(
            &db_path,
            ProjectFilters {
                ai_only: Some(true),
                sort_by: Some("score".to_string()),
                page: Some(1),
                limit: Some(2),
                ..ProjectFilters::default()
            },
        )
        .expect("list ai-only projects");

        assert_eq!(result.items.len(), 2);
        assert_eq!(result.total, 3);
        assert!(result.items.iter().all(|item| item.is_ai));

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn list_ai_project_sections_groups_by_era_and_reports_invalid_rows() {
        let db_path = create_test_database();
        let connection = open_connection(&db_path).expect("open connection");
        connection
            .execute(
                "UPDATE projects SET era = 'unknown' WHERE name = 'repo-one'",
                [],
            )
            .expect("inject invalid era row");

        let sections = list_ai_project_sections(&db_path, 12).expect("list sections");

        assert_eq!(sections.invalid_era_count, 1);
        assert!(sections.classic.iter().all(|item| item.era == "classic"));
        assert!(sections.latest.iter().all(|item| item.era == "latest"));
        assert!(sections.classic.iter().all(|item| item.is_ai));
        assert!(sections.latest.iter().all(|item| item.is_ai));

        let _ = fs::remove_file(db_path);
    }
}
