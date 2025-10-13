<template>
  <template v-if="data?.extension">
    <div class="flex items-center gap-2">
      <span class="h1"> {{ data.extension.name }} </span>
      <ElementsTextbadge
        v-if="user?.admin && user?.id != data.extension.author.id"
        :label="`${data.extension.author.name}`"
        icon="memory:account"
      />
      <ElementsTextbadge :label="`${data.extension.id}`" icon="memory:pound" />
    </div>
  </template>

  {{ data }}
</template>

<script setup lang="ts">
const route = useRoute()
const { user } = useAuth()

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
