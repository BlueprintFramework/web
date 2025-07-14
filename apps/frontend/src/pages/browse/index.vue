<template>
  <div class="flex items-center justify-between">
    <h1>Extensions? We've got 'em.</h1>
    <pre class="text-2xl font-bold">[* *  ]</pre>
  </div>
  <div class="flex flex-row gap-4">
    <div class="w-75">
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
            <div
              class="divide-y divide-neutral-700 rounded-2xl border border-neutral-700"
            >
              <div class="flex items-center gap-1.5 p-2 font-bold">
                <Icon
                  name="memory:format-text-single-line"
                  :size="22"
                  mode="svg"
                  class="block"
                />
                <span>Sort by</span>
              </div>
              <div class="space-y-2 p-2">
                <button
                  v-for="sortOption in sortOptions"
                  :key="sortOption.value"
                  @click="form.sortBy = sortOption.value"
                  class="hover:text-brand-50 text-default-font/60 block w-full cursor-pointer text-start transition-colors"
                  :class="{
                    '!text-default-font': form.sortBy === sortOption.value,
                  }"
                >
                  <span>{{ sortOption.label }}</span>
                </button>
              </div>
            </div>
            <div>
              <div
                @click="form.showExtensions = !form.showExtensions"
                class="hover:text-brand-50 cursor-pointer rounded-t-2xl border border-neutral-700 p-2 transition-colors"
                :class="
                  form.showExtensions
                    ? 'bg-neutral-900'
                    : 'text-default-font/50 bg-neutral-950'
                "
              >
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-1.5">
                    <Icon name="memory:cube" :size="24" />
                    <span>Extensions</span>
                  </div>
                  <Icon
                    name="memory:check"
                    class="text-default-font transition-opacity"
                    :class="form.showExtensions ? 'opacity-100' : 'opacity-0'"
                  />
                </div>
              </div>
              <div
                @click="form.showThemes = !form.showThemes"
                class="hover:text-brand-50 cursor-pointer rounded-b-2xl border border-t-0 border-neutral-700 p-2 transition-colors"
                :class="
                  form.showThemes
                    ? 'bg-neutral-900'
                    : 'text-default-font/50 bg-neutral-950'
                "
              >
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-1.5">
                    <Icon name="memory:image" :size="24" />
                    <span>Themes</span>
                  </div>
                  <Icon
                    name="memory:check"
                    class="text-default-font transition-opacity"
                    :class="form.showThemes ? 'opacity-100' : 'opacity-0'"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="w-[calc(100%-18.75rem)]">
      <div
        class="grid grid-cols-1 gap-px overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-700 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4"
      >
        <UiExtensioncard
          v-if="pending"
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

  <div
    class="absolute inset-0 top-[var(--nav-offset)] -z-10 h-[50vh] w-full bg-[linear-gradient(to_right,var(--color-neutral-800)_1px,transparent_1px),linear-gradient(to_bottom,var(--color-neutral-800)_1px,transparent_1px)] bg-[size:30px_30px]"
    style="background-position-x: -5px"
  >
    <div class="bg-linear-to-b h-full w-full from-transparent to-neutral-950" />
  </div>
</template>

<script setup lang="ts">
const { data: extensions, pending } = await useAsyncData<Extension[]>(
  'extensions',
  () => $fetch<Extension[]>('/api/extensions'),
  {
    server: false,
  }
)

const form = ref({
  search: '',
  sortBy: 'popularity',
  showExtensions: true,
  showThemes: true,
})

const sortOptions = [
  { value: 'popularity', label: 'Most Popular' },
  { value: 'name', label: 'Name (A-Z)' },
  { value: 'created', label: 'Newest First' },
]

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
</script>
