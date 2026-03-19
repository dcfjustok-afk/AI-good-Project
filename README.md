# AI 开源项目情报站

基于 Tauri 2、React 19、Vite 7 和 SQLite 的桌面应用，用来抓取 GitHub 上值得关注的 AI 开源项目，并生成适合中文产品/前端团队消费的结构化摘要。

## 当前能力

- 本地 SQLite 持久化项目、摘要和收藏状态
- 首页手动触发 GitHub 同步
- 已配置 MiniMax Key 时生成 AI 结构化摘要
- 未配置 MiniMax 或单仓库摘要失败时自动回退到规则摘要
- GitHub 请求支持重试与部分失败继续写库
- 列表支持分页、筛选、排序和收藏状态切换
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

- MINIMAX_API_KEY：你的 MiniMax Key
- GITHUB_TOKEN：可选，但建议配置以降低 GitHub 速率限制风险
- MINIMAX_MODEL：默认是 MiniMax-M2.5
- MINIMAX_BASE_URL：默认是 https://api.minimaxi.com/v1

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

- 没有 MINIMAX_API_KEY：整批同步仍会成功，但摘要来自规则生成
- 单个仓库调用 MiniMax 失败：只回退该仓库，不影响整批同步
- 部分 GitHub 查询失败：保留成功结果并写入数据库，同时在界面展示警告

## 推荐开发环境

- VS Code
- Tauri 扩展
- rust-analyzer

## 产物与日志

- Windows 打包产物默认位于 src-tauri/target/release/bundle/msi 和 src-tauri/target/release/bundle/nsis
- 应用运行日志默认位于系统应用数据目录下的 logs/ai-good-project.log
