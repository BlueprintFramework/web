<!-- Based on the default layout, specifically for pages with no vertical padding -->

<template>
  <NuxtLoadingIndicator color="#52A9FF" :height="1" />
  <div class="flex min-h-screen flex-col">
    <UiNavigation />
    <slot />
    <UiFooterContainer />
  </div>
</template>

<script setup lang="ts">
defineOgImage('Large.satori')

useSeoMeta({
  ogTitle: 'Blueprint',
  ogDescription:
    'The industry-leading modding platform for the Pterodactyl panel',
})
</script>

<style scoped>
.nuxt-loading-indicator {
  top: var(--nav-offset) !important;
}
</style>
