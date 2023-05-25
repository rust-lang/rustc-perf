<script setup lang="ts">
import {createUrlWithAppendedParams, getUrlParams} from "../../../utils/navigation";

const props = defineProps<{stat: string}>();

function createMetric(label: string, stat: string, description: string): {
  label: string,
  stat: string,
  description: string
} {
  return {label, stat, description};
}

function createUrlForMetric(stat: string): string {
  const params = getUrlParams();
  params["stat"] = stat;
  return createUrlWithAppendedParams(params).toString();
}

const metrics = [
  createMetric("Instructions", "instructions:u", "Number of executed instructions"),
  createMetric("Cycles", "cycles:u", "Number of executed cycles"),
  createMetric("Wall time", "wall-time", "Wall time"),
  createMetric("Max RSS", "max-rss", "Peak memory usage (resident set size)"),
  createMetric("Binary size", "size:linked_artifact", "Size of the generated binary artifact")
];
</script>

<template>
  <div class="quick-links">
    <div>Quick links:</div>
    <div v-for="metric in metrics" :class="{ active: props.stat === metric.stat }" :title="metric.description">
      <a :href="createUrlForMetric(metric.stat)">{{ metric.label }}</a>
    </div>
  </div>
</template>

<style scoped lang="scss">
.quick-links {
    display: flex;
    flex-direction: row;
    margin: 10px 0;
}

.quick-links div {
    margin-right: 10px;
}

.quick-links .active {
    font-weight: bold;
}
</style>
