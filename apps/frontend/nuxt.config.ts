// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  srcDir: 'src/',
  devtools: { enabled: true },
  modules: ['@nuxt/image', '@nuxt/icon', '@nuxt/fonts', '@nuxtjs/color-mode'],
  css: [
    'halfmoon/css/halfmoon.min.css',
    'halfmoon/css/cores/halfmoon.modern.css',

    '~/assets/css/utilities.css',
  ],

  app: {
    head: {
      title: 'Blueprint',
      htmlAttrs: {
        lang: 'en',
        'data-bs-theme': 'system',
        'data-bs-core': 'modern',
      },
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1, maximum-scale=1',
    },
  },
})
