import { Link, NavLink, Outlet } from "react-router-dom";
import { BrainCircuit, Heart, House, Sparkles } from "lucide-react";
import { publicAppConfig } from "../lib/env";
import { useAutoSync } from "../hooks/use-auto-sync";

const navigationItems = [
  { to: "/", label: "首页", icon: House },
  { to: "/favorites", label: "收藏夹", icon: Heart },
];

export function AppLayout() {
  useAutoSync();

  return (
    <div className="min-h-screen text-ink">
      <div className="mx-auto flex min-h-screen max-w-7xl flex-col px-4 py-6 sm:px-6 lg:px-8">
        <header className="rounded-[28px] border border-white/70 bg-white/70 px-6 py-5 shadow-card backdrop-blur">
          <div className="flex flex-col gap-6 lg:flex-row lg:items-center lg:justify-between">
            <div className="space-y-3">
              <Link to="/" className="inline-flex items-center gap-3 text-lg font-semibold">
                <span className="flex h-12 w-12 items-center justify-center rounded-2xl bg-ink text-white">
                  <BrainCircuit className="h-6 w-6" />
                </span>
                <span>
                  <span className="block text-sm uppercase tracking-[0.28em] text-slate/70">
                    AI Open Source Radar
                  </span>
                  <span className="block text-2xl font-semibold">{publicAppConfig.appName}</span>
                </span>
              </Link>
              <p className="max-w-2xl text-sm text-slate/80 sm:text-base">
                本地数据、自动同步、无限加载、详情与收藏闭环已经接通；当前界面支持更细的搜索、分类和缓存优先展示。
              </p>
            </div>

            <div className="flex flex-col gap-3 sm:flex-row sm:items-center">
              <div className="rounded-2xl border border-accent/20 bg-accent/10 px-4 py-3 text-sm text-ink">
                <div className="flex items-center gap-2 font-medium">
                  <Sparkles className="h-4 w-4 text-accent" />
                  模型配置
                </div>
                <p className="mt-1 text-slate/80">{publicAppConfig.modelLabel}</p>
              </div>

              <nav className="flex items-center gap-2 rounded-2xl border border-white/80 bg-white/80 p-2 shadow-sm backdrop-blur">
                {navigationItems.map(({ to, label, icon: Icon }) => (
                  <NavLink
                    key={to}
                    to={to}
                    className={({ isActive }) =>
                      [
                        "inline-flex items-center gap-2 rounded-xl px-4 py-2 text-sm font-medium transition",
                        isActive ? "bg-ink text-white" : "text-slate hover:bg-mist",
                      ].join(" ")
                    }
                  >
                    <Icon className="h-4 w-4" />
                    {label}
                  </NavLink>
                ))}
              </nav>
            </div>
          </div>
        </header>

        <main className="flex-1 py-6">
          <Outlet />
        </main>
      </div>
    </div>
  );
}