<script setup lang="ts">
import {CompareResponse, CompareSelector} from "../types";
import OverallSummary from "../summary/overall-summary.vue";
import {computed, ref} from "vue";
import {computeSummary, filterNonRelevant} from "../data";
import {
  computeRuntimeComparisonsWithNonRelevant,
  defaultRuntimeFilter,
} from "./common";
import {deepCopy} from "../../../utils/copy";
import MetricSelector from "../metric-selector.vue";
import {BenchmarkInfo} from "../../../api";
import {importantRuntimeMetrics} from "../metrics";

const props = defineProps<{
  data: CompareResponse;
  selector: CompareSelector;
  benchmarkInfo: BenchmarkInfo;
}>();

const filter = ref(deepCopy(defaultRuntimeFilter));
const allComparisons = computed(() =>
  computeRuntimeComparisonsWithNonRelevant(
    filter.value,
    props.data.runtime_comparisons
  )
);
const comparisons = computed(() =>
  filterNonRelevant(filter.value, allComparisons.value)
);
const filteredSummary = computed(() => computeSummary(comparisons.value));
</script>

<template>
  <MetricSelector
    :selected-metric="selector.stat"
    :benchmark-info="benchmarkInfo"
    :quick-links="importantRuntimeMetrics"
  />
  <OverallSummary :summary="filteredSummary" />
</template>
