<template>
  <UiNavigation />
  <div class="container space-y-12 py-12">
    <slot />
    <UiFooter />
  </div>
</template>

<script setup lang="ts">
const appConfig = useAppConfig()
const route = useRoute()

if (!route.path.startsWith('/browse/')) {
  defineOgImageComponent('Large')

  useSeoMeta({
    ogImage: () =>
      `${appConfig.url}/__og-image__/${import.meta.prerender ? 'static' : 'image'}/${route.path.substring(1)}/og.png`,
    ogType: 'article',
  })
}
</script>
