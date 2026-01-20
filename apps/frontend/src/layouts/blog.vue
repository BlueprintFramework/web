<template>
  <NuxtLoadingIndicator color="#52A9FF" :height="1" />
  <div class="fixed w-full">
    <div class="max-w-200 container px-0 md:px-4">
      <div
        class="flex items-center justify-between gap-1.5 border-b border-t-0 border-neutral-700 bg-neutral-950 p-3 md:border-x"
      >
        <BrandWordmark />
        <div>
          <NuxtLink href="/blog" tabindex="-1">
            <ElementsButtonSmall>Posts</ElementsButtonSmall>
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>

  <div class="max-w-200 container px-0 md:px-4">
    <div class="bg-neutral-950">
      <div
        class="pt-(--nav-offset) bg-stripes min-h-screen border-neutral-700 bg-neutral-950 md:border-x"
      >
        <div class="bg-neutral-950">
          <slot />
        </div>
        <div class="h-5" />
      </div>
      <div
        class="border-b-0 border-t border-neutral-700 bg-neutral-950 p-4 md:border-x"
      >
        <div class="flex flex-col items-center">
          <div class="flex items-center gap-1">
            <Icon name="memory:alpha-c" />
            <span>
              <span>2023-2026 Emma (</span>
              <NuxtLink
                to="https://prpl.wtf"
                class="hover:text-brand-50 default underline transition-colors"
                >prpl.wtf</NuxtLink
              >
              <span>) and contributors</span>
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div
    class="bg-size-[30px_30px] fixed inset-0 left-0 top-0 -z-10 h-full w-full bg-[linear-gradient(to_right,var(--color-neutral-800)_1px,transparent_1px),linear-gradient(to_bottom,var(--color-neutral-800)_1px,transparent_1px)]"
  >
    <div
      class="bg-linear-to-r hidden h-full w-full from-transparent via-neutral-950 to-transparent lg:block"
    />
    <div class="h-full w-full bg-neutral-950 lg:hidden" />
  </div>
</template>

<script setup lang="ts">
useSeoMeta({
  title: 'Blueprint Blog',
  description:
    'Articles from the industry-leading modding platform for Pterodactyl',
  ogType: 'article',
  ogTitle: 'Blueprint Blog',
  ogDescription:
    'Articles from the industry-leading modding platform for Pterodactyl',
})

defineOgImageComponent('Large')
</script>

<style>
.nuxt-loading-indicator {
  top: 0px !important;
}

:root {
  --nav-offset: 53px;
}
</style>
