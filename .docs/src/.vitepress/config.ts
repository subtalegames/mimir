import { defineConfig } from 'vitepress'

export default defineConfig({
    title: 'Mímir',
    description: 'Contextual query engine for dynamic video games',
    lastUpdated: true,
    appearance: 'dark',
    themeConfig: {
        siteTitle: 'Mímir',
        nav: [
            { text: 'Guide', link: '/guide/inspiration' },
            { text: 'Changelog', link: '/changelog' },
        ],
        socialLinks: [
            { icon: 'github', link: 'https://github.com/subtalegames/mimir' },
        ],
        footer: {
            message: 'Released under the MIT and Apache-2.0 licenses.',
            copyright: 'Copyright © 2022-present Subtale',
        },
        sidebar: [
            {
                text: 'Introduction',
                collapsible: true,
                items: [
                    {
                        text: 'Inspiration',
                        link: '/guide/inspiration',
                    },
                    {
                        text: 'High-level overview',
                        link: '/guide/overview',
                    },
                    {
                        text: 'Why Mímir?',
                        link: '/guide/naming',
                    },
                ],
            },
            {
                text: 'Concepts',
                collapsible: true,
                items: [
                    {
                        text: 'Evaluator',
                        link: '/concepts/evaluator',
                    },
                    {
                        text: 'Query',
                        link: '/concepts/query',
                    },
                    {
                        text: 'Rule',
                        link: '/concepts/rule',
                    },
                    {
                        text: 'Ruleset',
                        link: '/concepts/ruleset',
                    },
                ],
            },
            {
                text: 'Miscellaneous',
                collapsible: true,
                items: [
                    {
                        text: 'Serialization',
                        link: '/guide/serialization',
                    },
                    {
                        text: 'Performance',
                        link: '/guide/performance',
                    },
                ],
            },
            {
                text: 'Use cases',
                collapsible: true,
                items: [
                    {
                        text: 'Loading screen tips',
                        link: '/use-cases/tips',
                    },
                ],
            },
        ],
    },
})