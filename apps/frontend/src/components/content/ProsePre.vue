<template>
  <div class="overflow-auto">
    <div
      class="w-full overflow-hidden rounded-t-2xl border border-neutral-700 bg-neutral-950"
    >
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2 overflow-hidden p-2">
          <Icon name="memory:application-code" :size="24" mode="svg" />
          <span class="truncate">
            {{ props.filename || props.language || 'txt' }}
          </span>
        </div>
        <div class="border-s border-neutral-700">
          <button
            class="hover:text-brand-50 flex cursor-pointer items-center gap-1 p-3 transition-colors hover:bg-neutral-900 md:p-2"
            @click="copyPreContent"
          >
            <Icon
              :name="copied ? 'memory:check' : 'memory:clipboard'"
              :size="18"
              mode="svg"
              class="md:site-[unset] size-5 md:pb-0.5"
              :class="{ 'animate-pulse': copied }"
            />
            <span class="hidden md:inline">Copy</span>
          </button>
        </div>
      </div>
    </div>
    <div
      class="flex w-full rounded-b-2xl border border-t-0 border-neutral-700 bg-neutral-950"
    >
      <pre
        :class="props.class"
        class="w-full min-w-0 overflow-x-auto p-2"
      ><slot /></pre>
    </div>
  </div>
</template>
<script setup lang="ts">
const copied = ref(false)
const props = defineProps({
  code: {
    type: String,
    default: '',
  },
  language: {
    type: String,
    default: null,
  },
  filename: {
    type: String,
    default: null,
  },
  highlights: {
    type: Array as () => number[],
    default: () => [],
  },
  meta: {
    type: String,
    default: null,
  },
  class: {
    type: String,
    default: null,
  },
})
const copyPreContent = async () => {
  await navigator.clipboard.writeText(props.code)
  copied.value = true
  setTimeout(() => {
    copied.value = false
  }, 2000)
}
</script>
<style scoped>
pre code .line {
  display: block;
}
</style>
