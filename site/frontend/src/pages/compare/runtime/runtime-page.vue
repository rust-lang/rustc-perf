<script setup lang="ts">
import {CompareResponse, CompareSelector} from "../types";
import OverallSummary from "../summary/overall-summary.vue";
import {computed, ref} from "vue";
import {computeSummary, filterNonRelevant} from "../data";
import {
  computeRuntimeComparisonsWithNonRelevant,
  defaultRuntimeFilter,
  RuntimeBenchmarkFilter,
} from "./common";
import MetricSelector from "../metric-selector.vue";
import {BenchmarkInfo} from "../../../api";
import {importantRuntimeMetrics} from "../metrics";
import ComparisonsTable from "./comparisons-table.vue";
import {getBoolOrDefault} from "../shared";
import {getUrlParams} from "../../../utils/navigation";

const props = defineProps<{
  data: CompareResponse;
  selector: CompareSelector;
  benchmarkInfo: BenchmarkInfo;
}>();

function loadFilterFromUrl(
  urlParams: Dict<string>,
  defaultFilter: RuntimeBenchmarkFilter
): RuntimeBenchmarkFilter {
  return {
    name: urlParams["name"] ?? defaultFilter.name,
    nonRelevant: getBoolOrDefault(
      urlParams,
      "nonRelevant",
      defaultFilter.nonRelevant
    ),
    showRawData: getBoolOrDefault(
      urlParams,
      "showRawData",
      defaultFilter.showRawData
    ),
  };
}

const urlParams = getUrlParams();
const filter = ref(loadFilterFromUrl(urlParams, defaultRuntimeFilter));

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
  <ComparisonsTable
    :comparisons="comparisons"
    :has-non-relevant="allComparisons.length > 0"
    :show-raw-data="filter.showRawData"
  />
</template>
