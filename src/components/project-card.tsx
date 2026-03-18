import { ArrowUpRight, FolderHeart, GitFork, Star } from "lucide-react";
import { Link } from "react-router-dom";
import type { ProjectCardViewModel } from "../types/project";

type ProjectCardProps = {
  project: ProjectCardViewModel;
};

export function ProjectCard({ project }: ProjectCardProps) {
  return (
    <article className="group rounded-[24px] border border-white/80 bg-white/85 p-5 shadow-card transition hover:-translate-y-1 hover:shadow-2xl">
      <div className="flex items-start justify-between gap-4">
        <div>
          <p className="text-sm font-medium text-slate/70">{project.category}</p>
          <h2 className="mt-2 text-xl font-semibold text-ink">{project.name}</h2>
        </div>
        <span className="rounded-full bg-pine/10 px-3 py-1 text-xs font-medium text-pine">
          {project.frontendFitLabel}
        </span>
      </div>

      <p className="mt-4 text-sm leading-6 text-slate/90">{project.summary}</p>

      <div className="mt-5 flex flex-wrap gap-2">
        {project.tags.map((tag) => (
          <span key={tag} className="rounded-full bg-mist px-3 py-1 text-xs font-medium text-slate">
            {tag}
          </span>
        ))}
      </div>

      <div className="mt-6 flex flex-wrap items-center gap-4 text-sm text-slate/80">
        <span className="inline-flex items-center gap-1.5">
          <Star className="h-4 w-4 text-accent" />
          {project.stars}
        </span>
        <span className="inline-flex items-center gap-1.5">
          <GitFork className="h-4 w-4 text-pine" />
          {project.forks}
        </span>
        <span className="inline-flex items-center gap-1.5">
          <FolderHeart className="h-4 w-4 text-slate" />
          {project.language}
        </span>
      </div>

      <div className="mt-6 flex items-center justify-between border-t border-slate/10 pt-4 text-sm">
        <span className="text-slate/70">最近更新 {project.updatedAt}</span>
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