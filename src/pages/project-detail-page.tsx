import { Link, useParams } from "react-router-dom";
import { ArrowLeft, ExternalLink } from "lucide-react";

export function ProjectDetailPage() {
  const { owner, repo } = useParams();
  const projectName = owner && repo ? `${owner}/${repo}` : "未选择项目";

  return (
    <section className="space-y-6">
      <Link
        to="/"
        className="inline-flex items-center gap-2 text-sm font-medium text-slate transition hover:text-ink"
      >
        <ArrowLeft className="h-4 w-4" />
        返回榜单
      </Link>

      <article className="rounded-[32px] border border-white/80 bg-white/85 p-6 shadow-card backdrop-blur sm:p-8">
        <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
          <div>
            <p className="text-sm uppercase tracking-[0.28em] text-slate/60">详情页骨架</p>
            <h1 className="mt-2 text-3xl font-semibold text-ink">{projectName}</h1>
            <p className="mt-4 max-w-3xl text-sm leading-7 text-slate/80 sm:text-base">
              Phase 1 和 Phase 2 完成后，此页面会接入仓库基础信息、AI 摘要、亮点、适用场景、前端价值说明与收藏操作。
            </p>
          </div>

          <a
            href={`https://github.com/${projectName}`}
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-2 rounded-full border border-ink/10 bg-mist px-4 py-2 text-sm font-medium text-ink transition hover:border-accent hover:text-accent"
          >
            打开 GitHub
            <ExternalLink className="h-4 w-4" />
          </a>
        </div>

        <div className="mt-8 grid gap-4 lg:grid-cols-2">
          <section className="rounded-[24px] bg-mist p-5">
            <h2 className="text-lg font-semibold text-ink">结构化摘要</h2>
            <p className="mt-3 text-sm leading-6 text-slate/80">
              这里将展示一句话介绍、核心亮点、适用场景、前端价值和学习成本。
            </p>
          </section>

          <section className="rounded-[24px] bg-mist p-5">
            <h2 className="text-lg font-semibold text-ink">项目元数据</h2>
            <p className="mt-3 text-sm leading-6 text-slate/80">
              这里将展示 stars、forks、语言、最近更新时间、topics、license 与 Demo 链接。
            </p>
          </section>
        </div>
      </article>
    </section>
  );
}