# AI 开源项目情报站

基于 Tauri 2、React 19、Vite 7 和 SQLite 的桌面应用，用来抓取 GitHub 上值得关注的 AI 开源项目，并生成适合中文产品/前端团队消费的结构化摘要。

## 当前能力

- 本地 SQLite 持久化项目、摘要和收藏状态
- 首页手动触发 GitHub 同步
- 支持自动定时同步与最近同步时间展示
- 已配置 OpenAI 兼容 AI Key 时生成结构化摘要
- 未配置 AI Key 或单仓库摘要失败时自动回退到规则摘要
- GitHub 请求支持重试与部分失败继续写库
- 列表支持高级搜索、主题过滤、无限加载、排序和收藏状态切换
- 推荐分已结合活跃度、语言、主题和 Demo 信号
- 分类模型已扩展为 Agent、RAG、Frontend、Workflow、Multimodal、DevTools 等细类
- 应用日志会落盘到系统应用数据目录，便于排查同步与数据库错误

## 本地运行

1. 安装依赖

```bash
pnpm install
```

2. 配置环境变量

```bash
cp .env.example .env
```

至少建议填写以下变量：

- AI_API_KEY：你的 OpenAI 兼容 AI Key，若未设置则会回退读取 MINIMAX_API_KEY
- GITHUB_TOKEN：可选，但建议配置以降低 GitHub 速率限制风险
- AI_MODEL：AI 模型名，若未设置则会回退读取 MINIMAX_MODEL
- AI_BASE_URL：OpenAI 兼容接口地址，若未设置则会回退读取 MINIMAX_BASE_URL
- AI_TEMPERATURE：可选，默认 `0.3`；若提供商或模型有特殊限制，可单独调整

3. 启动桌面应用

```bash
pnpm tauri dev
```

## 校验命令

```bash
pnpm build
cd src-tauri && cargo check
cd src-tauri && cargo test
pnpm tauri build
```

## 同步兜底说明

- 没有 AI_API_KEY / MINIMAX_API_KEY：整批同步仍会成功，但摘要来自规则生成
- 单个仓库调用 OpenAI 兼容 AI 提供商失败：只回退该仓库，不影响整批同步
- 部分 GitHub 查询失败：保留成功结果并写入数据库，同时在界面展示警告

## 推荐开发环境

- VS Code
- Tauri 扩展
- rust-analyzer

## 产物与日志

- Windows 打包产物默认位于 src-tauri/target/release/bundle/msi 和 src-tauri/target/release/bundle/nsis
- 应用运行日志默认位于系统应用数据目录下的 logs/ai-good-project.log
