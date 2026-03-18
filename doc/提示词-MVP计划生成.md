# AI 开源项目情报站 MVP 计划生成提示词

你可以将以下这段提示词，复制发送给具备 Planner / Agent 模式的 AI 助手，让它自动为你生成接下来的拆解计划（Plan）：

---

```text
# 角色定义
你是一个高级前端/Rust架构师和产品开发经理（Planner）。现在我们需要为一个名为「AI 开源项目情报站」的应用实现其实际代码。该项目基于 Tauri + HTML Frontend + Rust 构建。

# 背景上下文
该项目的 MVP 版本（版本 A）核心功能为：利用 Tauri 客户端作为一个轻量级的抓取和展示工具，连接 GitHub API 读取最新的热门 AI 开源库，并调用大模型 API 对仓库信息进行简短摘要，最后持久化至本地 SQLite 中进行列表、详情展示与收藏功能。
现在，我们已经有了项目的 PRD 和技术架构方案。

# 你的任务
请帮我根据以上背景，生成一份详尽、可执行的 `PLAN.md`（开发执行计划），并在接下来的交互中指导我按照此计划一步步写代码（先不要生成代码，先给我计划大纲和拆解步骤）。

# `PLAN.md` 内容要求需拆分为以下阶段（Phases）：

## Phase 0: 基础设施搭建 (Infrastructure)
* 安装所需的前端库 (Vite 端：Tailwind, React-router, Zustand, react-query) 
* 安装所需的 Tauri/Rust 依赖 (reqwest, rusqlite, serde, tokio)
* 设计并在前端 / src-tauri 里面创建出规范的基础目录结构（按照 MVC 或合理的拆分）。

## Phase 1: 核心数据库与模型层实现 (Rust Backend - Data & Model)
* 在 src-tauri 的 `db.rs` 中编写 SQLite 初始化逻辑 (例如建立 projects, collections 等表)。
* 在 `models.rs` 定义对应的前后端通信数据结构 (struct)。
* 编写相关的 Tauri Command，实现对项目的 CRUD (包括获取列表、标星/收藏等)。

## Phase 2: 后台采集任务模块 (Data Fetching & AI Task)
* 编写 Rust 脚本通过 GitHub Search API 抓取符合条件的库（比如包含 ai, llm, agent 等 topic 且 star 增长较快的项目）。
* 调用大模型接口（预留 API Key 配置位置）对未处理过的 README/项目描述 生成核心亮点及“前端关联度分析”。
* 在 Rust 中整合上述两步到数据库存储的链路中，并暴露 `invoke('sync_data')` 命令给前端手动触发。

## Phase 3: 前端页面与交互 (Frontend React Development)
* **首页榜单页**：调用对应的 Tauri Commands 查询本地数据库并做列表化渲染，实现筛选（Language, 是否适合前端）。
* **项目详情页**：展示 AI 提取出的结构化摘要。
* **收藏夹页**：对喜欢的项目进行收藏和取消收藏，并独立页面展示。

## Phase 4: 兜底处理与打包
* API 报错重试兜底。
* 打包输出。

# 约束条件
这只是一份 Planner 提示，请先确认上述步骤是否合理？如合理，请输出一个包含 Checklist 的 `PLAN.md` 文本大纲。等待我确认大纲完毕后，我们再依次开启各个 Phase 下具体代码的生成和落地。
```