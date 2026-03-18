import { Link } from "react-router-dom";

export function NotFoundPage() {
  return (
    <main className="flex min-h-screen items-center justify-center px-4">
      <div className="max-w-md rounded-[28px] border border-white/80 bg-white/85 p-8 text-center shadow-card">
        <p className="text-sm uppercase tracking-[0.28em] text-slate/60">404</p>
        <h1 className="mt-3 text-3xl font-semibold text-ink">页面不存在</h1>
        <p className="mt-3 text-sm leading-6 text-slate/80">
          当前只启用了首页、详情页和收藏页基础路由，可能是你访问了尚未实现的路径。
        </p>
        <Link
          to="/"
          className="mt-6 inline-flex rounded-full bg-ink px-5 py-2.5 text-sm font-medium text-white transition hover:bg-slate"
        >
          返回首页
        </Link>
      </div>
    </main>
  );
}