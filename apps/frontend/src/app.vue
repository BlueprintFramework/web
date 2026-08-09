<template>
  <NuxtRouteAnnouncer />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
  <UiUpsellSurvey />
</template>

<script setup lang="ts">
useHead({
  meta: [
    {
      name: 'color-scheme',
      content: 'dark',
    },
    {
      name: 'theme-color',
      content: '#52A9FF',
    },
  ],
})
</script>
