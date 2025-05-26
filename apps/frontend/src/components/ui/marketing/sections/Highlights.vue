<template>
  <div :class="props?.class">
    <!-- Mobile -->
    <div class="rounded-3xl border border-neutral-700 bg-neutral-950 md:hidden">
      <div class="border-b border-neutral-700 p-4">
        <div class="relative mb-4 flex items-center justify-between">
          <h2>{{ features.first.title }}</h2>
          <Icon :name="features.first.icon" class="text-3xl" mode="svg" />
        </div>
        <span class="body">
          <span class="opacity-75">{{ features.first.caption.prefix }}</span>
          {{ features.first.caption.main }}
        </span>
      </div>
      <div class="border-b border-neutral-700 p-4">
        <div class="relative mb-4 flex items-center justify-between">
          <h2>{{ features.second.title }}</h2>
          <Icon :name="features.second.icon" class="text-3xl" mode="svg" />
        </div>
        <span class="body">
          <span class="opacity-75">{{ features.second.caption.prefix }}</span>
          {{ features.second.caption.main }}
        </span>
      </div>
      <div class="p-4">
        <div class="relative mb-4 flex items-center justify-between">
          <h2>{{ features.third.title }}</h2>
          <Icon :name="features.third.icon" class="text-3xl" mode="svg" />
        </div>
        <span class="body">
          <span class="opacity-75">{{ features.third.caption.prefix }}</span>
          {{ features.third.caption.main }}
          <span class="opacity-75">{{ features.third.caption.suffix }}</span>
        </span>
      </div>
    </div>

    <!-- Desktop -->
    <div
      class="hidden overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 md:block"
    >
      <div class="grid grid-cols-3 divide-x divide-neutral-700">
        <div class="relative flex h-full items-center justify-between p-4">
          <h2>{{ features.first.title }}</h2>
          <Icon :name="features.first.icon" class="text-3xl" mode="svg" />
        </div>
        <div class="relative flex h-full items-center justify-between p-4">
          <h2>{{ features.second.title }}</h2>
          <Icon :name="features.second.icon" class="text-3xl" mode="svg" />
        </div>
        <div class="relative flex h-full items-center justify-between p-4">
          <h2>{{ features.third.title }}</h2>
          <Icon :name="features.third.icon" class="text-3xl" mode="svg" />
        </div>
      </div>
      <div class="border-t border-neutral-700"></div>
      <div class="grid grid-cols-3 divide-x divide-neutral-700">
        <div class="p-4">
          <span class="body">
            <span class="opacity-75">{{ features.first.caption.prefix }}</span>
            {{ features.first.caption.main }}
          </span>
        </div>
        <div class="p-4">
          <span class="body">
            <span class="opacity-75">{{ features.second.caption.prefix }}</span>
            {{ features.second.caption.main }}
          </span>
        </div>
        <div class="p-4">
          <span class="body">
            <span class="opacity-75">{{ features.third.caption.prefix }}</span>
            {{ features.third.caption.main }}
            <span class="opacity-75">{{ features.third.caption.suffix }}</span>
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
const features = {
  first: {
    title: 'Extension ecosystem',
    icon: 'memory:cube',
    caption: {
      prefix:
        'Pterodactyl® themes, plugin installers, player managers, admin tools and much more.',
      main: "There's a Blueprint extension for that.",
    },
  },
  second: {
    title: 'Beginner and developer-friendly',
    icon: 'memory:application-code',
    caption: {
      prefix:
        'Our mission is to make Pterodactyl® modifications easier for everyone.',
      main: 'Thousands of administrators and developers call Blueprint home.',
    },
  },
  third: {
    title: 'Save time',
    icon: 'memory:time-sand',
    caption: {
      prefix:
        "You don't have an infinite supply of time on your hands, do you?",
      main: 'Blueprint saves a huge amount of time',
      suffix: 'for both developers and system administrators.',
    },
  },
}
</script>
