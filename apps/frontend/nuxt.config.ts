import tailwindcss from '@tailwindcss/vite'

export default defineNuxtConfig({
  compatibilityDate: '2025-07-20',
  srcDir: 'src/',
  devtools: { enabled: true },
  modules: [
    '@nuxt/icon',
    '@nuxt/fonts',
    '@nuxt/image',
    'nuxt-marquee',
    '@nuxt/content',
    '@nuxtjs/sitemap',
    '@nuxtjs/robots',
  ],
  css: ['~/assets/css/main.css'],
  app: {
    head: {
      title: 'Blueprint',
      htmlAttrs: {
        lang: 'en',
      },
    },
  },
  icon: {
    localApiEndpoint: '/__nuxt_icon',
  },
  imports: {
    dirs: ['types/**/*.ts', 'types/**/*.d.ts'],
  },
  vite: {
    plugins: [tailwindcss()],
    server: {
      proxy: {
        '/api': 'localhost:8000',
      },
    },
  },
  content: {
    build: {
      markdown: {
        highlight: {
          theme: 'github-dark',
        },
      },
    },
  },
  site: {
    url: 'https://blueprint.zip',
    name: 'Blueprint',
  },
  nitro: {
    routeRules: {
      '/api/**': {
        prerender: false,
        headers: { 'cache-control': 'no-cache' },
      },
      '/browse/**': {
        prerender: false,
      },
    },
  },
})
