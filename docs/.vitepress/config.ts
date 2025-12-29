import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Rusheet',
  description: 'High-performance spreadsheet engine powered by Rust + WASM',
  base: '/rusheet/',
  themeConfig: {
    nav: [
      { text: 'Guide', link: '/guide/' },
      { text: 'API', link: '/api/' },
      { text: 'GitHub', link: 'https://github.com/chrischeng-c4/rusheet' }
    ],
    sidebar: {
      '/guide/': [
        { text: 'Introduction', link: '/guide/' },
        { text: 'Getting Started', link: '/guide/getting-started' },
        { text: 'Architecture', link: '/guide/architecture' }
      ],
      '/api/': [
        { text: 'Overview', link: '/api/' },
        { text: 'Core API', link: '/api/core' },
        { text: 'Events', link: '/api/events' },
        { text: 'Types', link: '/api/types' }
      ]
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/chrischeng-c4/rusheet' }
    ]
  }
})
