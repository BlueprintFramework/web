<template>
  <div>
    <p v-if="label" class="mb-2 text-sm font-medium text-gray-300">
      {{ label }}
    </p>
    <div class="grid grid-cols-2 divide-x divide-blue-500">
      <ElementsButton
        v-for="(option, index) in options"
        :key="String(option.value)"
        :class="[
          'flex w-full flex-col items-center',
          index === 0
            ? 'rounded-e-none border-e-0'
            : 'rounded-s-none border-s-0',
          modelValue === option.value
            ? index === 0
              ? '!border-e'
              : '!border-s'
            : '',
        ]"
        :active="modelValue === option.value"
        @click="$emit('update:modelValue', option.value)"
      >
        <div class="flex items-center gap-2">
          <Icon :name="option.icon" />
          <span>{{ option.label }}</span>
        </div>
      </ElementsButton>
    </div>
    <div v-if="description" class="mt-2 text-sm text-gray-400">
      {{ description }}
    </div>
  </div>
</template>

<script setup lang="ts" generic="T extends string | boolean">
interface Option<V = T> {
  value: V
  icon: string
  label: string
}

interface Props {
  modelValue: T
  options: readonly Option<T>[]
  label?: string
  description?: string
}

defineProps<Props>()
defineEmits<{
  'update:modelValue': [value: T]
}>()
</script>
