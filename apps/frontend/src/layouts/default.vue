<template>
  <UiNavigation />
  <div class="container space-y-12 py-12">
    <slot />
    <UiFooter />
  </div>
</template>

<script setup lang="ts">
defineOgImageComponent('Large')

useSeoMeta({
  ogTitle: 'Blueprint',
  ogDescription:
    'The industry-leading modding platform for the Pterodactyl panel',
})
</script>
