<template>
  <UiNavigation />
  <div class="container space-y-12 py-12">
    <slot />
    <UiFooter />
  </div>
</template>

<script setup lang="ts">
const route = useRoute()
const ogImagePath = useOgImagePath()

if (!/^\/browse\/.+$/.test(route.path)) {
  defineOgImageComponent('Large')
  useSeoMeta({
    ogImage: ogImagePath,
    ogImageUrl: ogImagePath,
    twitterCard: 'summary_large_image',
    twitterImage: ogImagePath,
  })
}
</script>
