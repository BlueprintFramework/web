<template>
  <div :class="props?.class" class="marquee h-32">
    <NuxtMarquee :autoFill="true" :pauseOnHover="true" :speed="35">
      <div
        v-for="(item, index) in items"
        :key="`item-${index}`"
        class="mx-4 inline h-[128px] w-[256px]"
      >
        <NuxtLink :to="`/browse/${item.identifier}`" tabindex="-1">
          <div
            class="overflow-hidden rounded-2xl transition-transform hover:scale-95"
          >
            <div
              class="absolute h-[128px] w-[256px] bg-neutral-950/50 opacity-0 transition-all hover:opacity-100"
            >
              <Icon
                name="memory:arrow-up-right-box"
                :size="48"
                mode="svg"
                class="top-6/12 left-6/12 -translate-3/6 absolute"
              />
            </div>
            <NuxtImg
              :src="`https://s3.blueprint.zip/static/${item.identifier}.jpeg`"
              :height="128"
              :width="256"
              :alt="item.name"
              class="aspect-[2/1] bg-neutral-800 object-cover"
            />
          </div>
        </NuxtLink>
      </div>
    </NuxtMarquee>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  class: {
    type: String,
    required: false,
  },
})

// extension banners should be prerendered and served by the api once
// possible, but until thats done i've hardcoded the values here
const items = ref([
  {
    name: 'Nebula',
    identifier: 'nebula',
  },
  {
    name: 'Pull Files',
    identifier: 'pullfiles',
  },
  {
    name: 'Server Splitter',
    identifier: 'serversplitter',
  },
  {
    name: 'Eggify',
    identifier: 'eggify',
  },
  {
    name: 'Social Login',
    identifier: 'sociallogin',
  },
  {
    name: 'Euphoria',
    identifier: 'euphoriatheme',
  },
  {
    name: 'Layeredy Announce',
    identifier: 'lyrdyannounce',
  },
  {
    name: 'Extension Manager',
    identifier: 'extmanager',
  },
  {
    name: 'PteroMonaco',
    identifier: 'pteromonaco',
  },
])
</script>
