/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: [
      "./components/**/*.tsx",
      "./pages/**/*.{tsx,mdx}",
      "./theme.config.jsx",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
