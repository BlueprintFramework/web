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
    :class="props.class + (richtext ? ' rounded-t-none' : '')"
    @input="resize"
    @select="saveSelection"
    @click="saveSelection"
    @keyup="saveSelection"
    class="h-auto w-full resize-none overflow-hidden rounded-2xl border border-neutral-700 bg-gray-800/40 p-4 outline-0 transition-colors focus:bg-gray-800/60"
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
    richtext?: boolean
    supportsImages?: boolean
  }>(),
  {
    autoresize: true,
    richtext: false,
    wrap: 'hard',
    rows: 4,
  }
)

const model = defineModel<string>()
const id = useId()
const textarea = useTemplateRef('textarea')
const selectionStart = ref(0)
const selectionEnd = ref(0)

const saveSelection = () => {
  if (!textarea.value) return
  selectionStart.value = textarea.value.selectionStart
  selectionEnd.value = textarea.value.selectionEnd
}

const insertMarkdown = (before: string, after: string = '') => {
  if (!textarea.value) return

  const text = model.value || ''
  const start = selectionStart.value
  const end = selectionEnd.value
  const selected = text.substring(start, end)

  model.value =
    text.substring(0, start) + before + selected + after + text.substring(end)

  nextTick(() => {
    if (!textarea.value) return
    const newPos = start + before.length + selected.length
    textarea.value.focus()
    textarea.value.setSelectionRange(newPos, newPos)
    saveSelection()
    resize()
  })
}

const wrapMarkdown = (wrapper: string) => insertMarkdown(wrapper, wrapper)

const insertLink = async () => {
  if (!textarea.value) return

  const clipboard = await navigator.clipboard.readText()
  let url
  try {
    url = new URL(clipboard)
  } catch {}

  if (url) {
    insertMarkdown('[', `](${url.href})`)
    return
  }

  insertMarkdown('[', `](https://example.com)`)
}

const insertImage = (url: string) => {
  if (!props.supportsImages) return
  insertMarkdown('![', `](${url})`)
}

const insertHeading = () => {
  if (!textarea.value) return

  const text = model.value || ''
  const cursorPos = selectionStart.value

  // find current line boundaries
  const beforeCursor = text.substring(0, cursorPos)
  const afterCursor = text.substring(cursorPos)
  const lineStart = beforeCursor.lastIndexOf('\n') + 1
  const lineEndInAfter = afterCursor.indexOf('\n')
  const lineEnd =
    lineEndInAfter === -1 ? text.length : cursorPos + lineEndInAfter

  const currentLine = text.substring(lineStart, lineEnd)

  // check current heading level
  const headingMatch = currentLine.match(/^(#{1,6})\s/)
  const currentLevel =
    headingMatch && headingMatch[1] ? headingMatch[1].length : 0

  let newLine: string
  let cursorOffset = 0

  if (currentLevel === 0) {
    // no heading, add h1
    newLine = '# ' + currentLine
    cursorOffset = -2
  } else if (currentLevel < 6) {
    // cycle to next level
    newLine = currentLine.replace(/^#{1,6}/, '#'.repeat(currentLevel + 1))
    cursorOffset = -1
  } else {
    // at h6, remove heading
    newLine = currentLine.replace(/^#{6}\s/, '')
    cursorOffset = 7
  }

  model.value = text.substring(0, lineStart) + newLine + text.substring(lineEnd)

  nextTick(() => {
    if (!textarea.value) return
    const newCursorPos = cursorPos - cursorOffset
    textarea.value.focus()
    textarea.value.setSelectionRange(newCursorPos, newCursorPos)
    saveSelection()
    resize()
  })
}

const insertList = () => insertMarkdown('- ', '')
const insertQuote = () => insertMarkdown('> ', '')
const insertCard = () => {
  insertMarkdown('::card\n', '\n::')
}
const insertCodeblock = () => {
  insertMarkdown('```txt [file.txt]\n', '\n```')
}
const insertTable = () => {
  insertMarkdown(
    '| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |\n',
    ''
  )
}

defineExpose({
  supportsImages: props.supportsImages,
  insertMarkdown,
  wrapMarkdown,
  insertLink,
  insertImage,
  insertHeading,
  insertQuote,
  insertList,
  insertCard,
  insertCodeblock,
  insertTable,
  bold: () => wrapMarkdown('**'),
  italic: () => wrapMarkdown('*'),
  strikethrough: () => wrapMarkdown('~~'),
  code: () => wrapMarkdown('`'),
})

const resize = () => {
  if (!textarea.value || !props.autoresize) return
  textarea.value.style.height = 'auto'
  textarea.value.style.height = textarea.value.scrollHeight + 'px'
}

onMounted(() => {
  resize()
  if (!textarea.value) return
  const observer = new ResizeObserver(() => resize())
  observer.observe(textarea.value)
  onBeforeUnmount(() => observer.disconnect())
})

watch(model, () => {
  nextTick(() => resize())
})
</script>
