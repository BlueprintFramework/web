<template>
  <div class="my-4 rounded-2xl border border-neutral-700 bg-neutral-900 p-4">
    <AreaChart
      :data="AreaChartData"
      :height="260"
      y-label="Extensions"
      :categories="categories"
      :x-num-ticks="11"
      :y-grid-line="true"
      :legend-position="LegendPosition.TopRight"
      :hide-legend="false"
      :x-formatter="xFormatter"
    />
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  // Disable class and style props
  class?: string
  style?: string
}>()

const categories: Record<string, BulletLegendItemInterface> = {
  avgextensions: { name: 'Average installed extensions', color: '#52A9FF' },
}

interface AreaChartItem {
  date: string
  avgextensions: number
}

const AreaChartData: AreaChartItem[] = [
  { date: 'Week 1', avgextensions: 2.69 },
  { date: 'Week 2', avgextensions: 2.75 },
  { date: 'Week 3', avgextensions: 2.64 },
  { date: 'Week 4', avgextensions: 2.75 },
  { date: 'Week 5', avgextensions: 2.84 },
  { date: 'Week 6', avgextensions: 2.93 },
  { date: 'Week 7', avgextensions: 3.04 },
  { date: 'Week 8', avgextensions: 2.95 },
  { date: 'Week 9', avgextensions: 3.05 },
  { date: 'Week 10', avgextensions: 2.94 },
  { date: 'Week 11', avgextensions: 2.9 },
  { date: 'Week 12', avgextensions: 3.0 },
  { date: 'Week 13', avgextensions: 2.92 },
  { date: 'Week 14', avgextensions: 2.95 },
  { date: 'Week 15', avgextensions: 2.93 },
  { date: 'Week 16', avgextensions: 3.2 },
  { date: 'Week 17', avgextensions: 3.23 },
  { date: 'Week 18', avgextensions: 3.3 },
  { date: 'Week 19', avgextensions: 3.38 },
  { date: 'Week 20', avgextensions: 3.43 },
  { date: 'Week 21', avgextensions: 3.4 },
  { date: 'Week 22', avgextensions: 3.48 },
  { date: 'Week 23', avgextensions: 3.52 },
  { date: 'Week 24', avgextensions: 3.54 },
  { date: 'Week 25', avgextensions: 3.55 },
  { date: 'Week 26', avgextensions: 3.51 },
  { date: 'Week 27', avgextensions: 3.55 },
  { date: 'Week 28', avgextensions: 3.61 },
  { date: 'Week 29', avgextensions: 3.66 },
  { date: 'Week 30', avgextensions: 3.68 },
  { date: 'Week 31', avgextensions: 3.73 },
  { date: 'Week 32', avgextensions: 3.78 },
  { date: 'Week 33', avgextensions: 3.83 },
  { date: 'Week 34', avgextensions: 3.9 },
  { date: 'Week 35', avgextensions: 3.91 },
  { date: 'Week 36', avgextensions: 3.93 },
  { date: 'Week 37', avgextensions: 3.97 },
  { date: 'Week 38', avgextensions: 3.95 },
  { date: 'Week 39', avgextensions: 4.02 },
  { date: 'Week 40', avgextensions: 4.05 },
  { date: 'Week 41', avgextensions: 4.03 },
  { date: 'Week 42', avgextensions: 4.1 },
  { date: 'Week 43', avgextensions: 4.2 },
  { date: 'Week 44', avgextensions: 4.28 },
  { date: 'Week 45', avgextensions: 4.27 },
  { date: 'Week 46', avgextensions: 4.34 },
  { date: 'Week 47', avgextensions: 4.23 },
  { date: 'Week 48', avgextensions: 4.34 },
  { date: 'Week 49', avgextensions: 4.34 },
  { date: 'Week 50', avgextensions: 4.38 },
  { date: 'Week 51', avgextensions: 4.34 },
  { date: 'Week 52', avgextensions: 4.39 },
  { date: 'Week 53', avgextensions: 4.34 },
]

const xFormatter = (tick: number, _i?: number, _ticks?: number[]): string => {
  return `${AreaChartData[tick]?.date ?? ''}`
}
</script>
