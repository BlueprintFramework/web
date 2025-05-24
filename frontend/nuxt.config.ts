// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  devtools: { enabled: true },
  modules: ['@nuxt/image', '@nuxt/icon', '@nuxt/fonts'],
  css: [
    'halfmoon/css/halfmoon.min.css'
  ],
  app: {
    head: {
      title: 'Blueprint',
      htmlAttrs: {
        lang: 'en',
        'data-bs-theme': 'dark'
      },
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1, maximum-scale=1',
    }
  }
})