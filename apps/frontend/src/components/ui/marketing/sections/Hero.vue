<template>
  <div
    :class="props?.class"
    class="grid grid-cols-2 grid-rows-2 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
  >
    <div class="grid grid-rows-2">
      <div class="grid h-full border-b border-e border-neutral-700 p-16">
        <div class="space-y-3 self-center">
          <Icon name="memory:anvil" mode="svg" :size="48" />
          <h1>A solid platform for Pterodactyl® mods</h1>
          <span class="opacity-75">
            Pterodactyl modifications, addons and themes have suffered for ages.
            Blueprint brings change, allowing Pterodactyl and it's modding
            industry to reach new heights.
          </span>
        </div>
      </div>
      <div class="grid h-full border-b border-e border-neutral-700 p-16">
        <div class="space-y-3 self-center">
          <Icon name="memory:heart" mode="svg" :size="48" />
          <h1>Engineered for people, not profit</h1>
          <span class="opacity-75">
            Blueprint is fisically sponsored by a 501(c)(3) nonprofit
            foundation. We're funded by donations, corporate sponsors and
            affiliate revenue.
          </span>
        </div>
      </div>
    </div>
    <div class="border-b border-neutral-700">
      <NuxtImg
        src="/img/pterodactyl.jpeg"
        :width="1280"
        :height="1024"
        class="aspect-video h-full w-full object-cover"
      />
    </div>
    <div class="border-e border-neutral-700">
      <video
        width="1280"
        height="1024"
        autoplay
        muted
        loop
        class="h-full w-full object-cover"
      >
        <source src="/img/console.mp4" type="video/mp4" />
        <source src="/img/console.webm" type="video/webm" />
      </video>
    </div>
    <div class="grid grid-rows-2">
      <div class="grid h-full border-b border-neutral-700 p-16">
        <div class="space-y-3 self-center">
          <Icon name="memory:terminal" mode="svg" :size="48" />
          <h1>Powerful CLI utility</h1>
          <span class="opacity-75">
            Installs, removes and updates all of your extensions within minutes.
            Save yourself from weekends of debugging, sleepless nights and
            unreliable install scripts.
          </span>
        </div>
      </div>
      <div class="grid h-full p-16">
        <div class="space-y-3 self-center">
          <Icon name="memory:pickaxe" mode="svg" :size="48" />
          <h1>Tools developers know and love</h1>
          <span class="opacity-75">
            We're constantly building tools to improve your developer
            experience. Making Pterodactyl add-ons and themes is easier with
            Blueprint.
          </span>
        </div>
      </div>
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
