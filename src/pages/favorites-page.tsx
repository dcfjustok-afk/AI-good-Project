import { Heart } from "lucide-react";
import { ProjectCard } from "../components/project-card";
import { useFavorites } from "../hooks/use-favorites";
import { useToggleFavorite } from "../hooks/use-toggle-favorite";
import { useState } from "react";
import type { ProjectFilters } from "../types/project";

export function FavoritesPage() {
  const [sortBy, setSortBy] = useState<NonNullable<ProjectFilters["sortBy"]>>("favoritedAt");
  const favoritesQuery = useFavorites(sortBy);
  const toggleFavoriteMutation = useToggleFavorite();
  const favorites = favoritesQuery.data ?? [];

  return (
    <section className="space-y-5 rounded-[28px] border border-white/80 bg-white/80 p-6 shadow-card backdrop-blur">
      <div className="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
        <div className="flex items-center gap-3 text-ink">
        <span className="flex h-11 w-11 items-center justify-center rounded-2xl bg-accent/10 text-accent">
          <Heart className="h-5 w-5" />
        </span>
        <div>
          <h1 className="text-2xl font-semibold">收藏夹</h1>
          <p className="text-sm text-slate/80">收藏页现已支持按收藏时间、更新时间和热度排序，刷新时会优先显示缓存结果。</p>
        </div>
      </div>
        <label className="space-y-2 text-sm text-slate/80">
          <span className="font-medium text-ink">排序</span>
          <select
            value={sortBy}
            onChange={(event) => setSortBy(event.target.value as NonNullable<ProjectFilters["sortBy"]>)}
            className="w-full min-w-44 rounded-2xl border border-slate/15 bg-white px-4 py-3 text-sm text-ink outline-none transition focus:border-accent"
          >
            <option value="favoritedAt">最近收藏</option>
            <option value="updatedAt">最近更新</option>
            <option value="stars">Star 数</option>
            <option value="frontendRelevance">前端相关度</option>
          </select>
        </label>
      </div>

      {favoritesQuery.isLoading ? <p className="text-sm text-slate/80">正在读取收藏列表...</p> : null}
      {favoritesQuery.isFetching && favorites.length ? (
        <p className="text-sm text-slate/70">正在后台刷新，当前展示的是缓存中的收藏结果。</p>
      ) : null}
      {favoritesQuery.isError ? <p className="text-sm text-red-700">收藏列表读取失败。</p> : null}

      {favorites.length ? (
        <div className="grid gap-4 xl:grid-cols-2">
          {favorites.map((project) => (
            <ProjectCard
              key={project.id}
              project={project}
              onToggleFavorite={(projectId) => toggleFavoriteMutation.mutate(projectId)}
              isTogglingFavorite={toggleFavoriteMutation.isPending}
            />
          ))}
        </div>
      ) : null}

      {favoritesQuery.data && favorites.length === 0 ? (
        <div className="rounded-[24px] bg-mist p-5 text-sm text-slate/80">
          还没有收藏项目。你可以在首页卡片或详情页把感兴趣的仓库加入收藏。
        </div>
      ) : null}
    </section>
  );
}