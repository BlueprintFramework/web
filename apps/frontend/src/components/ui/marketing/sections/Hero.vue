<template>
  <div
    :class="props?.class"
    class="grid grid-cols-2 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
  >
    <div class="grid grid-rows-2">
      <div class="space-y-3 self-center border-b border-neutral-700 p-16">
        <Icon name="memory:anvil" mode="svg" :size="48" />
        <h1>A solid platform for Pterodactyl® mods</h1>
        <span class="opacity-75">
          Pterodactyl modifications, addons and themes have suffered for ages.
          Blueprint brings change, allowing Pterodactyl and it's modding
          industry to reach new heights.
        </span>
      </div>
      <div class="space-y-3 self-center p-16">
        <Icon name="memory:heart" mode="svg" :size="48" />
        <h1>Engineered for people, not profit</h1>
        <span class="opacity-75">
          Blueprint is fisically sponsored by a 501(c)(3) nonprofit foundation.
          We're funded by donations, corporate sponsors and affiliate revenue.
        </span>
      </div>
    </div>
    <div class="border-s border-neutral-700">
      <NuxtImg
        src="/img/pterodactyl.jpeg"
        :width="1280"
        :height="1024"
        class="aspect-video h-full w-full object-cover"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  class: {
    type: String,
    required: false,
  },
})
</script>
