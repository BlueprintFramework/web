<template>
  <div v-if="publicUser" class="space-y-12">
    {{ publicUser }}
    <div class="rounded-3xl border border-neutral-700">hello</div>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()
const { user } = useAuth()

const { data: publicUser } = await useAsyncData<PublicUser | null>(
  `publicuser-${route.params.id}`,
  () => $fetch(`/api/users/${route.params.id}`),
  {
    server: false,
    default: () => null,
  }
)

useSeoMeta({
  title: () => publicUser.value?.user.name,
  description: () =>
    `Browse extensions created by ${publicUser.value?.user.name} directly on Blueprint.`,
})
</script>
