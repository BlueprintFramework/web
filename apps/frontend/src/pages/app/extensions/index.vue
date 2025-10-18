<template>
  <div class="flex items-center justify-between">
    <h1>Extensions</h1>
    <div v-if="user?.admin">
      <ElementsButton @click="adminToggle" :disabled="loading">
        {{
          viewOthers.enabled ? 'Show my extensions' : "Show others' extensions"
        }}
      </ElementsButton>
    </div>
  </div>

  <div
    v-if="viewOthers.enabled"
    class="flex items-center divide-x divide-neutral-700 overflow-hidden rounded-2xl border border-neutral-700"
  >
    <div class="w-full">
      <ElementsButton
        class="w-full rounded-none border-none"
        :active="viewOthers.show == 'unspecified'"
        @click="adminCategory('unspecified')"
      >
        All
      </ElementsButton>
    </div>
    <div class="w-full">
      <ElementsButton
        class="w-full rounded-none border-none"
        :active="viewOthers.show == 'ready'"
        @click="adminCategory('ready')"
      >
        Ready
      </ElementsButton>
    </div>
    <div class="w-full">
      <ElementsButton
        class="w-full rounded-none border-none"
        :active="viewOthers.show == 'pending'"
        @click="adminCategory('pending')"
      >
        Pending
      </ElementsButton>
    </div>
    <div class="w-full">
      <ElementsButton
        class="w-full rounded-none border-none"
        :active="viewOthers.show == 'denied'"
        @click="adminCategory('denied')"
      >
        Rejected
      </ElementsButton>
    </div>
  </div>
  <UiAppExtensionsCreatenew v-else />

  <div
    v-if="data && data.extensions.total > 0"
    class="grid grid-cols-1 gap-[1px] divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-700 xl:grid-cols-2"
  >
    <NuxtLink
      v-for="extension in data.extensions.data"
      :to="`/app/extensions/${extension.id}`"
      class="group flex items-center justify-between gap-4 bg-neutral-950 p-4 transition-colors hover:bg-neutral-900"
    >
      <div
        class="flex w-full flex-col gap-4 overflow-hidden md:w-auto md:flex-row md:items-center"
      >
        <div class="w-full md:w-auto">
          <div
            class="md:w-50 aspect-video w-full overflow-hidden rounded-2xl border border-neutral-700"
          >
            <NuxtImg
              :src="extension.banner.lowres"
              class="h-full w-full transition-transform group-hover:scale-105"
            />
          </div>
        </div>
        <div class="space-y-1 overflow-hidden">
          <p
            class="!text-default-font/60 mb-1.5 truncate text-nowrap text-sm"
            v-if="viewOthers.enabled"
          >
            {{ extension.author.name }}
          </p>
          <p
            class="!text-default-font group-hover:!text-brand-50 truncate text-nowrap text-xl font-bold transition-colors"
          >
            {{ extension.name }}
          </p>
          <p class="!text-default-font truncate text-nowrap">
            {{ extension.summary }}
          </p>
        </div>
      </div>
      <Icon
        name="memory:chevron-right"
        :size="24"
        class="text-brand-50 !hidden opacity-0 transition-opacity group-hover:opacity-100 md:!block"
      />
    </NuxtLink>

    <div class="min-h-5 w-[calc(400%+3px)] bg-neutral-950">
      <div class="bg-stripes h-full w-full" />
    </div>
  </div>
  <div
    v-else-if="data?.extensions.total == 0"
    class="flex flex-col items-center rounded-3xl border border-neutral-700 p-12"
  >
    <div class="max-w-100 flex flex-col items-center gap-2">
      <Icon name="memory:cube" :size="32" />
      <span class="text-lg">No extensions found!</span>
      <span class="text-default-font/60"> tldr; you should make some </span>
    </div>
  </div>

  <div
    v-if="hasMore && data"
    ref="loadMoreTrigger"
    class="flex h-20 items-center justify-center"
  >
    <span v-if="loading" class="text-default-font/60">loading more...</span>
  </div>
</template>

<script setup lang="ts">
const { user } = useAuth()

definePageMeta({
  middleware: 'user-verified',
  layout: 'dashboard',
})

const data = ref<UserExtensions>()
const page = ref(1)
const loading = ref(false)
const loadMoreTrigger = ref<HTMLElement | null>(null)
const viewOthers = ref<{
  enabled: boolean
  show: 'unspecified' | 'ready' | 'pending' | 'denied'
}>({
  enabled: false,
  show: 'unspecified',
})

const basePath = computed(() => {
  if (!user.value?.admin || !viewOthers.value.enabled)
    return `/api/user/extensions`

  switch (viewOthers.value.show) {
    case 'unspecified':
      return '/api/user/admin/extensions'
    case 'ready':
      return '/api/user/admin/extensions/ready'
    case 'pending':
      return '/api/user/admin/extensions/pending'
    case 'denied':
      return '/api/user/admin/extensions/denied'
  }
})

const hasMore = computed(() => {
  if (!data.value) return false
  const totalPages = Math.ceil(
    data.value.extensions.total / data.value.extensions.per_page
  )
  return page.value < totalPages
})

const fetchExtensions = async () => {
  if (loading.value || (!hasMore.value && page.value > 1)) return

  loading.value = true
  try {
    const result = await $fetch<UserExtensions>(
      `${basePath.value}?page=${page.value}&per_page=26`,
      {
        method: 'GET',
      }
    )

    if (page.value === 1) {
      data.value = result
    } else if (data.value) {
      data.value.extensions.data.push(...result.extensions.data)
      data.value.extensions.page = result.extensions.page
      data.value.extensions.total = result.extensions.total
      data.value.extensions.per_page = result.extensions.per_page
    }
  } catch (error) {
    console.error('failed to fetch sessions:', error)
  } finally {
    loading.value = false
  }
}

const resetAndFetch = () => {
  page.value = 1
  data.value = undefined
  fetchExtensions()
}

watch(basePath, resetAndFetch)

let observer: IntersectionObserver | null = null

const setupObserver = () => {
  observer?.disconnect()

  if (!loadMoreTrigger.value) return

  observer = new IntersectionObserver(
    (entries) => {
      if (entries[0]?.isIntersecting && hasMore.value && !loading.value) {
        page.value++
      }
    },
    { threshold: 0.5 }
  )

  observer.observe(loadMoreTrigger.value)
}

onMounted(async () => {
  await fetchExtensions()

  watch(loadMoreTrigger, () => {
    nextTick(() => setupObserver())
  })

  onUnmounted(() => observer?.disconnect())
})

watch(page, () => {
  if (page.value > 1) fetchExtensions()
})

const adminToggle = () => {
  viewOthers.value.enabled = !viewOthers.value.enabled
}

const adminCategory = (
  category: 'unspecified' | 'ready' | 'pending' | 'denied'
) => {
  viewOthers.value.show = category
}
</script>
