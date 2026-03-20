import { defineConfig } from 'vitepress'

export default defineConfig({
  lang: 'zh-CN',
  title: 'Browser Sync CLI',
  description: '浏览器收藏夹实时同步工具',
  base: '/browser-sync-cli/',

  themeConfig: {
    logo: '/logo.svg',
    nav: [
      { text: '首页', link: '/' },
      { text: '使用指南', link: '/guide/usage' },
      { text: '开发指南', link: '/guide/development' },
    ],
    sidebar: {
      '/guide/usage': [
        {
          text: '使用指南',
          items: [
            { text: '介绍', link: '/guide/' },
            { text: '安装', link: '/guide/installation' },
            { text: '使用', link: '/guide/usage' },
            { text: '故障排查', link: '/guide/troubleshooting' },
          ],
        },
      ],
      '/guide/development': [
        {
          text: '开发指南',
          items: [
            { text: '介绍', link: '/guide/' },
            { text: '开发', link: '/guide/development' },
          ],
        },
      ],
      '/guide/': [
        {
          text: '指南',
          items: [
            { text: '介绍', link: '/guide/' },
            { text: '安装', link: '/guide/installation' },
            { text: '使用', link: '/guide/usage' },
            { text: '开发', link: '/guide/development' },
            { text: '故障排查', link: '/guide/troubleshooting' },
          ],
        },
      ],
    },
    footer: {
      message: 'MIT Licensed',
      copyright: 'Copyright © 2024'
    }
  }
})