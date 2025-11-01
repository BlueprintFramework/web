<template>
  <NuxtLoadingIndicator color="#52A9FF" :height="1" />
  <NuxtRouteAnnouncer />
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
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
