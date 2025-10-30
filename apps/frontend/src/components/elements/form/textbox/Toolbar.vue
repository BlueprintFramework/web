<template>
  <div
    v-if="textareaRef"
    class="scrollbar-none flex divide-x divide-neutral-700 overflow-x-scroll rounded-t-2xl border-x border-t border-neutral-700"
  >
    <div class="flex items-center p-2">
      <button
        @mousedown.prevent
        @click="textareaRef.bold()"
        title="Bold"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:bold" />
      </button>
      <button
        @mousedown.prevent
        @click="textareaRef.italic()"
        title="Italic"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:italic" />
      </button>
      <button
        @mousedown.prevent
        @click="textareaRef.strikethrough()"
        title="Strikethrough"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:strikethrough" />
      </button>
      <button
        @mousedown.prevent
        @click="textareaRef.code()"
        title="Code"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:code" />
      </button>
      <button
        @mousedown.prevent
        @click="textareaRef.insertLink()"
        title="Link"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:link" />
      </button>
    </div>
    <div class="flex items-center p-2">
      <button
        @mousedown.prevent
        @click="textareaRef.insertHeading()"
        title="Title"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:heading-1" />
      </button>
      <button
        @mousedown.prevent
        @click="textareaRef.insertList()"
        title="List"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:list" />
      </button>
      <button
        @mousedown.prevent
        @click="textareaRef.insertQuote()"
        title="Quote"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:quote" />
      </button>
    </div>
    <div class="flex items-center p-2">
      <button
        @mousedown.prevent
        @click="textareaRef.insertCodeblock()"
        title="Codeblock"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:square-code" />
      </button>
      <button
        v-if="textareaRef.supportsImages && $props.onImage"
        @mousedown.prevent
        @click="$props.onImage()"
        title="Image"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:image" />
      </button>
      <button
        @mousedown.prevent
        @click="textareaRef.insertTable()"
        title="Table"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:table-2" />
      </button>
      <button
        @mousedown.prevent
        @click="textareaRef.insertCard()"
        title="Card"
        class="cursor-pointer rounded-[8px] p-2 transition-colors hover:bg-neutral-800"
      >
        <Icon name="lucide:square" />
      </button>
    </div>
    <div class="bg-stripes w-full min-w-10" />
  </div>
</template>

<script setup lang="ts">
defineProps<{
  textareaRef: any
  onImage?: () => void
}>()
</script>
