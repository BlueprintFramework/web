<template>
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
  ],
})

defineOgImageComponent('Large')
</script>
