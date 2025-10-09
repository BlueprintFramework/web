<template>
  <h1>Extensions</h1>

  <div
    v-if="data"
    class="divide-y divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700"
  >
    <ElementsExtensionList
      v-for="extension in data.extensions.data"
      :key="extension.id"
      :extension="extension"
    />
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
