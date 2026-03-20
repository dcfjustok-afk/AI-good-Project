/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        ink: "#112031",
        accent: "#ff7a59",
        mist: "#f6f2ea",
        pine: "#1b4d3e",
        slate: "#385170",
        dawn: "#f8f1e4",
        sand: "#eadfcd",
        amber: "#d99b45",
      },
      boxShadow: {
        card: "0 24px 60px rgba(17, 32, 49, 0.12)",
        soft: "0 12px 30px rgba(17, 32, 49, 0.08)",
        glow: "0 16px 40px rgba(217, 155, 69, 0.22)",
      },
      fontFamily: {
        sans: ["Avenir Next", "SF Pro Display", "PingFang SC", "Hiragino Sans GB", "sans-serif"],
      },
      keyframes: {
        "fade-up": {
          "0%": { opacity: "0", transform: "translateY(12px)" },
          "100%": { opacity: "1", transform: "translateY(0)" },
        },
        shimmer: {
          "0%": { backgroundPosition: "-300px 0" },
          "100%": { backgroundPosition: "300px 0" },
        },
      },
      animation: {
        "fade-up": "fade-up 420ms cubic-bezier(0.22, 1, 0.36, 1)",
        shimmer: "shimmer 1.6s linear infinite",
      },
    },
  },
  plugins: [],
};
