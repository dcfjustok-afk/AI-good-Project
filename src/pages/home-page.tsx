import { CheckCircle2, Database, RefreshCcw, Server } from "lucide-react";
import { ProjectCard } from "../components/project-card";
import { useAppHealth } from "../hooks/use-app-health";
import { useProjects } from "../hooks/use-projects";
import { useSyncData } from "../hooks/use-sync-data";
import { useToggleFavorite } from "../hooks/use-toggle-favorite";
import { useProjectFiltersStore } from "../store/use-project-filters";

const milestones = [
  {
    title: "Rust 数据层",
    description: "SQLite 初始化、模型定义和 Tauri 命令会在 Phase 1 接入。",
    icon: Database,
  },
  {
    title: "同步链路",
    description: "GitHub 抓取与 MiniMax 摘要生成会在 Phase 2 接入。",
    icon: Server,
  },
  {
    title: "前端交互",
    description: "真实筛选、详情页与收藏流会在 Phase 3 补全。",
    icon: RefreshCcw,
  },
];

export function HomePage() {
  const { frontendOnly, toggleFrontendOnly } = useProjectFiltersStore();
  const healthQuery = useAppHealth();
  const projectsQuery = useProjects({
    frontendOnly,
    sortBy: "score",
    limit: 12,
  });
  const syncDataMutation = useSyncData();
  const toggleFavoriteMutation = useToggleFavorite();
  const syncErrorMessage =
    syncDataMutation.error instanceof Error
      ? syncDataMutation.error.message
      : "请检查网络、GitHub 可达性，以及 MINIMAX_API_KEY / GITHUB_TOKEN 是否已在本地环境中配置。";

  return (
    <div className="space-y-6">
      <section className="grid gap-6 lg:grid-cols-[1.35fr_0.9fr]">
        <div className="rounded-[32px] bg-ink px-6 py-8 text-white shadow-card sm:px-8">
          <p className="text-sm uppercase tracking-[0.3em] text-white/60">MVP Phase 2</p>
          <h1 className="mt-3 max-w-2xl text-3xl font-semibold leading-tight sm:text-4xl">
            本地数据闭环已经打通，现在可以手动同步 GitHub AI 项目并把摘要写回 SQLite。
          </h1>
          <p className="mt-4 max-w-2xl text-sm leading-7 text-white/75 sm:text-base">
            如果本地配置了 MiniMax API Key，同步时会直接调用兼容 OpenAI 的接口生成结构化摘要；否则自动回退到规则摘要，保证链路可用。
          </p>

          <div className="mt-8 flex flex-wrap items-center gap-3">
            <button
              type="button"
              onClick={toggleFrontendOnly}
              className={[
                "rounded-full border px-4 py-2 text-sm font-medium transition",
                frontendOnly
                  ? "border-accent bg-accent text-white"
                  : "border-white/20 bg-white/10 text-white/80 hover:bg-white/20",
              ].join(" ")}
            >
              {frontendOnly ? "仅看前端相关：已开启" : "仅看前端相关：未开启"}
            </button>
            <button
              type="button"
              onClick={() => syncDataMutation.mutate()}
              disabled={syncDataMutation.isPending}
              className="rounded-full border border-white/20 bg-white/10 px-4 py-2 text-sm font-medium text-white/80 transition hover:bg-white/20 disabled:cursor-wait disabled:opacity-60"
            >
              {syncDataMutation.isPending ? "正在同步 GitHub 数据..." : "手动同步 GitHub 榜单"}
            </button>
          </div>

          {syncDataMutation.data ? (
            <div className="mt-4 rounded-2xl border border-white/15 bg-white/10 px-4 py-3 text-sm text-white/80">
              {syncDataMutation.data.message} 本次处理 {syncDataMutation.data.processed} 个仓库，新增 {syncDataMutation.data.inserted} 个，更新 {syncDataMutation.data.updated} 个。
              {syncDataMutation.data.usedFallback ? (
                <p className="mt-2 text-white/70">
                  兜底信息：GitHub 失败 {syncDataMutation.data.githubRequestsFailed} 次，AI 回退 {syncDataMutation.data.aiFallbackCount} 个仓库。
                </p>
              ) : null}
              {syncDataMutation.data.warnings.length ? (
                <ul className="mt-2 space-y-1 text-xs text-white/70">
                  {syncDataMutation.data.warnings.slice(0, 3).map((warning) => (
                    <li key={warning}>- {warning}</li>
                  ))}
                </ul>
              ) : null}
            </div>
          ) : null}

          {syncDataMutation.isError ? (
            <div className="mt-4 rounded-2xl border border-red-300/40 bg-red-500/10 px-4 py-3 text-sm text-red-100">
              同步失败。{syncErrorMessage}
            </div>
          ) : null}
        </div>

        <div className="rounded-[32px] border border-white/80 bg-white/85 p-6 shadow-card backdrop-blur">
          <div className="flex items-center gap-2 text-sm font-medium text-slate/70">
            <CheckCircle2 className="h-4 w-4 text-pine" />
            应用自检
          </div>
          <div className="mt-4 rounded-3xl bg-mist p-4">
            <p className="text-sm text-slate/70">Tauri 健康状态</p>
            <p className="mt-2 text-lg font-semibold text-ink">
              {healthQuery.data?.status || (healthQuery.isLoading ? "读取中..." : "未返回")}
            </p>
            <p className="mt-2 text-sm leading-6 text-slate/80">
              {healthQuery.data?.message || "Rust 端命令返回运行时配置后，这里会展示同步服务是否可用。"}
            </p>
          </div>

          <dl className="mt-5 space-y-4 text-sm text-slate/80">
            <div>
              <dt className="font-medium text-ink">模型</dt>
              <dd className="mt-1">{healthQuery.data?.model || "MiniMax-M2.5"}</dd>
            </div>
            <div>
              <dt className="font-medium text-ink">MiniMax Key</dt>
              <dd className="mt-1">
                {healthQuery.data?.minimaxApiKeyConfigured ? "已配置，可走 AI 摘要" : "未配置，将回退规则摘要"}
              </dd>
            </div>
            <div>
              <dt className="font-medium text-ink">GitHub Token</dt>
              <dd className="mt-1">
                {healthQuery.data?.githubTokenConfigured ? "已配置，可提升速率限制" : "未配置，使用匿名请求"}
              </dd>
            </div>
            <div>
              <dt className="font-medium text-ink">兼容端点</dt>
              <dd className="mt-1 break-all">{healthQuery.data?.baseUrl || "https://api.minimaxi.com/v1"}</dd>
            </div>
            <div>
              <dt className="font-medium text-ink">数据库位置</dt>
              <dd className="mt-1 break-all text-xs">{healthQuery.data?.databasePath || "初始化中"}</dd>
            </div>
          </dl>
        </div>
      </section>

      <section className="grid gap-4 md:grid-cols-3">
        {milestones.map(({ title, description, icon: Icon }) => (
          <article
            key={title}
            className="rounded-[24px] border border-white/80 bg-white/75 p-5 shadow-card backdrop-blur"
          >
            <span className="flex h-11 w-11 items-center justify-center rounded-2xl bg-pine/10 text-pine">
              <Icon className="h-5 w-5" />
            </span>
            <h2 className="mt-4 text-lg font-semibold text-ink">{title}</h2>
            <p className="mt-2 text-sm leading-6 text-slate/80">{description}</p>
          </article>
        ))}
      </section>

      <section className="space-y-4">
        <div className="flex items-end justify-between gap-4">
          <div>
            <p className="text-sm uppercase tracking-[0.28em] text-slate/60">本地榜单</p>
            <h2 className="mt-2 text-2xl font-semibold text-ink">本地 SQLite 已支持种子数据和手动同步结果</h2>
          </div>
        </div>

        {projectsQuery.isLoading ? (
          <div className="rounded-[24px] border border-white/80 bg-white/80 p-6 text-sm text-slate/80 shadow-card">
            正在从本地数据库读取项目列表...
          </div>
        ) : null}

        {projectsQuery.isError ? (
          <div className="rounded-[24px] border border-red-200 bg-red-50 p-6 text-sm text-red-700 shadow-card">
            项目列表读取失败，请检查 Tauri 命令与数据库初始化状态。
          </div>
        ) : null}

        {projectsQuery.data?.length ? (
          <div className="grid gap-4 xl:grid-cols-2">
            {projectsQuery.data.map((project) => (
              <ProjectCard
                key={project.id}
                project={project}
                onToggleFavorite={(projectId) => toggleFavoriteMutation.mutate(projectId)}
                isTogglingFavorite={toggleFavoriteMutation.isPending}
              />
            ))}
          </div>
        ) : null}

        {projectsQuery.data && projectsQuery.data.length === 0 ? (
          <div className="rounded-[24px] border border-white/80 bg-white/80 p-6 text-sm text-slate/80 shadow-card">
            当前筛选条件下没有项目，后续接入真实同步后这里会展示数据库中的 GitHub 仓库列表。
          </div>
        ) : null}
      </section>
    </div>
  );
}