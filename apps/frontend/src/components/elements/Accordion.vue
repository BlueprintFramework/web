<template>
  <div class="flex flex-col gap-[1px]">
    <div
      v-for="(item, index) in props.items"
      :key="index"
      class="bg-neutral-950 transition-colors focus-within:bg-neutral-900 hover:bg-neutral-900"
    >
      <button
        class="hover:text-brand-50 focus:text-brand-50 flex w-full cursor-pointer items-center justify-between p-4 outline-0 transition-colors"
        @click="toggleAccordion(index)"
        @mousedown.prevent
      >
        <h2>
          {{ item.title }}
        </h2>
        <Icon
          name="memory:chevron-up"
          :size="24"
          :class="{ 'rotate-180': activeIndex !== index }"
          mode="svg"
          class="transition-transform"
        />
      </button>
      <div
        class="overflow-hidden transition-[max-height] duration-300 ease-in-out"
        :style="{
          maxHeight: activeIndex === index ? contentHeights[index] + 'px' : '0',
        }"
      >
        <div
          :ref="(el) => setContentRef(el as Element | null, index)"
          class="text-default-font/75 p-4 pt-0"
        >
          <template v-if="activeIndex == index || oldIndex == index">
            <p v-if="item.text">
              {{ item.text }}
            </p>
            <template v-else>
              <component
                v-for="(vnode, vnodeIndex) in (item.inner as Function)()"
                :key="vnodeIndex"
                :is="vnode"
              />
            </template>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface AccordionItem {
  title: string
  text?: string
  inner?: Function
}

const props = defineProps<{
  items: AccordionItem[]
  default?: number
}>()

const activeIndex = ref(-1)
const oldIndex = ref(-1)
const blocked = ref(false)
const contentRefs = ref<(Element | null)[]>([])
const contentHeights = ref<number[]>([])

const setContentRef = (el: Element | null, index: number) => {
  if (el) {
    contentRefs.value[index] = el
  }
}

onMounted(() => {
  if (props?.default != null) {
    toggleAccordion(props?.default || 0)
  }
})

const toggleAccordion = async (index: number) => {
  if (blocked.value) return

  oldIndex.value = activeIndex.value
  if (activeIndex.value === index) {
    activeIndex.value = -1
  } else {
    activeIndex.value = index
    await nextTick()

    if (contentRefs.value[index]) {
      contentHeights.value[index] = contentRefs.value[index]!.scrollHeight
    }
  }

  blocked.value = true
  setTimeout(() => {
    oldIndex.value = -1
    blocked.value = false
  }, 200)
}
</script>
