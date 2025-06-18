<template>
  <div>
    <div
      v-for="(item, index) in props.items"
      :key="index"
      class="border-b border-neutral-700 transition-colors hover:bg-neutral-900"
      :class="{ '!border-b-0': index === props.items.length - 1 }"
    >
      <div
        class="hover:text-brand-50 flex cursor-pointer items-center justify-between p-4 transition-colors"
        @click="toggleAccordion(index)"
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
      </div>
      <div
        class="overflow-hidden transition-[max-height] duration-300 ease-in-out"
        :style="{
          maxHeight: activeIndex === index ? contentHeights[index] + 'px' : '0',
        }"
      >
        <div
          :ref="(el) => setContentRef(el as Element | null, index)"
          class="p-4 pt-0"
        >
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
  if (activeIndex.value === index) {
    activeIndex.value = -1
  } else {
    activeIndex.value = index
    await nextTick()

    if (contentRefs.value[index]) {
      contentHeights.value[index] = contentRefs.value[index]!.scrollHeight
    }
  }
}
</script>
