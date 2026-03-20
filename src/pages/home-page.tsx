import { useDeferredValue, useEffect, useMemo, useRef } from "react";
import { CheckCircle2, Database, RefreshCcw, Search, Server, TimerReset } from "lucide-react";
import { ProjectCard } from "../components/project-card";
import { useAppHealth } from "../hooks/use-app-health";
import { useInfiniteProjects } from "../hooks/use-infinite-projects";
import { useProjects } from "../hooks/use-projects";
import { useSyncData } from "../hooks/use-sync-data";
import { useToggleFavorite } from "../hooks/use-toggle-favorite";
import { useProjectFiltersStore } from "../store/use-project-filters";
import { useSyncPreferencesStore } from "../store/use-sync-preferences";
import type { ProjectFilters, ProjectSummary } from "../types/project";

const milestones = [
  {
    title: "Rust 数据层",
    description: "SQLite 初始化、模型定义、分页查询与收藏命令已经接入。",
    icon: Database,
  },
  {
    title: "同步链路",
    description: "GitHub 抓取、OpenAI 兼容摘要、失败重试和规则回退已经打通。",
    icon: Server,
  },
  {
    title: "前端交互",
    description: "首页、详情、收藏、筛选和分页切换都已可用。",
    icon: RefreshCcw,
  },
];

