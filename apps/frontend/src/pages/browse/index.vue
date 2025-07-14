<template>
  <div class="flex items-center justify-between">
    <h1>Extensions? We've got 'em.</h1>
    <pre class="text-2xl font-bold">[* *  ]</pre>
  </div>
  <div class="flex flex-row gap-4">
    <div class="w-1/5">
      <div
        class="sticky top-[calc(var(--nav-offset)+1rem)] overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
      >
        <div class="border-b border-neutral-700 p-4">
          <UiFormInput
            name="search"
            type="text"
            :rules="[]"
            leading-icon="memory:search"
            placeholder="Search.."
            @validate="void"
          />
        </div>
        <div class="border-b border-neutral-700 p-4">
          <span>Sort by</span>
        </div>
        <div class="p-4">
          <div
            class="rounded-t-2xl border border-neutral-700 bg-neutral-900 p-2"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-1.5">
                <Icon name="memory:cube" :size="24" />
                <span>Extensions</span>
              </div>
              <Icon name="memory:check" />
            </div>
          </div>
          <div
            class="rounded-b-2xl border border-t-0 border-neutral-700 bg-neutral-950 p-2"
          >
            <div class="flex items-center justify-between">
              <div class="text-default-font/50 flex items-center gap-1.5">
                <Icon name="memory:image" :size="24" />
                <span>Themes</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div
      class="grid w-4/5 grid-cols-1 gap-px overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-700 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
    >
      <UiExtensioncard
        v-if="pending"
        v-for="n in 28"
        class="flex flex-col bg-neutral-950"
      />

      <UiExtensioncard
        v-else
        v-for="extension in extensions || []"
        :key="extension.id"
        :extension="extension"
        class="flex flex-col bg-neutral-950"
      />

      <div class="min-h-5 w-[calc(400%+3px)] bg-neutral-950">
        <div class="bg-stripes h-full w-full" />
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
</script>
