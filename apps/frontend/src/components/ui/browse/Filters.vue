<template>
  <div
    class="divide-y divide-neutral-700 rounded-2xl border border-neutral-700 transition-colors focus-within:divide-neutral-500 focus-within:border-neutral-500"
  >
    <div class="flex items-center gap-1.5 p-2 font-bold transition-colors">
      <Icon
        name="memory:format-text-single-line"
        :size="22"
        mode="svg"
        class="block"
      />
      <span>Sort by</span>
    </div>
    <div class="space-y-2 p-2 transition-colors">
      <button
        v-for="sortOption in sortOptions"
        :key="sortOption.value"
        class="hover:text-brand-50 focus:text-brand-50 text-default-font/60 block w-full cursor-pointer text-start outline-0 transition-colors focus:font-bold"
        tabindex="0"
        :class="{
          '!text-default-font': props.form.sortBy === sortOption.value,
        }"
        @click="props.form.sortBy = sortOption.value"
        @mousedown.prevent
      >
        <span>{{ sortOption.label }}</span>
      </button>
    </div>
  </div>

  <div
    class="divide-y divide-neutral-700 rounded-2xl border border-neutral-700 transition-colors focus-within:divide-neutral-500 focus-within:border-neutral-500"
  >
    <div class="flex items-center gap-1.5 p-2 font-bold transition-colors">
      <Icon
        name="memory:cash"
        :size="22"
        mode="svg"
        class="block"
      />
      <span>Price</span>
    </div>
    <div class="space-y-3 p-4 transition-colors">
      <input
        type="range"
        min="0"
        max="1000"
        step="10"
        v-model.number="props.form.maxPrice"
        class="slider block w-full rounded-lg appearance-none cursor-pointer"
      />
      <div class="flex justify-between text-xs text-default-font/40">
        <span>Free</span>
        <span>Paid</span>
      </div>
    </div>
  </div>

  <div class="group">
    <button
      class="focus:text-brand-50 hover:text-brand-50 w-full cursor-pointer rounded-t-2xl border border-neutral-700 p-2 outline-0 transition-colors group-focus-within:border-neutral-500"
      :class="
        props.form.showExtensions
          ? 'bg-neutral-900'
          : 'text-default-font/50 bg-neutral-950'
      "
      @click="props.form.showExtensions = !props.form.showExtensions"
      @mousedown.prevent
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
      class="focus:text-brand-50 hover:text-brand-50 w-full cursor-pointer rounded-b-2xl border border-t-0 border-neutral-700 p-2 outline-0 transition-colors group-focus-within:border-neutral-500"
      :class="
        props.form.showThemes
          ? 'bg-neutral-900'
          : 'text-default-font/50 bg-neutral-950'
      "
      @click="props.form.showThemes = !props.form.showThemes"
      @mousedown.prevent
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
    maxPrice: number
  }
}>()

const sortOptions = [
  { value: 'popularity', label: 'Most Popular' },
  { value: 'name', label: 'Name (A-Z)' },
  { value: 'created', label: 'Newest First' },
]
</script>

<style scoped>
.slider {
  background: white;
  height: 6px;
  border-radius: 999px;
}

.slider::-webkit-slider-runnable-track {
  background: white;
  height: 6px;
  border-radius: 999px;
}

.slider::-moz-range-track {
  background: white;
  height: 6px;
  border-radius: 999px;
}

.slider::-webkit-slider-thumb {
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #6db7ff;
  cursor: pointer;
  border: 2px solid #6db7ff;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.35);
  margin-top: -6px;
}

.slider::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #6db7ff;
  cursor: pointer;
  border: 2px solid #6db7ff;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.35);
  margin-top: -6px;
}

.slider::-webkit-slider-thumb:hover {
  transform: scale(1.1);
}

.slider::-moz-range-thumb:hover {
  transform: scale(1.1);
}
</style>