export function HomePage() {
  const {
    search,
    topic,
    language,
    category,
    frontendOnly,
    hasDemo,
    sortBy,
    limit,
    setSearch,
    setTopic,
    setLanguage,
    setCategory,
    toggleFrontendOnly,
    toggleHasDemo,
    setSortBy,
    setLimit,
    resetFilters,
  } = useProjectFiltersStore();
  const autoSyncEnabled = useSyncPreferencesStore((state) => state.autoSyncEnabled);
  const intervalMinutes = useSyncPreferencesStore((state) => state.intervalMinutes);
  const lastCompletedAt = useSyncPreferencesStore((state) => state.lastCompletedAt);
  const setAutoSyncEnabled = useSyncPreferencesStore((state) => state.setAutoSyncEnabled);
  const setIntervalMinutes = useSyncPreferencesStore((state) => state.setIntervalMinutes);
  const healthQuery = useAppHealth();
  const deferredSearch = useDeferredValue(search);
  const projectFilters: Omit<ProjectFilters, "page"> = {
    search: deferredSearch || undefined,
    topic: topic || undefined,
    language: language || undefined,
    category: category || undefined,
    frontendOnly,
    hasDemo,
    sortBy,
    limit,
  };
  const projectsQuery = useInfiniteProjects(projectFilters);
  const facetsQuery = useProjects({
    sortBy: "score",
    page: 1,
    limit: 120,
  });
  const syncDataMutation = useSyncData();
  const toggleFavoriteMutation = useToggleFavorite();
  const sentinelRef = useRef<HTMLDivElement | null>(null);
  const syncErrorMessage =
    syncDataMutation.error instanceof Error
      ? syncDataMutation.error.message
      : "请检查网络、GitHub 可达性，以及 AI_API_KEY 或 MINIMAX_API_KEY / GITHUB_TOKEN 是否已在本地环境中配置。";
  const pages = projectsQuery.data?.pages ?? [];
  const projects = useMemo(
    () => pages.flatMap((pageData) => pageData.items),
    [pages],
  );
  const hasLocalCache = projects.length > 0;
  const cachedHintVisible = projectsQuery.isFetching && !projectsQuery.isFetchingNextPage && projects.length > 0;
  const availableLanguages = collectFacetOptions(facetsQuery.data?.items, "language");
  const availableCategories = collectFacetOptions(facetsQuery.data?.items, "category");
  const availableTopics = collectTopicOptions(facetsQuery.data?.items);
  const latestPage = pages[pages.length - 1];
  const total = latestPage?.total ?? 0;
  const hasMore = Boolean(projectsQuery.hasNextPage);

  useEffect(() => {
    if (!sentinelRef.current || !hasMore) {
      return;
    }

    const observer = new IntersectionObserver(
      (entries) => {
        const entry = entries[0];
        if (entry?.isIntersecting && hasMore && !projectsQuery.isFetchingNextPage) {
          void projectsQuery.fetchNextPage();
        }
      },
      {
        rootMargin: "220px 0px",
      },
    );

    observer.observe(sentinelRef.current);
    return () => observer.disconnect();
  }, [hasMore, projectsQuery]);

  return (
    <div className="space-y-6">
      <section className="grid gap-6 lg:grid-cols-[1.35fr_0.9fr]">
        <div className="rounded-[32px] bg-ink px-6 py-8 text-white shadow-card sm:px-8">
          <p className="text-sm uppercase tracking-[0.3em] text-white/60">Next Iteration</p>
          <h1 className="mt-3 max-w-2xl text-3xl font-semibold leading-tight sm:text-4xl">
            现在不仅能手动同步，还能自动定时拉取、做高级搜索，并以无限加载方式浏览 AI 开源项目榜单。
          </h1>
          <p className="mt-4 max-w-2xl text-sm leading-7 text-white/75 sm:text-base">
            推荐分已经加入活跃度、主题、语言和 Demo 信号，更细分类会在同步时自动归一；如果本地配置了 OpenAI 兼容 AI Key，则继续生成结构化摘要，否则自动回退规则摘要。
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

          <div className="mt-4 grid gap-3 sm:grid-cols-2">
            <label className="rounded-2xl border border-white/15 bg-white/10 px-4 py-3 text-sm text-white/80">
              <span className="flex items-center gap-2 font-medium text-white">
                <TimerReset className="h-4 w-4" />
                自动同步
              </span>
              <div className="mt-3 flex items-center justify-between gap-3">
                <select
                  value={String(intervalMinutes)}
                  onChange={(event) => setIntervalMinutes(Number(event.target.value))}
                  className="rounded-xl border border-white/15 bg-white/10 px-3 py-2 text-sm text-white outline-none"
                >
                  <option value="15">每 15 分钟</option>
                  <option value="30">每 30 分钟</option>
                  <option value="60">每 60 分钟</option>
                </select>
                <button
                  type="button"
                  onClick={() => setAutoSyncEnabled(!autoSyncEnabled)}
                  className={[
                    "rounded-full border px-4 py-2 text-sm font-medium transition",
                    autoSyncEnabled
                      ? "border-white/30 bg-white text-ink"
                      : "border-white/20 bg-white/10 text-white/80 hover:bg-white/20",
                  ].join(" ")}
                >
                  {autoSyncEnabled ? "已开启" : "未开启"}
                </button>
              </div>
              <p className="mt-2 text-xs text-white/65">
                {lastCompletedAt
                  ? `最近一次自动同步：${formatDateTime(lastCompletedAt)}`
                  : "自动同步开启后，会在后台定时刷新本地榜单。"}
              </p>
            </label>

            <label className="rounded-2xl border border-white/15 bg-white/10 px-4 py-3 text-sm text-white/80">
              <span className="flex items-center gap-2 font-medium text-white">
                <Search className="h-4 w-4" />
                高级搜索
              </span>
              <input
                value={search}
                onChange={(event) => setSearch(event.target.value)}
                placeholder="搜索仓库名、描述、摘要或主题"
                className="mt-3 w-full rounded-xl border border-white/15 bg-white/10 px-4 py-3 text-sm text-white outline-none placeholder:text-white/45"
              />
              <p className="mt-2 text-xs text-white/65">
                支持按关键字检索 repo 名、描述、摘要与 topics。
              </p>
            </label>
          </div>

          {cachedHintVisible ? (
            <div className="mt-4 rounded-2xl border border-white/15 bg-white/10 px-4 py-3 text-sm text-white/75">
              正在后台刷新，当前优先展示上一次成功查询的本地缓存结果。
            </div>
          ) : null}

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
            <div
              className={[
                "mt-4 rounded-2xl px-4 py-3 text-sm",
                hasLocalCache
                  ? "border border-amber-300/40 bg-amber-500/10 text-amber-50"
                  : "border border-red-300/40 bg-red-500/10 text-red-100",
              ].join(" ")}
            >
              {hasLocalCache ? (
                <>
                  同步失败，已明确回退到本地缓存。当前仍可浏览 {projects.length} 条已落库项目。{syncErrorMessage}
                </>
              ) : (
                <>同步失败，且当前没有可回退的本地缓存。{syncErrorMessage}</>
              )}
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
              <dt className="font-medium text-ink">AI Key</dt>
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
            <div>
              <dt className="font-medium text-ink">日志位置</dt>
              <dd className="mt-1 break-all text-xs">{healthQuery.data?.logPath || "初始化中"}</dd>
            </div>
            <div>
              <dt className="font-medium text-ink">项目数量</dt>
              <dd className="mt-1">{healthQuery.data?.projectCount ?? 0}</dd>
            </div>
            <div>
              <dt className="font-medium text-ink">收藏数量</dt>
              <dd className="mt-1">{healthQuery.data?.favoriteCount ?? 0}</dd>
            </div>
            <div>
              <dt className="font-medium text-ink">最近同步</dt>
              <dd className="mt-1">{healthQuery.data?.lastSyncedAt ? formatDateTime(healthQuery.data.lastSyncedAt) : "尚未同步"}</dd>
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

      <section className="rounded-[28px] border border-white/80 bg-white/75 p-5 shadow-card backdrop-blur">
        <div className="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
          <div>
            <p className="text-sm uppercase tracking-[0.28em] text-slate/60">真实筛选</p>
            <h2 className="mt-2 text-2xl font-semibold text-ink">按语言、分类、Demo 和排序规则筛选本地榜单</h2>
          </div>
          <button
            type="button"
            onClick={resetFilters}
            className="inline-flex w-fit items-center rounded-full border border-slate/15 bg-white px-4 py-2 text-sm font-medium text-slate transition hover:border-accent hover:text-accent"
          >
            重置筛选
          </button>
        </div>

        <div className="mt-5 grid gap-4 md:grid-cols-2 xl:grid-cols-5">
          <label className="space-y-2 text-sm text-slate/80">
            <span className="font-medium text-ink">语言</span>
            <select
              value={language}
              onChange={(event) => setLanguage(event.target.value)}
              className="w-full rounded-2xl border border-slate/15 bg-white px-4 py-3 text-sm text-ink outline-none transition focus:border-accent"
            >
              <option value="">全部语言</option>
              {availableLanguages.map((item) => (
                <option key={item} value={item}>
                  {item}
                </option>
              ))}
            </select>
          </label>

          <label className="space-y-2 text-sm text-slate/80">
            <span className="font-medium text-ink">分类</span>
            <select
              value={category}
              onChange={(event) => setCategory(event.target.value)}
              className="w-full rounded-2xl border border-slate/15 bg-white px-4 py-3 text-sm text-ink outline-none transition focus:border-accent"
            >
              <option value="">全部分类</option>
              {availableCategories.map((item) => (
                <option key={item} value={item}>
                  {item}
                </option>
              ))}
            </select>
          </label>

          <label className="space-y-2 text-sm text-slate/80">
            <span className="font-medium text-ink">主题</span>
            <select
              value={topic}
              onChange={(event) => setTopic(event.target.value)}
              className="w-full rounded-2xl border border-slate/15 bg-white px-4 py-3 text-sm text-ink outline-none transition focus:border-accent"
            >
              <option value="">全部主题</option>
              {availableTopics.map((item) => (
                <option key={item} value={item}>
                  {item}
                </option>
              ))}
            </select>
          </label>

          <label className="space-y-2 text-sm text-slate/80">
            <span className="font-medium text-ink">排序</span>
            <select
              value={sortBy}
              onChange={(event) => setSortBy(event.target.value as NonNullable<ProjectFilters["sortBy"]>)}
              className="w-full rounded-2xl border border-slate/15 bg-white px-4 py-3 text-sm text-ink outline-none transition focus:border-accent"
            >
              <option value="score">综合推荐</option>
              <option value="stars">Star 数</option>
              <option value="updatedAt">最近更新</option>
              <option value="frontendRelevance">前端相关度</option>
            </select>
          </label>

          <label className="space-y-2 text-sm text-slate/80">
            <span className="font-medium text-ink">展示数量</span>
            <select
              value={String(limit)}
              onChange={(event) => setLimit(Number(event.target.value))}
              className="w-full rounded-2xl border border-slate/15 bg-white px-4 py-3 text-sm text-ink outline-none transition focus:border-accent"
            >
              <option value="12">12</option>
              <option value="24">24</option>
              <option value="48">48</option>
            </select>
          </label>

          <div className="grid gap-3 text-sm text-slate/80 sm:grid-cols-2 md:grid-cols-1">
            <button
              type="button"
              onClick={toggleFrontendOnly}
              className={toggleClassName(frontendOnly)}
            >
              {frontendOnly ? "仅看前端相关：已开启" : "仅看前端相关：未开启"}
            </button>
            <button
              type="button"
              onClick={toggleHasDemo}
              className={toggleClassName(hasDemo)}
            >
              {hasDemo ? "仅看带 Demo：已开启" : "仅看带 Demo：未开启"}
            </button>
          </div>
        </div>
      </section>

      <section className="space-y-4">
        <div className="flex items-end justify-between gap-4">
          <div>
            <p className="text-sm uppercase tracking-[0.28em] text-slate/60">本地榜单</p>
            <h2 className="mt-2 text-2xl font-semibold text-ink">本地 SQLite 已支持高级搜索、无限加载和精细推荐分</h2>
          </div>
          <div className="rounded-full bg-white/80 px-4 py-2 text-sm text-slate/80 shadow-sm">
            已加载 {projects.length} / {total} 条
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

        {syncDataMutation.isError && hasLocalCache ? (
          <div className="rounded-[24px] border border-amber-200 bg-amber-50 p-6 text-sm text-amber-900 shadow-card">
            当前列表来自本地缓存而非最新同步结果。你仍可继续筛选、查看详情和收藏，待网络恢复后再重试同步即可。
          </div>
        ) : null}

        {projects.length ? (
          <>
            <div className="grid gap-4 xl:grid-cols-2">
              {projects.map((project) => (
                <ProjectCard
                  key={project.id}
                  project={project}
                  onToggleFavorite={(projectId) => toggleFavoriteMutation.mutate(projectId)}
                  isTogglingFavorite={toggleFavoriteMutation.isPending}
                />
              ))}
            </div>

            <div className="flex flex-col gap-3 rounded-[24px] border border-white/80 bg-white/80 p-4 text-sm text-slate/80 shadow-card sm:flex-row sm:items-center sm:justify-between">
              <p>无限加载已启用，滚动到底部会继续读取下一批项目；当前每批 {limit} 条。</p>
              <div className="flex items-center gap-3">
                <button
                  type="button"
                  onClick={() => void projectsQuery.fetchNextPage()}
                  disabled={!hasMore}
                  className="rounded-full border border-slate/15 bg-white px-4 py-2 font-medium text-slate transition hover:border-accent hover:text-accent disabled:cursor-not-allowed disabled:opacity-50"
                >
                  {hasMore ? "继续加载" : "已加载完毕"}
                </button>
              </div>
            </div>

            <div ref={sentinelRef} className="h-2" />

            {projectsQuery.isFetchingNextPage ? (
              <div className="rounded-[24px] border border-white/80 bg-white/80 p-4 text-sm text-slate/70 shadow-card">
                正在加载更多项目...
              </div>
            ) : null}
          </>
        ) : null}

        {projectsQuery.data && projects.length === 0 ? (
          <div className="rounded-[24px] border border-white/80 bg-white/80 p-6 text-sm text-slate/80 shadow-card">
            当前筛选条件下没有项目。你可以放宽语言、分类或 Demo 条件，或者先触发一次同步。
          </div>
        ) : null}
      </section>
    </div>
  );
}

function collectFacetOptions(projects: ProjectSummary[] | undefined, key: "language" | "category") {
  if (!projects?.length) {
    return [];
  }

  return [...new Set(projects.map((project) => project[key]).filter(Boolean))].sort((left, right) =>
    String(left).localeCompare(String(right), "zh-CN")
  ) as string[];
}

function toggleClassName(active: boolean) {
  return [
    "rounded-2xl border px-4 py-3 text-left text-sm font-medium transition",
    active
      ? "border-accent/30 bg-accent/10 text-accent"
      : "border-slate/15 bg-white text-slate hover:border-accent hover:text-accent",
  ].join(" ");
}

function collectTopicOptions(projects: ProjectSummary[] | undefined) {
  if (!projects?.length) {
    return [];
  }

  return [...new Set(projects.flatMap((project) => project.topics))].sort((left, right) =>
    left.localeCompare(right, "zh-CN")
  );
}

function formatDateTime(value: string) {
  const timestamp = Date.parse(value);
  if (Number.isNaN(timestamp)) {
    return value;
  }

  return new Intl.DateTimeFormat("zh-CN", {
    month: "numeric",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(new Date(timestamp));
}