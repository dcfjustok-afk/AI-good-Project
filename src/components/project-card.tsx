import { ArrowUpRight, FolderHeart, GitFork, Heart, Star } from "lucide-react";
import { Link } from "react-router-dom";
import type { ProjectSummary } from "../types/project";

type ProjectCardProps = {
  project: ProjectSummary;
  onToggleFavorite?: (projectId: number) => void;
  isTogglingFavorite?: boolean;
};

function formatCompactNumber(value: number) {
  return new Intl.NumberFormat("zh-CN", {
    notation: "compact",
    maximumFractionDigits: 1,
  }).format(value);
}

function getFrontendFitLabel(frontendRelevance: number) {
  if (frontendRelevance >= 3) return "前端相关度高";
  if (frontendRelevance >= 2) return "可做交互研究";
  return "偏后端视角";
}

function formatRelativeDate(value: string) {
  const timestamp = Date.parse(value);

  if (Number.isNaN(timestamp)) {
    return value;
  }

  const diff = Date.now() - timestamp;
  const day = 24 * 60 * 60 * 1000;

  if (diff < day) {
    return "今天";
  }

  if (diff < day * 2) {
    return "1 天前";
  }

  if (diff < day * 30) {
    return `${Math.floor(diff / day)} 天前`;
  }

  return new Intl.DateTimeFormat("zh-CN", {
    month: "numeric",
    day: "numeric",
  }).format(new Date(timestamp));
}

export function ProjectCard({ project, onToggleFavorite, isTogglingFavorite = false }: ProjectCardProps) {
  const description = project.descriptionLong || project.summary;

  return (
    <article className="group rounded-[24px] border border-white/80 bg-white/85 p-5 shadow-card transition hover:-translate-y-1 hover:shadow-2xl">
      <div className="flex items-start justify-between gap-4">
        <div>
          <p className="text-sm font-medium text-slate/70">{project.category}</p>
          <h2 className="mt-2 text-xl font-semibold text-ink">{project.repoName}</h2>
          <p className="mt-2 text-xs font-medium uppercase tracking-[0.24em] text-slate/55">
            推荐分 {project.score}
          </p>
        </div>
        <div className="flex items-center gap-2">
          <span className="rounded-full bg-pine/10 px-3 py-1 text-xs font-medium text-pine">
            {getFrontendFitLabel(project.frontendRelevance)}
          </span>
          {onToggleFavorite ? (
            <button
              type="button"
              disabled={isTogglingFavorite}
              onClick={() => onToggleFavorite(project.id)}
              className={[
                "inline-flex h-10 w-10 items-center justify-center rounded-full border transition",
                project.isFavorite
                  ? "border-accent/30 bg-accent/10 text-accent"
                  : "border-slate/15 bg-white text-slate hover:border-accent/30 hover:text-accent",
              ].join(" ")}
              aria-label={project.isFavorite ? "取消收藏" : "加入收藏"}
            >
              <Heart className="h-4 w-4" fill={project.isFavorite ? "currentColor" : "none"} />
            </button>
          ) : null}
        </div>
      </div>

      <p
        className="mt-4 text-sm leading-6 text-slate/90"
        style={{
          display: "-webkit-box",
          WebkitLineClamp: 4,
          WebkitBoxOrient: "vertical",
          overflow: "hidden",
        }}
      >
        {description}
      </p>

      <div className="mt-5 flex flex-wrap gap-2">
        {project.topics.map((tag) => (
          <span key={tag} className="rounded-full bg-mist px-3 py-1 text-xs font-medium text-slate">
            {tag}
          </span>
        ))}
      </div>

      <div className="mt-6 flex flex-wrap items-center gap-4 text-sm text-slate/80">
        <span className="inline-flex items-center gap-1.5">
          <Star className="h-4 w-4 text-accent" />
          {formatCompactNumber(project.stars)}
        </span>
        <span className="inline-flex items-center gap-1.5">
          <GitFork className="h-4 w-4 text-pine" />
          {formatCompactNumber(project.forks)}
        </span>
        <span className="inline-flex items-center gap-1.5">
          <FolderHeart className="h-4 w-4 text-slate" />
          {project.language || "未知"}
        </span>
      </div>

      <div className="mt-6 flex items-center justify-between border-t border-slate/10 pt-4 text-sm">
        <span className="text-slate/70">最近更新 {formatRelativeDate(project.updatedAt)}</span>
        <Link
          to={`/projects/${project.owner}/${project.repo}`}
          className="inline-flex items-center gap-1.5 font-medium text-ink transition group-hover:text-accent"
        >
          查看详情
          <ArrowUpRight className="h-4 w-4" />
        </Link>
      </div>
    </article>
  );
}
