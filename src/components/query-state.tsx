import { AlertTriangle, CheckCircle2, Sparkles } from "lucide-react";

type QueryStateProps = {
  type: "loading" | "empty" | "error" | "success";
  title: string;
  detail: string;
  action?: React.ReactNode;
  className?: string;
};

const stateStyles = {
  loading: "border-white/70 bg-white/80 text-slate/80",
  empty: "border-white/70 bg-white/80 text-slate/80",
  error: "border-red-200 bg-red-50 text-red-800",
  success: "border-emerald-200 bg-emerald-50 text-emerald-800",
};

const stateIcons = {
  loading: Sparkles,
  empty: Sparkles,
  error: AlertTriangle,
  success: CheckCircle2,
};

export function QueryState({ type, title, detail, action, className }: QueryStateProps) {
  const Icon = stateIcons[type];

  return (
    <div className={["status-card shadow-soft", stateStyles[type], className].filter(Boolean).join(" ")}>
      <p className="inline-flex items-center gap-2 text-sm font-semibold">
        <Icon className="h-4 w-4" />
        {title}
      </p>
      <p className="mt-2 text-sm leading-6">{detail}</p>
      {action ? <div className="mt-3">{action}</div> : null}
    </div>
  );
}
