<template>
  <textarea
    v-model="model"
    ref="textarea"
    :id="id"
    :autocapitalize="props.autocapitalize"
    :autocomplete="props.autocomplete"
    :autofocus="props.autofocus"
    :cols="props.cols"
    :dirname="props.dirname"
    :disabled="props.disabled"
    :form="props.form"
    :minlength="props.minlength"
    :maxlength="props.maxlength"
    :name="props.name"
    :placeholder="props.placeholder"
    :readonly="props.readonly"
    :required="props.required"
    :rows="props.rows"
    :spellcheck="props.spellcheck"
    :wrap="props.wrap"
    :class="props.class"
    @input="resize"
    class="h-auto w-full resize-none overflow-hidden rounded-2xl border border-neutral-700 bg-gray-800/40 p-4"
  />
</template>

<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    autocapitalize?: string
    autocomplete?: string | 'on' | 'off'
    autofocus?: boolean
    cols?: number
    class?: string
    dirname?: string
    disabled?: boolean
    form?: string
    minlength?: number
    maxlength?: number
    name?: string
    placeholder?: string
    readonly?: boolean
    required?: boolean
    rows?: number
    spellcheck?: boolean
    wrap?: 'hard' | 'soft'
    autoresize?: boolean
  }>(),
  {
    autoresize: true,
    wrap: 'hard',
    rows: 4,
  }
)

const model = defineModel<string>()
const id = useId()
const textarea = ref()

const resize = () => {
  if (!textarea.value || !props.autoresize) return
  textarea.value.style.height = 'auto'
  textarea.value.style.height = textarea.value.scrollHeight + 'px'
}

onMounted(() => resize())
</script>
