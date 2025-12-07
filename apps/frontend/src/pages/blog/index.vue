<template>
  <div class="space-y-2 border-b border-neutral-700 p-4 text-center">
    <h1>Blog posts</h1>
    <p>Articles from the industry-leading modding platform for Pterodactyl</p>
  </div>

  <div
    class="grid grid-cols-1 gap-px border-b border-neutral-700 bg-neutral-700 md:grid-cols-2"
  >
    <NuxtLink
      v-for="post in posts"
      :key="post.id"
      :to="post.path"
      class="hover:text-brand-50 group overflow-hidden bg-neutral-950 transition-colors"
    >
      <div
        class="absolute ms-3 mt-3 inline-block rounded-full border border-neutral-700 bg-neutral-900"
      >
        <span class="text-default-font! px-2 py-0.5 text-sm">
          <NuxtTime relative :datetime="post?.date" />
        </span>
      </div>
      <div
        class="h-40 bg-cover bg-center bg-no-repeat"
        :style="`background-image: url('/img/blog/thumbnails/${post.thumbnail || 'default.jpeg'}');`"
      >
        <div
          class="bg-linear-to-b h-full w-full from-transparent from-35% to-neutral-950"
        />
      </div>
      <div class="-mt-8 space-y-1 p-3">
        <p class="h2 truncate">{{ post.title || 'Untitled post' }}</p>
        <p class="text-default-font! truncate">
          {{ post.description || 'No description provided' }}
        </p>
      </div>
    </NuxtLink>
    <div
      v-if="(posts?.length || 0) % 2 == 0"
      class="relative -mt-px hidden w-[calc(200%+1px)] bg-neutral-950 md:block"
    ></div>
    <div
      v-if="(posts?.length || 0) % 2 != 0"
      class="hidden bg-neutral-950 md:block"
    />
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'blog',
})

const { data: posts } = await useAsyncData(`blogs`, () =>
  queryCollection('blog')
    .orWhere((query) =>
      query.where('unlisted', '=', false).where('unlisted', 'IS NULL')
    )
    .order('num', 'DESC')
    .all()
)
</script>
