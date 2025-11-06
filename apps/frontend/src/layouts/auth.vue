<template>
  <NuxtLoadingIndicator color="#52A9FF" :height="1" />
  <UiNavigation />
  <div class="container">
    <div
      class="grid h-[calc(100vh_-_var(--nav-offset))] grid-cols-1 divide-neutral-700 overflow-hidden border-x border-neutral-700 bg-neutral-950 lg:grid-cols-2 lg:divide-x"
    >
      <div
        class="relative flex h-full items-center bg-[linear-gradient(to_right,var(--color-neutral-800)_1px,transparent_1px),linear-gradient(to_bottom,var(--color-neutral-800)_1px,transparent_1px)] bg-[size:30px_30px] bg-[position:-15px_-15px]"
      >
        <div
          class="absolute inset-0 z-0 bg-gradient-to-b from-transparent via-neutral-950 to-transparent"
        />
        <div class="relative z-10 w-full bg-neutral-950">
          <slot />
        </div>
      </div>
      <div class="hidden bg-[black] lg:block">
        <NuxtImg
          src="/img/auth.jpeg"
          class="h-[calc(100vh_-_var(--nav-offset))] w-full object-cover"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.nuxt-loading-indicator {
  top: var(--nav-offset) !important;
}
</style>
