<script setup lang="ts">
import {
  createUrlWithAppendedParams,
  navigateToUrlParams,
} from "../../utils/navigation";
import {BenchmarkInfo} from "../../api";

const props = defineProps<{
  metric: string;
  benchmarkInfo: BenchmarkInfo;
}>();

function createMetric(
  label: string,
  metric: string,
  description: string
): {
  label: string;
  metric: string;
  description: string;
} {
  return {label, metric, description};
}

function navigateToMetric(metric: string) {
  const params = {stat: metric};
  const url = createUrlWithAppendedParams(params);
  navigateToUrlParams(url.searchParams);
}

function changeMetric(e: Event) {
  const target = e.target as HTMLSelectElement;
  const metric = target.value;
  navigateToMetric(metric);
}

const metrics = [
  createMetric(
    "Instructions",
    "instructions:u",
    "Number of executed instructions"
  ),
  createMetric("Cycles", "cycles:u", "Number of executed cycles"),
  createMetric("Wall time", "wall-time", "Wall time"),
  createMetric("Max RSS", "max-rss", "Peak memory usage (resident set size)"),
  createMetric(
    "Binary size",
    "size:linked_artifact",
    "Size of the generated binary artifact"
  ),
];
</script>

<template>
  <div class="wrapper">
    <div class="label">Select metric:</div>
    <div
      v-for="metric in metrics"
      :class="{active: props.metric === metric.metric}"
      :title="metric.description"
    >
      <a href="#" @click.prevent="() => navigateToMetric(metric.metric)">{{
        metric.label
      }}</a>
    </div>
    <select class="stats" @change="changeMetric">
      <option
        v-for="value in benchmarkInfo.stats"
        :value="value"
        :selected="value === metric"
      >
        {{ value }}
      </option>
    </select>
  </div>
</template>

<style scoped lang="scss">
.wrapper {
  display: flex;
  flex-direction: row;
  align-items: center;
  margin: 10px 0;

  div {
    margin-right: 10px;
  }

  .active {
    font-weight: bold;
  }

  .label {
    display: none;

    @media (min-width: 600px) {
      display: block;
    }
  }
}
</style>
