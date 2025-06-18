<template>
  <div
    :class="props.class"
    class="grid overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-4"
  >
    <NuxtLink
      to="https://github.com/blueprintframework/framework"
      class="hover:text-brand-50 group space-y-2 border-b border-neutral-700 p-4 transition-colors hover:bg-neutral-900 md:border-e lg:border-b-0"
      target="_blank"
    >
      <h2>
        <div class="flex">
          blueprintframework/framework
          <Icon
            name="memory:chevron-right"
            :size="24"
            class="opacity-0 transition-opacity group-hover:opacity-100"
          />
        </div>
      </h2>
      <p class="text-default-font">
        The underlying extension framework, CLI, developer tools and more.
      </p>
    </NuxtLink>
    <NuxtLink
      to="https://github.com/blueprintframework/web"
      class="hover:text-brand-50 group space-y-2 border-b border-neutral-700 p-4 transition-colors hover:bg-neutral-900 lg:border-b-0 lg:border-e"
      target="_blank"
    >
      <h2>
        <div class="flex">
          blueprintframework/web
          <Icon
            name="memory:chevron-right"
            :size="24"
            class="opacity-0 transition-opacity group-hover:opacity-100"
          />
        </div>
      </h2>
      <p class="text-default-font">
        Where everything web happens. API and website.
      </p>
    </NuxtLink>
    <NuxtLink
      to="https://github.com/blueprintframework/docker"
      class="hover:text-brand-50 group space-y-2 border-b border-neutral-700 p-4 transition-colors hover:bg-neutral-900 md:border-b-0 md:border-e"
      target="_blank"
    >
      <h2>
        <div class="flex">
          blueprintframework/docker
          <Icon
            name="memory:chevron-right"
            :size="24"
            class="opacity-0 transition-opacity group-hover:opacity-100"
          />
        </div>
      </h2>
      <p class="text-default-font">
        Blueprint's extension ecosystem you know and love, in Docker.
      </p>
    </NuxtLink>
    <NuxtLink
      to="https://github.com/blueprintframework/templates"
      class="hover:text-brand-50 group space-y-2 p-4 transition-colors hover:bg-neutral-900"
      target="_blank"
    >
      <h2>
        <div class="flex">
          blueprintframework/templates
          <Icon
            name="memory:chevron-right"
            :size="24"
            class="opacity-0 transition-opacity group-hover:opacity-100"
          />
        </div>
      </h2>
      <p class="text-default-font">
        Starting points for extension developers, integrated into our CLI.
      </p>
    </NuxtLink>
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
