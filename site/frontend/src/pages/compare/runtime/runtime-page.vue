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
import {changeUrl, getUrlParams} from "../../../utils/navigation";
import Filters from "./filters.vue";

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
    name: urlParams["runtimeName"] ?? defaultFilter.name,
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

/**
 * Stores the given filter parameters into URL, so that the current "view" can be shared with
 * others easily.
 */
function storeFilterToUrl(
  filter: RuntimeBenchmarkFilter,
  defaultFilter: RuntimeBenchmarkFilter,
  urlParams: Dict<string>
) {
  function storeOrReset<T extends boolean | string>(
    name: string,
    value: T,
    defaultValue: T
  ) {
    if (value === defaultValue) {
      if (urlParams.hasOwnProperty(name)) {
        delete urlParams[name];
      }
    } else {
      urlParams[name] = value.toString();
    }
  }

  storeOrReset("runtimeName", filter.name || null, defaultFilter.name);
  storeOrReset("nonRelevant", filter.nonRelevant, defaultFilter.nonRelevant);
  storeOrReset("showRawData", filter.showRawData, defaultFilter.showRawData);
  changeUrl(urlParams);
}

function updateFilter(newFilter: RuntimeBenchmarkFilter) {
  storeFilterToUrl(newFilter, defaultRuntimeFilter, getUrlParams());
  filter.value = newFilter;
  refreshQuickLinks();
}

/**
 * When the filter changes, the URL is updated.
 * After that happens, we want to re-render the quick links component, because
 * it contains links that are "relative" to the current URL. Changing this
 * key ref will cause it to be re-rendered.
 */
function refreshQuickLinks() {
  quickLinksKey.value += 1;
}

const urlParams = getUrlParams();

const quickLinksKey = ref(0);
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
    :key="quickLinksKey"
    :quick-links="importantRuntimeMetrics"
    :selected-metric="selector.stat"
    :metrics="benchmarkInfo.runtime_metrics"
  />
  <Filters
    :defaultFilter="defaultRuntimeFilter"
    :initialFilter="filter"
    @change="updateFilter"
  />
  <OverallSummary :summary="filteredSummary" />
  <div class="warning">
    Runtime benchmarks are currently experimental and the results might be quite
    unstable/noisy. Please take this into account!
  </div>
  <ComparisonsTable
    :comparisons="comparisons"
    :has-non-relevant="allComparisons.length > 0"
    :show-raw-data="filter.showRawData"
    :metric="selector.stat"
    :commitA="data.a"
    :commitB="data.b"
  />
</template>

<style scoped lang="scss">
.warning {
  margin: 5px 0;
  text-align: center;
  font-weight: bold;
}
</style>
