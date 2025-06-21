<template>
  <div :class="props?.class" class="marquee h-32">
    <NuxtMarquee :autoFill="true" :pauseOnHover="true" :speed="35">
      <div
        v-for="(item, index) in items"
        :key="`item-${index}`"
        class="mx-4 inline h-[128px] w-[256px]"
      >
        <NuxtLink :to="`/browse/${item.identifier}`">
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
              :src="item.src"
              :height="128"
              :width="256"
              :alt="item.name"
            />
          </div>
        </NuxtLink>
      </div>
    </NuxtMarquee>
  </div>
</template>

<script setup lang="ts">
const img = useImage()

const props = defineProps({
  class: {
    type: String,
    required: false,
  },
})

// ipx_src is used for prerendering images upon static site generation
// as they are not generated through NuxtMarquee for some reason
//
// extension banners should be prerendered and served by the api once
// possible, but until thats done i've hardcoded the values here
const items = ref([
  {
    name: 'Custom Server Sort',
    identifier: 'customserversort',
    src: '/img/extensions/customserversort.jpg',
    ipx_src: img('/img/extensions/customserversort.jpg', {
      height: 128,
      width: 256,
    }),
  },
  {
    name: 'File Search',
    identifier: 'filesearch',
    src: '/img/extensions/filesearch.jpg',
    ipx_src: img('/img/extensions/filesearch.jpg', { height: 128, width: 256 }),
  },
  {
    name: 'Loader',
    identifier: 'loader',
    src: '/img/extensions/loader.jpg',
    ipx_src: img('/img/extensions/loader.jpg', { height: 128, width: 256 }),
  },
  {
    name: 'Nebula',
    identifier: 'nebula',
    src: '/img/extensions/nebula.jpg',
    ipx_src: img('/img/extensions/nebula.jpg', { height: 128, width: 256 }),
  },
  {
    name: 'Server Importer',
    identifier: 'serverimporter',
    src: '/img/extensions/serverimporter.jpg',
    ipx_src: img('/img/extensions/serverimporter.jpg', {
      height: 128,
      width: 256,
    }),
  },
  {
    name: 'Server Splitter',
    identifier: 'serversplitter',
    src: '/img/extensions/serversplitter.jpg',
    ipx_src: img('/img/extensions/serversplitter.jpg', {
      height: 128,
      width: 256,
    }),
  },
  {
    name: 'Social Login',
    identifier: 'sociallogin',
    src: '/img/extensions/sociallogin.jpg',
    ipx_src: img('/img/extensions/sociallogin.jpg', {
      height: 128,
      width: 256,
    }),
  },
  {
    name: 'Subdomains',
    identifier: 'subdomains',
    src: '/img/extensions/subdomains.jpg',
    ipx_src: img('/img/extensions/subdomains.jpg', { height: 128, width: 256 }),
  },
])
</script>

<style scoped>
.marquee {
  mask: linear-gradient(
    to right,
    transparent 0%,
    black 10%,
    black 90%,
    transparent 100%
  );
  -webkit-mask: linear-gradient(
    to right,
    transparent 0%,
    black 10%,
    black 90%,
    transparent 100%
  );
}
</style>
