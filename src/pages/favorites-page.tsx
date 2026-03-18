import { Heart } from "lucide-react";

export function FavoritesPage() {
  return (
    <section className="rounded-[28px] border border-white/80 bg-white/80 p-6 shadow-card backdrop-blur">
      <div className="flex items-center gap-3 text-ink">
        <span className="flex h-11 w-11 items-center justify-center rounded-2xl bg-accent/10 text-accent">
          <Heart className="h-5 w-5" />
        </span>
        <div>
          <h1 className="text-2xl font-semibold">收藏夹</h1>
          <p className="text-sm text-slate/80">Phase 1 完成后，这里会接入真实收藏数据与排序能力。</p>
        </div>
      </div>
    </section>
  );
}