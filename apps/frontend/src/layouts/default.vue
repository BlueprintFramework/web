<template>
  <UiNavigation />
  <div class="container space-y-12 py-12">
    <slot />
    <UiFooter />
  </div>
</template>

<script setup lang="ts">
const route = useRoute()

if (!route.path.startsWith('/browse/')) {
  defineOgImageComponent('Large')
}
</script>
