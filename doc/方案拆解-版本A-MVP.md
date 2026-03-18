# AI 开源项目情报站 方案拆解（版本 A：MVP 轻量版）

## 1. 页面结构

### 1.1 首页
#### 页面目标
让用户最快速看到当前值得关注的 AI 项目。

#### 页面模块
1. 顶部导航
   - Logo / 产品名
   - 搜索框
   - 分类入口
   - 我的收藏入口
2. 热门切换区
   - 今日热门
   - 本周热门
   - 本月热门
3. 筛选区
   - 分类筛选
   - 编程语言
   - 是否适合前端
   - 是否有 Demo
4. 项目列表区
   - 项目卡片
   - 分页 / 无限滚动
5. 侧边信息区
   - 热门标签
   - 推荐分类

### 1.2 分类列表页
#### 页面目标
按特定方向查看项目集合。

#### 页面模块
- 分类标题
- 分类说明
- 筛选器
- 项目列表

### 1.3 项目详情页
#### 页面目标
帮助用户快速判断单个项目是否值得关注。

#### 页面模块
1. 基础信息区
2. AI 摘要区
3. 核心亮点区
4. 技术栈区
5. 前端价值区
6. Demo / 官网链接区
7. GitHub 原链接区

### 1.4 我的收藏页
#### 页面目标
管理个人感兴趣项目。

#### 页面模块
- 收藏列表
- 排序方式
- 最近更新状态

## 2. 功能清单

### 2.1 首页功能
- 查看热门 AI 项目
- 切换时间维度
- 切换排序方式
- 按条件筛选
- 查看项目卡片摘要
- 进入项目详情页

### 2.2 详情页功能
- 查看 GitHub 数据
- 查看 AI 摘要
- 查看前端相关性
- 打开外部链接
- 收藏项目

### 2.3 收藏功能
- 添加收藏
- 取消收藏
- 查看收藏列表

### 2.4 后台任务功能
- 定时抓取 GitHub 数据
- 定时生成摘要
- 更新榜单缓存

## 3. 技术方案

## 3.1 总体架构
建议采用前后端分离，但 MVP 可以优先做成“前端 + 本地服务 / 轻量 API”的模式。

### 技术组合建议
- 桌面端：Tauri
- 前端：React + TypeScript + Vite
- UI：可选 Tailwind CSS 或现有 CSS 模块
- 状态管理：Zustand
- 数据请求：TanStack Query
- 后端能力：Tauri Rust Command 或 Node.js API 服务
- 数据存储：SQLite
- 定时任务：服务端 Cron 或桌面端启动时增量同步
- AI 摘要：调用大模型 API

## 3.2 适合当前项目的实现方式
考虑到当前工程已是 Tauri + React，建议直接基于现有项目迭代。

### 前端职责
- 榜单展示
- 分类筛选
- 详情页渲染
- 收藏交互
- 本地缓存展示

### Tauri / 后端职责
- GitHub API 拉取
- 项目数据清洗
- 热度评分计算
- AI 摘要生成
- SQLite 持久化

## 3.3 数据流
1. 定时从 GitHub API 获取 AI 项目数据。
2. 服务端或 Tauri 层对项目做分类与评分。
3. 对 README / 描述生成 AI 摘要。
4. 结构化存入 SQLite。
5. 前端读取接口数据进行展示。

## 3.4 GitHub 数据获取方案
### 方案建议
- 使用 GitHub Search API 获取 AI 相关仓库
- 结合 topic、description、stars、updated_at 做初筛
- 使用定时任务保存快照

### 关键词建议
- ai
- llm
- agent
- rag
- gpt
- multimodal
- image generation
- tts
- speech

## 3.5 AI 摘要方案
### 输入
- repo name
- description
- topics
- README 摘要内容
- 语言与活跃度信息

### 输出模板
- 一句话介绍
- 核心亮点 3 条
- 适用场景
- 前端价值
- 学习成本

## 3.6 数据表建议
### repositories
- id
- full_name
- owner
- description
- language
- stars
- forks
- issues
- topics
- homepage_url
- demo_url
- license
- updated_at
- score
- category
- frontend_relevance

### summaries
- repo_id
- summary
- highlights
- use_cases
- frontend_value
- learning_cost
- generated_at

### favorites
- id
- repo_id
- user_id
- created_at

## 3.7 页面路由建议
- /
- /category/:slug
- /project/:owner/:repo
- /favorites

## 3.8 MVP 开发优先级
### 第一阶段
- GitHub 数据抓取
- 首页项目列表
- 筛选器
- 项目详情页

### 第二阶段
- AI 摘要
- 收藏功能
- 本地持久化

### 第三阶段
- 评分优化
- 错误处理与缓存优化

## 4. 交付建议
- 先完成可浏览、可筛选、可查看详情的闭环。
- 再补 AI 摘要与收藏。
- 最后优化推荐分和数据质量。

## 5. MVP 总结
该版本应优先验证三件事：
1. 团队是否真的需要统一入口。
2. AI 摘要是否能明显降低阅读成本。
3. 前端相关标签是否能提升项目点击与收藏效率。