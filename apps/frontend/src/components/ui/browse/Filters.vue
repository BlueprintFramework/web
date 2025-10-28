<template>
  <div
    class="divide-y divide-neutral-700 rounded-2xl border border-neutral-700 transition-colors focus-within:divide-white focus-within:border-white"
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
        @click="props.form.sortBy = sortOption.value"
        class="hover:text-brand-50 focus:text-brand-50 focus:shadow-brand-50 text-default-font/60 block w-full cursor-pointer text-start outline-0 transition-colors focus:font-bold"
        tabindex="0"
        :class="{
          '!text-default-font': props.form.sortBy === sortOption.value,
        }"
      >
        <span>{{ sortOption.label }}</span>
      </button>
    </div>
  </div>
  <div class="group">
    <button
      @click="props.form.showExtensions = !props.form.showExtensions"
      class="focus:text-brand-50 hover:text-brand-50 w-full cursor-pointer rounded-t-2xl border border-neutral-700 p-2 outline-0 transition-colors group-focus-within:border-white"
      :class="
        props.form.showExtensions
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
          :class="props.form.showExtensions ? 'opacity-100' : 'opacity-0'"
        />
      </div>
    </button>
    <button
      @click="props.form.showThemes = !props.form.showThemes"
      class="focus:text-brand-50 hover:text-brand-50 w-full cursor-pointer rounded-b-2xl border border-t-0 border-neutral-700 p-2 outline-0 transition-colors group-focus-within:border-white"
      :class="
        props.form.showThemes
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
          :class="props.form.showThemes ? 'opacity-100' : 'opacity-0'"
        />
      </div>
    </button>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  form: {
    sortBy: string
    showExtensions: boolean
    showThemes: boolean
  }
}>()

const sortOptions = [
  { value: 'popularity', label: 'Most Popular' },
  { value: 'name', label: 'Name (A-Z)' },
  { value: 'created', label: 'Newest First' },
]
</script>
