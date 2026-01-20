<template>
  <NuxtLoadingIndicator color="#52A9FF" :height="1" />
  <UiNavigation />
  <div class="container max-w-4xl px-0">
    <div
      class="top-(--nav-offset) fixed h-[calc(100%-var(--nav-offset))] max-w-4xl"
    >
      <div
        class="h-full overflow-scroll border-neutral-700 bg-neutral-950 py-4 md:border-x md:py-8"
      >
        <div class="space-y-8 px-4 md:px-8">
          <slot />
        </div>
      </div>
    </div>
  </div>

  <div
    class="top-(--nav-offset) bg-size-[30px_30px] absolute inset-0 -z-10 min-h-[calc(100%-var(--nav-offset))] w-full bg-[linear-gradient(to_right,var(--color-neutral-800)_1px,transparent_1px),linear-gradient(to_bottom,var(--color-neutral-800)_1px,transparent_1px)]"
    style="background-position-x: -5px"
  >
    <div
      class="bg-linear-to-r h-full w-full from-transparent via-neutral-950 to-transparent"
    />
  </div>
</template>

<script setup lang="ts">
defineOgImageComponent('Large')
</script>

<style scoped>
.nuxt-loading-indicator {
  top: var(--nav-offset) !important;
}
</style>
