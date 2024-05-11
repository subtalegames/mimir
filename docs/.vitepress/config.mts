import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "Mímir",
  description: "Contextual query engine for dynamic video games.",
  srcDir: "src",

  themeConfig: {
    logo: {
      light: "/mimir-light.svg",
      dark: "/mimir-dark.svg",
    },

    siteTitle: false,

    footer: {
      message: "Made with ❤️ in Exeter",
      copyright: "Copyright &copy; 2024 Subtale"
    },

    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/overview' },
      { text: 'Release notes', link: '/release-notes' },
      { text: 'API Reference', link: 'https://docs.rs/subtale-mimir' },
    ],

    sidebar: [
      {
        text: 'Introduction',
        items: [
          { text: 'High-level overview', link: '/overview' },
          { text: 'Quick start', link: '/quick-start' },
          { text: 'Inspiration', link: '/inspiration' },
        ]
      },
      {
        text: 'Concepts',
        items: [
          { text: 'Evaluator', link: '/concepts/evaluator' },
          { text: 'Query', link: '/concepts/query' },
          { text: 'Rule', link: '/concepts/rule' },
          { text: 'Ruleset', link: '/concepts/ruleset' },
        ]
      },
      {
        text: 'Recipes',
        items: [
          { text: 'Loading screen tips', link: '/recipes/tips' },
          { text: 'Repeated evaluations', link: '/recipes/repeated-evaluations' },
        ]
      },
      {
        text: 'Miscellaneous',
        items: [
          { text: 'Performance', link: '/performance' },
          { text: 'Serialisation', link: '/serialisation' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/subtalegames/mimir' },
      { icon: 'x', link: 'https://x.com/subtalegames' },
      { icon: 'instagram', link: 'https://instagram.com/subtalegames' },
      { icon: 'youtube', link: 'https://youtube.com/@subtalegames' },
      { icon: 'discord', link: 'https://discord.subtale.com' },
    ]
  }
})
