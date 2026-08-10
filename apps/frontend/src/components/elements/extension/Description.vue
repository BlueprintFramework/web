<template>
  <Markdown
    v-if="markdown"
    class="prose-content"
    :class="props?.class"
    :value="markdown"
    :components="components"
    :plugins="plugins"
  />
</template>

<script setup lang="ts">
import security from '@comark/nuxt/plugins/security'

import Card from '~/components/prose/Card.vue'
import Externalimg from '~/components/prose/Externalimg.vue'
import ProseH2 from '~/components/prose/ProseH2.vue'
import ProseH3 from '~/components/prose/ProseH3.vue'
import ProseH4 from '~/components/prose/ProseH4.vue'
import ProseH5 from '~/components/prose/ProseH5.vue'
import ProseH6 from '~/components/prose/ProseH6.vue'
import ProseEm from '~/components/prose/ProseEm.vue'
import ProsePreSafe from '~/components/prose/ProsePreSafe.vue'
import ProseCode from '~/components/prose/ProseCode.vue'

const props = defineProps<{
  markdown?: string | null
  class?: string | null
}>()

const components = {
  card: Card,
  code: ProseCode,
  h1: ProseH2,
  h2: ProseH3,
  h3: ProseH4,
  h4: ProseH5,
  h5: ProseH6,
  h6: ProseH6,
  em: ProseEm,
  img: Externalimg,
  pre: ProsePreSafe,
  'prose-pre': ProsePreSafe,
  'prose-img': Externalimg,
}

const plugins = [
  security({
    blockedTags: [
      'script',
      'style',
      'iframe',
      'object',
      'embed',
      'ProseVideoPlayer',
      'prose-video-player',
    ],
  }),
]
</script>
