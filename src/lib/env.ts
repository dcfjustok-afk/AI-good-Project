const DEFAULT_APP_NAME = "AI 开源项目情报站";
const DEFAULT_MINIMAX_BASE_URL = "https://api.minimaxi.com/v1";
const DEFAULT_MINIMAX_MODEL = "MiniMax-M2.5";

export const publicAppConfig = {
  appName: import.meta.env.VITE_APP_NAME || DEFAULT_APP_NAME,
  minimaxBaseUrl:
    import.meta.env.VITE_AI_BASE_URL ||
    import.meta.env.VITE_MINIMAX_BASE_URL ||
    DEFAULT_MINIMAX_BASE_URL,
  modelLabel:
    import.meta.env.VITE_AI_MODEL ||
    import.meta.env.VITE_MINIMAX_MODEL ||
    DEFAULT_MINIMAX_MODEL,
};