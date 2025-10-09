<template>
  <h1>Extensions</h1>

  <div
    v-if="data"
    class="grid grid-cols-1 gap-[1px] divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-700 xl:grid-cols-2"
  >
    <NuxtLink
      v-for="extension in data.extensions.data"
      :to="`/app/extensions/${extension.id}`"
      class="group flex flex-row items-center gap-4 bg-neutral-950 p-4 transition-colors hover:bg-neutral-900"
    >
      <div>
        <div
          class="w-50 aspect-video overflow-hidden rounded-2xl border border-neutral-700"
        >
          <NuxtImg
            :src="extension.banner.lowres"
            class="h-full w-full transition-transform group-hover:scale-105"
          />
        </div>
      </div>
      <div class="space-y-1 overflow-hidden">
        <p
          class="!text-default-font group-hover:!text-brand-50 truncate text-nowrap text-xl font-bold transition-colors"
        >
          {{ extension.name }}
        </p>
        <p class="!text-default-font truncate text-nowrap">
          {{ extension.summary }}
        </p>
      </div>
    </NuxtLink>

    <div class="min-h-5 w-[calc(400%+3px)] bg-neutral-950">
      <div class="bg-stripes h-full w-full" />
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'user-verified',
  layout: 'dashboard',
})

const { data } = await useAsyncData<UserExtensions>(
  `user-extensions`,
  () => $fetch(`/api/user/extensions`),
  {
    server: false,
  }
)
</script>
