<template>
  <UiMarketingLanding />

  <div class="container grow space-y-12 py-12">
    <UiMarketingHighlights />
    <UiMarketingMarquee />
    <UiMarketingHero />
    <UiMarketingRepositories />
    <UiMarketingFaq id="faq" />
    <UiMarketingTestimonials v-if="!isMobile" class="hidden md:block" />
    <UiMarketingSponsors />
  </div>
</template>

<script setup lang="ts">
const isMobile = useMediaQuery('(max-width: 768px)')

definePageMeta({
  layout: 'default-landing',
})

onMounted(() => {
  if (
    document.referrer.includes('baidu.com') ||
    document.referrer.includes('bilibili.com')
  ) {
    Tally.openPopup('b5vK71', {
      doNotShowAfterSubmit: false,
      hideTitle: true,
      overlay: true,
      emoji: {
        text: '💖',
        animation: 'heart-beat',
      },
    })
  }
})
</script>
