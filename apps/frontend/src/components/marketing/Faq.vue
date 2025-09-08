<template>
  <div
    :class="props.class"
    class="grid grid-cols-1 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 md:grid-cols-2"
  >
    <div class="border-neutral-700 md:border-e">
      <UiAccordion :items="items" :default="0" />
    </div>
    <div class="relative hidden overflow-hidden md:block">
      <NuxtImg
        src="/img/gradient.jpeg"
        :width="1280"
        :height="450"
        class="h-full w-full object-cover"
      />
      <Icon
        name="memory:tooltip-above-help"
        mode="svg"
        :size="96"
        class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 transform"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { NuxtLink } from '#components'

const props = defineProps({
  class: {
    type: String,
    required: false,
  },
})

const items = [
  {
    title: "What's Blueprint?",
    text: 'Blueprint is the modding platform for Pterodactyl. We provide developers a head start with industry-leading developer tools and system administrators an easier time managing extensions.',
  },
  {
    title: 'Where can I find extensions?',
    inner: () => {
      return [
        h('span', {}, 'You can '),
        h(
          NuxtLink,
          {
            to: '/browse',
            class: 'text-link',
          },
          () => 'discover extensions verified by us'
        ),
        h(
          'span',
          {},
          ' on our site. Extensions not listed may not meet our compatibility and safety standards.'
        ),
      ]
    },
  },
  {
    title: 'Does Blueprint work with Pterodactyl forks?',
    text: 'No. At this time Blueprint does not work with Pterodactyl alternatives, including forks.',
  },
  {
    title: 'Is Blueprint a fork of Pterodactyl?',
    text: "No. Blueprint cannot function on it's own, just like any other 'standalone' Pterodactyl modification.",
  },
  {
    title: 'Are standalone modifications compatible?',
    inner: () => {
      return [
        h('span', {}, 'No. '),
        h(
          NuxtLink,
          {
            to: '/browse',
            class: 'text-link',
          },
          () => 'Find a Blueprint-compatible alternative'
        ),
        h(
          'span',
          {},
          " or ask the modification's developer to port it over. Select standalone modifications may be compatible with Blueprint, provided that their developer actively maintains a Blueprint fork."
        ),
      ]
    },
  },
  {
    title: 'Is Blueprint open-source?',
    inner: () => {
      return [
        h(
          'span',
          {},
          'Yeah! Our source code is largely under a MIT license and available on '
        ),
        h(
          NuxtLink,
          {
            to: 'https://github.com/blueprintframework',
            class: 'text-link',
            target: '_blank',
          },
          () => 'our GitHub organization.'
        ),
      ]
    },
  },
]
</script>
