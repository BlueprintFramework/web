<template>
  <div
    class="flex divide-x divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
  >
    <div class="flex-1/2 divide-y divide-neutral-700">
      <div class="space-y-3 p-8">
        <div class="flex items-center gap-2">
          <Icon name="memory:book" mode="svg" :size="48" />
          <h1>Guides</h1>
        </div>
        <span class="opacity-75">
          Learn how to interact with our platform. From installing Blueprint to
          developing extensions, we're here to help you get started.
        </span>
      </div>
      <div class="p-8">
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
    </div>
    <div class="flex-1/2 relative">
      <NuxtImg
        src="/img/guides.jpeg"
        :width="1280"
        :height="600"
        class="absolute inset-0 h-full w-full object-cover"
      />
    </div>
  </div>
</template>
<script setup lang="ts">
const form = ref({
  search: '',
})
</script>
