<template>
  <div
    :class="props.class"
    class="group overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 transition-colors focus-within:border-neutral-500"
  >
    <div
      class="border-b border-neutral-700 transition-colors group-focus-within:border-neutral-500"
    >
      <div
        class="grid items-center divide-y divide-neutral-700 group-focus-within:divide-neutral-500 lg:grid-cols-2 lg:divide-x lg:divide-y-0"
      >
        <div class="flex h-full items-center gap-2 p-4 transition-colors">
          <Icon name="memory:heart" mode="svg" :size="32" />
          <h1>Sponsors</h1>
        </div>
        <p class="p-4 transition-colors">
          <span class="opacity-75">
            We develop, maintain and create open-source software used by a wide
            range of users.
          </span>
          Support our work by
          <a
            class="text-link"
            href="https://hcb.hackclub.com/donations/start/blueprint"
            target="_blank"
            @mousedown.prevent
          >
            donating to Blueprint.
          </a>
        </p>
      </div>
    </div>
    <div
      class="flex items-center bg-[linear-gradient(to_right,var(--color-neutral-800)_1px,transparent_1px),linear-gradient(to_bottom,var(--color-neutral-800)_1px,transparent_1px)] bg-[size:40px_40px] bg-[position:-8px_-8px] md:min-h-44"
    >
      <div
        class="flex w-full flex-col items-start justify-center gap-4 p-4 md:flex-row md:items-center md:gap-12 md:p-6"
      >
        <NuxtLink
          to="https://ultraservers.com"
          target="_blank"
          class="hover:text-brand-50 focus:text-brand-50 focus:outline-brand-50 flex items-center rounded-sm outline-1 outline-offset-8 outline-transparent transition-colors first:gap-2"
          @mousedown.prevent
        >
          <SvgUltraservers size="32" />
          <span class="h2 text-[1.6rem]">UltraServers</span>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  class: {
    class: String,
    required: false,
  },
})
</script>
