<template>
  <div class="flex items-center justify-between">
    <h1>Extensions? We've got 'em.</h1>
    <pre class="text-2xl font-bold">[* *  ]</pre>
  </div>
  <div class="flex flex-row gap-4">
    <!-- Desktop filters -->
    <div class="w-75 hidden lg:block">
      <div
        class="sticky top-[calc(var(--nav-offset)+1rem)] overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
      >
        <div class="border-b border-neutral-700 p-4">
          <UiFormInput
            v-model="form.search"
            name="search"
            type="text"
            :rules="[]"
            leading-icon="memory:search"
            placeholder="Search.."
            @validate="void"
          />
        </div>
        <div class="divide-y divide-neutral-700">
          <div class="space-y-4 p-4">
            <UiBrowseFilters :form="form" />
          </div>
        </div>
      </div>
    </div>

    <div class="w-full lg:w-[calc(100%-18.75rem)]">
      <div
        class="grid grid-cols-1 gap-px overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-700 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4"
      >
        <UiExtensioncard
          v-if="pending || extensions == undefined"
          v-for="n in 28"
          class="flex flex-col bg-neutral-950"
        />
        <UiExtensioncard
          v-else
          v-for="extension in filteredAndSortedExtensions"
          :key="extension.id"
          :extension="extension"
          class="flex flex-col bg-neutral-950"
        />
        <div
          v-if="!pending && filteredAndSortedExtensions.length === 0"
          class="col-span-full flex flex-col items-center justify-center bg-neutral-950 p-12 text-center"
        >
          <Icon
            name="memory:search"
            :size="48"
            class="text-default-font/30 mb-4"
          />
          <h3 class="mb-2 text-xl font-semibold">No extensions found</h3>
          <p class="text-default-font/60">
            {{
              !form.showExtensions && !form.showThemes
                ? 'Enable at least one filter to see results'
                : 'Try adjusting your search or filters'
            }}
          </p>
        </div>
        <div class="min-h-5 w-[calc(400%+3px)] bg-neutral-950">
          <div class="bg-stripes h-full w-full" />
        </div>
      </div>
    </div>
  </div>

  <!-- Mobile filters -->
  <div
    class="z-15 fixed bottom-0 left-0 m-0 w-full lg:hidden"
    ref="drawerContainer"
  >
    <div class="container">
      <div
        class="overflow-hidden rounded-t-3xl border border-b-0 border-neutral-700 bg-neutral-950"
      >
        <div class="flex divide-x divide-neutral-700">
          <div class="w-full p-4">
            <UiFormInput
              v-model="form.search"
              name="search"
              type="text"
              :rules="[]"
              leading-icon="memory:search"
              placeholder="Search.."
              @validate="void"
            />
          </div>
          <button
            class="p-4 transition-colors hover:bg-neutral-900"
            @click="toggleDrawer"
          >
            <Icon
              name="memory:chevron-up"
              mode="svg"
              :size="28"
              class="transition-transform duration-300"
              :class="{ 'rotate-180': drawer.open }"
            />
          </button>
        </div>

        <div
          ref="drawerContent"
          class="overflow-hidden duration-300"
          :style="{ height: drawer.height }"
        >
          <div class="space-y-4 border-t border-neutral-700 p-4">
            <UiBrowseFilters :form="form" />
          </div>
        </div>
      </div>
    </div>
  </div>

  <Transition
    enter-active-class="transition-opacity duration-300"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition-opacity duration-200"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div
      class="fixed left-0 top-0 z-10 h-full w-full bg-neutral-950/50"
      @click="toggleDrawer()"
      v-show="drawer.open"
    />
  </Transition>

  <UiGridbackground />
</template>

<script setup lang="ts">
const { data: extensions, pending } = await useAsyncData<Extension[]>(
  'extensions',
  () => $fetch<Extension[]>('/api/extensions'),
  {
    server: false,
  }
)

const drawerContainer = ref<HTMLElement>()
const drawerContent = ref<HTMLElement>()
const drawer = reactive({
  open: false,
  height: '0px',
})

const form = ref({
  search: '',
  sortBy: 'popularity',
  showExtensions: true,
  showThemes: true,
})

const filteredAndSortedExtensions = computed(() => {
  if (!extensions.value) return []

  let filtered = [...extensions.value]

  const searchTerm = form.value.search.toLowerCase()
  if (searchTerm) {
    filtered = filtered.filter(
      (extension) =>
        extension.name.toLowerCase().includes(searchTerm) ||
        extension.summary.toLowerCase().includes(searchTerm) ||
        extension.keywords.some((keyword) =>
          keyword.toLowerCase().includes(searchTerm)
        ) ||
        extension.author.name.toLowerCase().includes(searchTerm)
    )
  }

  filtered = filtered.filter((extension) => {
    if (extension.type === 'EXTENSION' && !form.value.showExtensions)
      return false
    if (extension.type === 'THEME' && !form.value.showThemes) return false
    return true
  })

  switch (form.value.sortBy) {
    case 'popularity':
      filtered.sort((a, b) => b.stats.panels - a.stats.panels)
      break
    case 'name':
      filtered.sort((a, b) => a.name.localeCompare(b.name))
      break
    case 'created':
      filtered.sort(
        (a, b) => new Date(b.created).getTime() - new Date(a.created).getTime()
      )
      break
  }

  return filtered
})

const toggleDrawer = async () => {
  if (!drawerContent.value) return

  if (drawer.open) {
    drawer.height = '0px'
    drawer.open = false
  } else {
    drawer.open = true

    await nextTick()

    const contentHeight = drawerContent.value.scrollHeight
    drawer.height = `${contentHeight}px`
  }
}
</script>

<style>
@media (max-width: 1024px) {
  :root {
    --extend-footer: 5rem;
  }
}
</style>
