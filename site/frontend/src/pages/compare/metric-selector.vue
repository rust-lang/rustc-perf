<script setup lang="ts">
import {
  createUrlWithAppendedParams,
  navigateToUrlParams,
} from "../../utils/navigation";
import {BenchmarkInfo} from "../../api";
import {MetricDescription} from "./metrics";

const props = defineProps<{
  quickLinks: MetricDescription[];
  selectedMetric: string;
  metrics: string[];
}>();

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
</script>

<template>
  <div class="wrapper">
    <div class="label">Select metric:</div>
    <div
      v-for="metric in quickLinks"
      :class="{active: props.selectedMetric === metric.metric}"
      :title="metric.description"
    >
      <a href="#" @click.prevent="() => navigateToMetric(metric.metric)">{{
        metric.label
      }}</a>
    </div>
    <select class="stats" @change="changeMetric" v-if="metrics.length > 0">
      <option
        v-for="value in metrics"
        :value="value"
        :selected="value === selectedMetric"
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
