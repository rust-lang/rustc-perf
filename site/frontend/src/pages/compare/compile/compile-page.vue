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
import {importantCompileMetrics} from "../metrics";
import {getBoolOrDefault} from "../shared";

const props = defineProps<{
  data: CompareResponse;
  selector: CompareSelector;
  benchmarkInfo: BenchmarkInfo;
}>();

function loadFilterFromUrl(
  urlParams: Dict<string>,
  defaultFilter: CompileBenchmarkFilter
): CompileBenchmarkFilter {
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
    profile: {
      check: getBoolOrDefault(urlParams, "check", defaultFilter.profile.check),
      debug: getBoolOrDefault(urlParams, "debug", defaultFilter.profile.debug),
      opt: getBoolOrDefault(urlParams, "opt", defaultFilter.profile.opt),
      doc: getBoolOrDefault(urlParams, "doc", defaultFilter.profile.doc),
    },
    scenario: {
      full: getBoolOrDefault(urlParams, "full", defaultFilter.scenario.full),
      incrFull: getBoolOrDefault(
        urlParams,
        "incrFull",
        defaultFilter.scenario.incrFull
      ),
      incrUnchanged: getBoolOrDefault(
        urlParams,
        "incrUnchanged",
        defaultFilter.scenario.incrUnchanged
      ),
      incrPatched: getBoolOrDefault(
        urlParams,
        "incrPatched",
        defaultFilter.scenario.incrPatched
      ),
    },
    backend: {
      llvm: getBoolOrDefault(
        urlParams,
        "backend-llvm",
        defaultFilter.backend.llvm
      ),
      cranelift: getBoolOrDefault(
        urlParams,
        "backend-clif",
        defaultFilter.backend.cranelift
      ),
    },
    category: {
      primary: getBoolOrDefault(
        urlParams,
        "primary",
        defaultFilter.category.primary
      ),
      secondary: getBoolOrDefault(
        urlParams,
        "secondary",
        defaultFilter.category.secondary
      ),
    },
    artifact: {
      binary: getBoolOrDefault(
        urlParams,
        "binary",
        defaultFilter.artifact.binary
      ),
      library: getBoolOrDefault(
        urlParams,
        "library",
        defaultFilter.artifact.library
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
  storeOrReset("backend-llvm", filter.backend.llvm, defaultFilter.backend.llvm);
  storeOrReset(
    "backend-clif",
    filter.backend.cranelift,
    defaultFilter.backend.cranelift
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
  storeOrReset("binary", filter.artifact.binary, defaultFilter.artifact.binary);
  storeOrReset(
    "library",
    filter.artifact.library,
    defaultFilter.artifact.library
  );

  changeUrl(urlParams);
}

function updateFilter(newFilter: CompileBenchmarkFilter) {
  storeFilterToUrl(newFilter, defaultCompileFilter, getUrlParams());
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

function exportData() {
  exportToMarkdown(comparisons.value);
}

const urlParams = getUrlParams();

const quickLinksKey = ref(0);
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
  <MetricSelector
    :key="quickLinksKey"
    :quick-links="importantCompileMetrics"
    :selected-metric="selector.stat"
    :metrics="benchmarkInfo.compile_metrics"
  />
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
    :benchmark-map="benchmarkMap"
  ></Benchmarks>
</template>
