/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: [
      "./components/**/*.{js,jsx,ts,tsx}",
      "./pages/**/*.{js,jsx,ts,tsx,mdx}",
      "./theme.config.jsx",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
