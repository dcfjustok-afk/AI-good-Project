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
      },
      boxShadow: {
        card: "0 24px 60px rgba(17, 32, 49, 0.12)",
      },
      fontFamily: {
        sans: ["SF Pro Display", "PingFang SC", "Hiragino Sans GB", "sans-serif"],
      },
    },
  },
  plugins: [],
};