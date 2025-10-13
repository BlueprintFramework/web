<template>
  <template v-if="data?.extension">
    <h1>{{ data.extension.name }}</h1>
  </template>

  {{ data }}
</template>

<script setup lang="ts">
const route = useRoute()

definePageMeta({
  middleware: 'user-verified',
  layout: 'dashboard',
})

const { data } = await useAsyncData<{ extension: FullExtension }>(
  `user-extension-${route.params.id}`,
  () => $fetch(`/api/user/extensions/${route.params.id}`),
  {
    server: false,
  }
)
</script>
