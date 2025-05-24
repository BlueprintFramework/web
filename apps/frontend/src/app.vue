<template>
  <NuxtRouteAnnouncer />
  <UiNav />
  <div>
    <p>blueprinting so hard i ran out of cyan ink</p>
    <select v-model="$colorMode.preference">
      <option value="system">System</option>
      <option value="light">Light</option>
      <option value="dark">Dark</option>
      <option value="sepia">Sepia</option>
    </select>
  </div>
</template>

<script setup lang="ts">
const colorMode = useColorMode()
const updateTheme = () => {
  useHead({
    htmlAttrs: {
      'data-bs-theme': colorMode.preference,
    },
  })
}

updateTheme()
watch(colorMode, () => updateTheme())
</script>
