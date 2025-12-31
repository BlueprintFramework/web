<template>
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2.5">
      <BrandEmblem class="text-brand-50" />
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
    <span class="hidden text-lg lg:inline">
      The modding platform for Pterodactyl
    </span>
  </div>
</template>
