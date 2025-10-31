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
    '@nuxtjs/mdc',
    '@nuxtjs/plausible',
    '@nuxtjs/turnstile',
    '@vueuse/nuxt',
    'nuxt-og-image',
    'nuxt-site-config',
    '@nuxt/scripts',
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
  sitemap: {
    sitemapsPathPrefix: '/',
  },
  ogImage: {
    fonts: [
      'Roboto:400',
      'Roboto:700',
      'Funnel+Display:400',
      'Funnel+Display:700',
    ],
    zeroRuntime: true,
  },
  icon: {
    localApiEndpoint: '/__nuxt_icon',
  },
  imports: {
    dirs: ['types/**/*.ts', 'types/**/*.d.ts'],
  },
  vite: {
    plugins: [tailwindcss()],
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
            'tsx',
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
    },
  },
  site: {
    url: 'https://blueprint.zip',
    name: 'Blueprint',
  },
  nitro: {
    devProxy: {
      '/api': 'http://localhost:8000/api',
      '/browse/sitemap.xml': 'http://localhost:8000/browse/sitemap.xml',
      '/yay': 'https://blueprint.zip/yay',
    },
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
  routeRules: {
    '/__og-image__/image/**': {
      proxy: { to: '/__og-image__/static/**' },
    },
  },
  plausible: {
    apiHost: 'https://blueprint.zip/yay/script.js',
    domain: 'blueprint.zip',
    autoOutboundTracking: true,
    ignoredHostnames: ['localhost'],
  },
  turnstile: {
    siteKey: import.meta.dev
      ? '1x00000000000000000000AA'
      : '0x4AAAAAAB7bNfQex8uoMyq6',
  },

  hooks: {
    'nitro:build:public-assets': async (nitro) => {
      if (nitro.options.preset === 'static') {
        const { promises: fs } = await import('fs')
        const { join } = await import('path')

        const publicDir = nitro.options.output.publicDir
        const srcPath = join(publicDir, '__og-image__', 'static')
        const destPath = join(publicDir, '__og-image__', 'image')

        try {
          await fs.cp(srcPath, destPath, { recursive: true })
        } catch (err) {
          console.warn('og-image copy failed:', err)
        }
      }
    },
  },
})
