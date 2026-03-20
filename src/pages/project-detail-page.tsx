import { Link, useParams } from "react-router-dom";
import { ArrowLeft, ExternalLink, Heart } from "lucide-react";
import { useProjectDetail } from "../hooks/use-project-detail";
import { useToggleFavorite } from "../hooks/use-toggle-favorite";
import { QueryState } from "../components/query-state";

export function ProjectDetailPage() {
  const { owner, repo } = useParams();
  const projectName = owner && repo ? `${owner}/${repo}` : "未选择项目";
  const detailQuery = useProjectDetail(owner, repo);
  const toggleFavoriteMutation = useToggleFavorite();

  const detail = detailQuery.data;

  return (
    <section className="space-y-6">
      <Link
        to="/"
        className="inline-flex items-center gap-2 text-sm font-medium text-slate transition hover:text-ink"
      >
        <ArrowLeft className="h-4 w-4" />
        返回榜单
      </Link>

      <article className="glass-surface rounded-[32px] border border-white/80 p-6 shadow-soft sm:p-8">
        {detailQuery.isLoading ? (
          <QueryState
            type="loading"
            title="正在读取项目详情"
            detail="正在从本地数据库加载详情内容，请稍候。"
          />
        ) : null}

        {detailQuery.isFetching && detail ? (
          <p className="text-sm text-slate/70">正在后台刷新，当前优先展示上一次读取到的详情缓存。</p>
        ) : null}

        {detailQuery.isError ? (
          <QueryState
            type="error"
            title="详情读取失败"
            detail="请确认数据库中存在该项目，或先返回首页执行一次同步。"
          />
        ) : null}

        <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
          <div>
            <p className="text-sm uppercase tracking-[0.28em] text-slate/60">项目详情</p>
            <h1 className="mt-2 text-3xl font-semibold text-ink">{detail?.repoName || projectName}</h1>
            <p className="mt-4 max-w-3xl text-sm leading-7 text-slate/80 sm:text-base">
              {detail?.summary || "当前项目暂无摘要内容，可先返回首页触发一次同步。"}
            </p>
          </div>

          <div className="flex flex-wrap gap-3">
            {detail ? (
              <button
                type="button"
                onClick={() => toggleFavoriteMutation.mutate(detail.id)}
                className={[
                  "inline-flex items-center gap-2 rounded-full border px-4 py-2 text-sm font-medium transition",
                  detail.isFavorite
                    ? "border-accent/30 bg-accent/10 text-accent"
                    : "border-ink/10 bg-mist text-ink hover:border-accent hover:text-accent",
                ].join(" ")}
              >
                <Heart className="h-4 w-4" fill={detail.isFavorite ? "currentColor" : "none"} />
                {detail.isFavorite ? "已收藏" : "加入收藏"}
              </button>
            ) : null}

            <a
              href={detail?.githubUrl || `https://github.com/${projectName}`}
              target="_blank"
              rel="noreferrer"
              className="inline-flex items-center gap-2 rounded-full border border-ink/10 bg-mist px-4 py-2 text-sm font-medium text-ink transition hover:border-accent hover:text-accent"
            >
              打开 GitHub
              <ExternalLink className="h-4 w-4" />
            </a>
          </div>
        </div>

        <div className="mt-8 grid gap-4 lg:grid-cols-2">
          <section className="glass-surface rounded-[24px] bg-mist p-5">
            <h2 className="text-lg font-semibold text-ink">结构化摘要</h2>
            <ul className="mt-3 space-y-3 text-sm leading-6 text-slate/80">
              {(detail?.highlights || ["当前仓库暂无亮点摘要，可在下一次同步后补齐"]).map((item) => (
                <li key={item}>• {item}</li>
              ))}
            </ul>

            <div className="mt-5 rounded-2xl bg-white/70 p-4 text-sm text-slate/80">
              <p className="font-medium text-ink">前端价值</p>
              <p className="mt-2">{detail?.frontendValue || "当前未生成前端价值判断，可在同步后重试查看。"}</p>
            </div>
          </section>

          <section className="glass-surface rounded-[24px] bg-mist p-5">
            <h2 className="text-lg font-semibold text-ink">项目元数据</h2>
            <dl className="mt-3 grid gap-3 text-sm text-slate/80 sm:grid-cols-2">
              <div>
                <dt className="font-medium text-ink">语言</dt>
                <dd className="mt-1">{detail?.language || "未知"}</dd>
              </div>
              <div>
                <dt className="font-medium text-ink">Stars</dt>
                <dd className="mt-1">{detail?.stars || 0}</dd>
              </div>
              <div>
                <dt className="font-medium text-ink">Forks</dt>
                <dd className="mt-1">{detail?.forks || 0}</dd>
              </div>
              <div>
                <dt className="font-medium text-ink">Open Issues</dt>
                <dd className="mt-1">{detail?.openIssues || 0}</dd>
              </div>
              <div>
                <dt className="font-medium text-ink">学习成本</dt>
                <dd className="mt-1">{detail?.learningCost || "中"}</dd>
              </div>
              <div>
                <dt className="font-medium text-ink">推荐分</dt>
                <dd className="mt-1">{detail?.score || 0}</dd>
              </div>
              <div>
                <dt className="font-medium text-ink">License</dt>
                <dd className="mt-1">{detail?.license || "未知"}</dd>
              </div>
            </dl>

            <div className="mt-5 flex flex-wrap gap-2">
              {(detail?.topics || []).map((topic) => (
                <span key={topic} className="rounded-full bg-white/80 px-3 py-1 text-xs font-medium text-slate">
                  {topic}
                </span>
              ))}
            </div>

            {detail?.demoUrl || detail?.homepageUrl ? (
              <div className="mt-5 flex flex-wrap gap-3 text-sm">
                {detail.homepageUrl ? (
                  <a href={detail.homepageUrl} target="_blank" rel="noreferrer" className="font-medium text-ink underline-offset-4 hover:underline">
                    官网
                  </a>
                ) : null}
                {detail.demoUrl ? (
                  <a href={detail.demoUrl} target="_blank" rel="noreferrer" className="font-medium text-ink underline-offset-4 hover:underline">
                    Demo
                  </a>
                ) : null}
              </div>
            ) : null}
          </section>
        </div>

        {detail?.useCases?.length ? (
          <section className="glass-surface mt-4 rounded-[24px] bg-mist p-5">
            <h2 className="text-lg font-semibold text-ink">适用场景</h2>
            <ul className="mt-3 space-y-2 text-sm leading-6 text-slate/80">
              {detail.useCases.map((item) => (
                <li key={item}>• {item}</li>
              ))}
            </ul>
          </section>
        ) : null}
      </article>
    </section>
  );
}
