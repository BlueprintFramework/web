<template>
  <div
    :class="props?.class"
    class="grid grid-cols-2 rounded-3xl border border-neutral-700 bg-neutral-950"
  >
    <div class="space-y-3 self-center p-16">
      <Icon name="memory:anvil" mode="svg" :size="48" />
      <h1>A solid platform for Pterodactyl® mods</h1>
      <span class="opacity-75">
        Pterodactyl modifications, addons and themes have suffered for ages.
        Blueprint brings change, allowing Pterodactyl and it's modding industry
        to reach new heights.
      </span>
    </div>
    <div class="border-s border-neutral-700">
      <video
        width="1280"
        height="720"
        class="rounded-e-3xl"
        autoplay
        muted
        loop
      >
        <source src="/img/pterodactyl.mp4" type="video/mp4" />
      </video>
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
