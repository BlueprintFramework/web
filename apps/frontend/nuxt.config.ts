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
    '@nuxtjs/mdc',
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
        '/api': 'http://localhost:8000',
      },
    },
    optimizeDeps: {
      include: ['debug'],
    },
  },
  components: {
    dirs: [
      {
        path: '~/components/prose',
        global: true,
      },
      '~/components',
    ],
  },
  content: {
    build: {
      markdown: {
        highlight: {
          theme: 'github-dark',
          langs: [
            'json',
            'js',
            'ts',
            'html',
            'css',
            'vue',
            'shell',
            'md',
            'yaml',
            'diff',
            'php',
          ],
        },
      },
    },
  },
  mdc: {
    components: {
      prose: true,
      map: {
        script: 'ProseDisabled',
        style: 'ProseDisabled',
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
