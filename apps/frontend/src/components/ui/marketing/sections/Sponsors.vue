<template>
  <div
    :class="props.class"
    class="bg-neutral-950` overflow-hidden rounded-3xl border border-neutral-700"
  >
    <div class="border-b border-neutral-700">
      <div
        class="grid items-center divide-y divide-neutral-700 lg:grid-cols-2 lg:divide-x lg:divide-y-0"
      >
        <div class="flex h-full items-center gap-2 p-4">
          <Icon name="memory:heart" mode="svg" :size="32" />
          <h1>Sponsors</h1>
        </div>
        <p class="p-4">
          <span class="opacity-75">
            We develop, maintain and create open-source software used by a wide
            range of users.
          </span>
          Support our future by donating to Blueprint.
        </p>
      </div>
    </div>
    <div
      class="flex min-h-44 items-center bg-[linear-gradient(to_right,var(--color-neutral-800)_1px,transparent_1px),linear-gradient(to_bottom,var(--color-neutral-800)_1px,transparent_1px)] bg-[size:40px_40px] bg-[position:-8px_-8px]"
    >
      <div class="flex w-full justify-center gap-12 p-4">
        <div>
          <h2>Sponsor 1</h2>
        </div>
        <div>
          <h2>Sponsor 2</h2>
        </div>
        <div>
          <h2>Sponsor 3</h2>
        </div>
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
