# AI 开源项目情报站 架构设计（MVP版）

## 1. 整体架构
采用 **Tauri + React + SQLite** 的单机轻量桌面端架构。MVP 版本不依赖外部服务器后端，由 Tauri 客户端自身通过 HTTP 抓取数据并持久化到本地。

### 核心分层：
- **UI 层 (React/前端)**：负责页面展示、列表渲染、路由交互。
- **IPC 通信层 (Tauri Command)**：充当桥梁，隔离业务逻辑和视图。
- **数据与逻辑层 (Rust/后端)**：执行 GitHub 分页拉取、大模型 API 调用（生成摘要）以及本地 SQLite 数据增删查改。

---

## 2. 技术栈选型
### 2.1 客户端前端（UI）
- **框架**: Native-like React (Vite + TS)
- **路由**: `react-router-dom`
- **状态管理**: 
  - 全局/本地 UI 状态：`zustand`
  - 异步数据流/缓存：`@tanstack/react-query` (极其适合处理轮询、局部刷新)
- **样式**: `Tailwind CSS` + 基础组件库 (如 `shadcn/ui` 或单纯的自定义 UI)
- **图标**: `lucide-react`

### 2.2 客户端核心（Rust 底层）
- **运行时环境**: Tauri (1.x 或 2.x 均可)
- **网络请求**: `reqwest` (调用 GitHub API 和 OpenAI 兼容接口)
- **数据库**: `rusqlite` + `r2d2` 形式封装，或直接使用 `sqlx` 组件，存储到系统内置的数据目录（App Data Directory）
- **序列化**: `serde` / `serde_json`
- **异步运行时**: `tokio`

---

## 3. 核心流转架构（Data Flow）

### 场景 1：定时/手动更新数据
`Tauri 后台线程` -> 定期调用 `GitHub Search API` (根据关键字: ai, llm 等) 
-> `清洗并合并旧数据` 
-> 请求 `大模型 API` (获取未生成的 README 的 AI 摘要)
-> 写入/更新 `SQLite (repositories / summaries 表)`
-> `Tauri Event` 推送通知到前端：“数据已更新，请刷新列表” 
-> 前端 `React Query` 失效缓存，重新拉取本地数据。

### 场景 2：前端展现数据
`React/前端` -> 发送 IPC 请求 `invoke('get_projects')` 
-> `Tauri Rust` 查询本地 SQLite，根据条件过滤 (排序、分类、搜索)
-> 返回 `JSON Array` 
-> `React` 渲染虚拟列表 (如果数据过多)。

---

## 4. 数据表结构设计 (SQLite 本地)

### `projects` 表
- `id` (INTEGER PRIMARY KEY)
- `repo_name` (TEXT) : 如 `openai/whisper`
- `description` (TEXT) : 原始描述
- `github_url` (TEXT)
- `language` (TEXT)
- `stars` (INTEGER)
- `forks` (INTEGER)
- `pushed_at` (TEXT) : 最近更新日期
- `topics` (TEXT) : JSON格式保存标签
- `ai_summary` (TEXT) : 核心一句话摘要
- `frontend_relevance` (INTEGER) : 前端相关评分 (1-3星)
- `is_favorite` (BOOLEAN): 默认 false

---

## 5. 目录结构规划
```text
/src                # 前端代码
  /components       # 业务组件 (Layout, ProjectCard等)
  /hooks            # 自定义 hooks (useProjects, useSyncData)
  /pages            # 页面视图 (Home, Favorites, Settings)
  /lib              # API / Tauri invoke 封装
  /store            # zustand 状态

/src-tauri
  /src
    main.rs         # Tauri 入口
    commands.rs     # IPC 指令 (get_projects, toggle_favorite, sync_github)
    db.rs           # SQLite 数据库表初始化、CRUD封装
    api.rs          # 封装 reqwest 调用 GitHub/大模型
    models.rs       # serde struct 映射
```