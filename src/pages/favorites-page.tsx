import { Heart } from "lucide-react";
import { ProjectCard } from "../components/project-card";
import { useFavorites } from "../hooks/use-favorites";
import { useToggleFavorite } from "../hooks/use-toggle-favorite";

export function FavoritesPage() {
  const favoritesQuery = useFavorites();
  const toggleFavoriteMutation = useToggleFavorite();

  return (
    <section className="space-y-5 rounded-[28px] border border-white/80 bg-white/80 p-6 shadow-card backdrop-blur">
      <div className="flex items-center gap-3 text-ink">
        <span className="flex h-11 w-11 items-center justify-center rounded-2xl bg-accent/10 text-accent">
          <Heart className="h-5 w-5" />
        </span>
        <div>
          <h1 className="text-2xl font-semibold">收藏夹</h1>
          <p className="text-sm text-slate/80">这里已经接入本地收藏数据，后续会补充排序和同步后的动态刷新。</p>
        </div>
      </div>

      {favoritesQuery.isLoading ? <p className="text-sm text-slate/80">正在读取收藏列表...</p> : null}
      {favoritesQuery.isError ? <p className="text-sm text-red-700">收藏列表读取失败。</p> : null}

      {favoritesQuery.data?.length ? (
        <div className="grid gap-4 xl:grid-cols-2">
          {favoritesQuery.data.map((project) => (
            <ProjectCard
              key={project.id}
              project={project}
              onToggleFavorite={(projectId) => toggleFavoriteMutation.mutate(projectId)}
              isTogglingFavorite={toggleFavoriteMutation.isPending}
            />
          ))}
        </div>
      ) : null}

      {favoritesQuery.data && favoritesQuery.data.length === 0 ? (
        <div className="rounded-[24px] bg-mist p-5 text-sm text-slate/80">
          还没有收藏项目。你可以在首页卡片或详情页把感兴趣的仓库加入收藏。
        </div>
      ) : null}
    </section>
  );
}