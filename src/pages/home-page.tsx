import { CheckCircle2, Database, RefreshCcw, Server } from "lucide-react";
import { ProjectCard } from "../components/project-card";
import { useAppHealth } from "../hooks/use-app-health";
import { useProjects } from "../hooks/use-projects";
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
  const toggleFavoriteMutation = useToggleFavorite();

  return (
    <div className="space-y-6">
      <section className="grid gap-6 lg:grid-cols-[1.35fr_0.9fr]">
        <div className="rounded-[32px] bg-ink px-6 py-8 text-white shadow-card sm:px-8">
          <p className="text-sm uppercase tracking-[0.3em] text-white/60">MVP Phase 0</p>
          <h1 className="mt-3 max-w-2xl text-3xl font-semibold leading-tight sm:text-4xl">
            用桌面端骨架先跑通路由、运行时配置和 Tauri 通信，再逐步接入真实数据链路。
          </h1>
          <p className="mt-4 max-w-2xl text-sm leading-7 text-white/75 sm:text-base">
            当前页面使用占位项目展示未来信息布局，同时已经验证前端可通过 React Query 调用 Rust 命令读取应用健康状态。
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
            <span className="rounded-full border border-white/15 bg-white/10 px-4 py-2 text-sm text-white/70">
              下一步将接入真实筛选查询参数
            </span>
          </div>
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
            <h2 className="mt-2 text-2xl font-semibold text-ink">Phase 1 已接入 SQLite 种子数据与查询命令</h2>
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