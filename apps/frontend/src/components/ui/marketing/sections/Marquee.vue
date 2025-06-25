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
            <NuxtImg src="" :height="128" :width="256" :alt="item.name" />
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
  },
  {
    name: 'File Search',
    identifier: 'filesearch',
  },
  {
    name: 'Loader',
    identifier: 'loader',
  },
  {
    name: 'Nebula',
    identifier: 'nebula',
  },
  {
    name: 'Server Importer',
    identifier: 'serverimporter',
  },
  {
    name: 'Server Splitter',
    identifier: 'serversplitter',
  },
  {
    name: 'Social Login',
    identifier: 'sociallogin',
  },
  {
    name: 'Subdomains',
    identifier: 'subdomains',
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
