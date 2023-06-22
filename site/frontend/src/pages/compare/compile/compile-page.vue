<script setup lang="ts">
import MetricSelector from "../metric-selector.vue";
import Filters from "./filters.vue";
import OverallSummary from "../summary/overall-summary.vue";
import Aggregations from "./aggregations.vue";
import Benchmarks from "./benchmarks.vue";
import {CompareResponse, CompareSelector} from "../types";
import {computed, ref} from "vue";
import {changeUrl, getUrlParams} from "../../../utils/navigation";
import {exportToMarkdown} from "./export";
import {computeSummary, filterNonRelevant} from "../data";
import {
  CompileBenchmarkFilter,
  computeCompileComparisonsWithNonRelevant,
  createCompileBenchmarkMap,
  defaultCompileFilter,
} from "./common";
import {BenchmarkInfo} from "../../../api";

const props = defineProps<{
  data: CompareResponse;
  selector: CompareSelector;
  benchmarkInfo: BenchmarkInfo;
}>();

function loadFilterFromUrl(
  urlParams: Dict<string>,
  defaultFilter: CompileBenchmarkFilter
): CompileBenchmarkFilter {
  function getBoolOrDefault(name: string, defaultValue: boolean): boolean {
    if (urlParams.hasOwnProperty(name)) {
      return urlParams[name] === "true";
    }
    return defaultValue;
  }

  return {
    name: urlParams["name"] ?? defaultFilter.name,
    nonRelevant: getBoolOrDefault("nonRelevant", defaultFilter.nonRelevant),
    showRawData: getBoolOrDefault("showRawData", defaultFilter.showRawData),
    profile: {
      check: getBoolOrDefault("check", defaultFilter.profile.check),
      debug: getBoolOrDefault("debug", defaultFilter.profile.debug),
      opt: getBoolOrDefault("opt", defaultFilter.profile.opt),
      doc: getBoolOrDefault("doc", defaultFilter.profile.doc),
    },
    scenario: {
      full: getBoolOrDefault("full", defaultFilter.scenario.full),
      incrFull: getBoolOrDefault("incrFull", defaultFilter.scenario.incrFull),
      incrUnchanged: getBoolOrDefault(
        "incrUnchanged",
        defaultFilter.scenario.incrUnchanged
      ),
      incrPatched: getBoolOrDefault(
        "incrPatched",
        defaultFilter.scenario.incrPatched
      ),
    },
    category: {
      primary: getBoolOrDefault("primary", defaultFilter.category.primary),
      secondary: getBoolOrDefault(
        "secondary",
        defaultFilter.category.secondary
      ),
    },
  };
}

/**
 * Stores the given filter parameters into URL, so that the current "view" can be shared with
 * others easily.
 */
function storeFilterToUrl(
  filter: CompileBenchmarkFilter,
  defaultFilter: CompileBenchmarkFilter,
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

  storeOrReset("name", filter.name || null, defaultFilter.name);
  storeOrReset("nonRelevant", filter.nonRelevant, defaultFilter.nonRelevant);
  storeOrReset("showRawData", filter.showRawData, defaultFilter.showRawData);
  storeOrReset("check", filter.profile.check, defaultFilter.profile.check);
  storeOrReset("debug", filter.profile.debug, defaultFilter.profile.debug);
  storeOrReset("opt", filter.profile.opt, defaultFilter.profile.opt);
  storeOrReset("doc", filter.profile.doc, defaultFilter.profile.doc);
  storeOrReset("full", filter.scenario.full, defaultFilter.scenario.full);
  storeOrReset(
    "incrFull",
    filter.scenario.incrFull,
    defaultFilter.scenario.incrFull
  );
  storeOrReset(
    "incrUnchanged",
    filter.scenario.incrUnchanged,
    defaultFilter.scenario.incrUnchanged
  );
  storeOrReset(
    "incrPatched",
    filter.scenario.incrPatched,
    defaultFilter.scenario.incrPatched
  );
  storeOrReset(
    "primary",
    filter.category.primary,
    defaultFilter.category.primary
  );
  storeOrReset(
    "secondary",
    filter.category.secondary,
    defaultFilter.category.secondary
  );

  changeUrl(urlParams);
}

function updateFilter(newFilter: CompileBenchmarkFilter) {
  storeFilterToUrl(newFilter, defaultCompileFilter, getUrlParams());
  filter.value = newFilter;
}

function exportData() {
  exportToMarkdown(comparisons.value);
}

const urlParams = getUrlParams();

const filter = ref(loadFilterFromUrl(urlParams, defaultCompileFilter));

const benchmarkMap = createCompileBenchmarkMap(props.data);
const allComparisons = computed(() =>
  computeCompileComparisonsWithNonRelevant(
    filter.value,
    props.data.compile_comparisons,
    benchmarkMap
  )
);
const comparisons = computed(() =>
  filterNonRelevant(filter.value, allComparisons.value)
);
const filteredSummary = computed(() => computeSummary(comparisons.value));
</script>

<template>
  <MetricSelector :metric="selector.stat" :benchmark-info="benchmarkInfo" />
  <Filters
    :defaultFilter="defaultCompileFilter"
    :initialFilter="filter"
    @change="updateFilter"
    @export="exportData"
  />
  <OverallSummary :summary="filteredSummary" />
  <Aggregations :cases="comparisons" />
  <Benchmarks
    :data="data"
    :test-cases="comparisons"
    :all-test-cases="allComparisons"
    :filter="filter"
    :stat="selector.stat"
  ></Benchmarks>
</template>
