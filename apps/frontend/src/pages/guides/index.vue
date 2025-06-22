<template>
  <div
    class="grid grid-cols-2 divide-x divide-neutral-700 overflow-hidden rounded-3xl border border-neutral-700 bg-neutral-950"
  >
    <div class="flex p-8 lg:p-16">
      <div class="space-y-3 self-center">
        <div class="flex items-center gap-2">
          <Icon name="memory:book" mode="svg" :size="48" />
          <h1>Guides</h1>
        </div>
        <span class="opacity-75">
          Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque
          faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi
          pretium tellus duis convallis. Tempus leo eu aenean sed diam urna
          tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas.
          Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut
          hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent
          per conubia nostra inceptos himenaeos.
        </span>
      </div>
    </div>
    <NuxtImg
      src="/img/guides.jpeg"
      :width="1280"
      :height="1024"
      class="aspect-video h-full w-full object-cover"
    />
  </div>
</template>
