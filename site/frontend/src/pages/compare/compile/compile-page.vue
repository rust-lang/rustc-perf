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
  transformDataForBackendComparison,
} from "./common";
import {BenchmarkInfo, DEFAULT_COMPILE_TARGET_TRIPLE} from "../../../api";
import {importantCompileMetrics} from "../metrics";
import {
  getBoolOrDefault,
  isSameStringArray,
  loadTargetsFromUrl,
  storeOrResetBool,
  storeOrResetStringArray,
} from "../shared";

const props = defineProps<{
  data: CompareResponse;
  selector: CompareSelector;
  benchmarkInfo: BenchmarkInfo;
}>();

function loadFilterFromUrl(
  urlParams: Dict<string>,
  defaultFilter: CompileBenchmarkFilter
): CompileBenchmarkFilter {
  let target = loadTargetsFromUrl(urlParams, defaultFilter.target);
  // If we don't have data for the default target, try to use a present target
  // as the default target filter. This is to provide compatibility for
  // deployment that might have a different default target.
  if (
    isSameStringArray(target, defaultFilter.target) &&
    props.data.compile_comparisons.find(
      (testCase) => testCase.target === DEFAULT_COMPILE_TARGET_TRIPLE
    ) === undefined &&
    props.data.compile_comparisons.length > 0
  ) {
    target = [props.data.compile_comparisons[0].target];
  }

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
    target,
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
    changes: {
      regressions: getBoolOrDefault(
        urlParams,
        "regressions",
        defaultFilter.changes.regressions
      ),
      improvements: getBoolOrDefault(
        urlParams,
        "improvements",
        defaultCompileFilter.changes.improvements
      ),
    },
    selfCompareBackend: getBoolOrDefault(
      urlParams,
      "selfCompareBackend",
      defaultFilter.selfCompareBackend
    ),
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
  storeOrResetBool(urlParams, "name", filter.name || null, defaultFilter.name);
  storeOrResetBool(
    urlParams,
    "nonRelevant",
    filter.nonRelevant,
    defaultFilter.nonRelevant
  );
  storeOrResetBool(
    urlParams,
    "showRawData",
    filter.showRawData,
    defaultFilter.showRawData
  );
  storeOrResetBool(
    urlParams,
    "check",
    filter.profile.check,
    defaultFilter.profile.check
  );
  storeOrResetBool(
    urlParams,
    "debug",
    filter.profile.debug,
    defaultFilter.profile.debug
  );
  storeOrResetBool(
    urlParams,
    "opt",
    filter.profile.opt,
    defaultFilter.profile.opt
  );
  storeOrResetBool(
    urlParams,
    "doc",
    filter.profile.doc,
    defaultFilter.profile.doc
  );
  storeOrResetBool(
    urlParams,
    "full",
    filter.scenario.full,
    defaultFilter.scenario.full
  );
  storeOrResetBool(
    urlParams,
    "incrFull",
    filter.scenario.incrFull,
    defaultFilter.scenario.incrFull
  );
  storeOrResetBool(
    urlParams,
    "incrUnchanged",
    filter.scenario.incrUnchanged,
    defaultFilter.scenario.incrUnchanged
  );
  storeOrResetBool(
    urlParams,
    "incrPatched",
    filter.scenario.incrPatched,
    defaultFilter.scenario.incrPatched
  );
  storeOrResetBool(
    urlParams,
    "backend-llvm",
    filter.backend.llvm,
    defaultFilter.backend.llvm
  );
  storeOrResetBool(
    urlParams,
    "backend-clif",
    filter.backend.cranelift,
    defaultFilter.backend.cranelift
  );
  storeOrResetStringArray(
    urlParams,
    "target",
    filter.target,
    defaultFilter.target
  );
  storeOrResetBool(
    urlParams,
    "primary",
    filter.category.primary,
    defaultFilter.category.primary
  );
  storeOrResetBool(
    urlParams,
    "secondary",
    filter.category.secondary,
    defaultFilter.category.secondary
  );
  storeOrResetBool(
    urlParams,
    "binary",
    filter.artifact.binary,
    defaultFilter.artifact.binary
  );
  storeOrResetBool(
    urlParams,
    "library",
    filter.artifact.library,
    defaultFilter.artifact.library
  );
  storeOrResetBool(
    urlParams,
    "regressions",
    filter.changes.regressions,
    defaultFilter.changes.regressions
  );
  storeOrResetBool(
    urlParams,
    "improvements",
    filter.changes.improvements,
    defaultFilter.changes.improvements
  );
  storeOrResetBool(
    urlParams,
    "selfCompareBackend",
    filter.selfCompareBackend,
    defaultFilter.selfCompareBackend
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

const urlParams = getUrlParams();

const quickLinksKey = ref(0);
const filter = ref(loadFilterFromUrl(urlParams, defaultCompileFilter));

// Should we use the backend as the source of before/after data?
const selfCompareBackend = computed(() => {
  return canCompareBackends.value && filter.value.selfCompareBackend;
});
const canCompareBackends = computed(() => {
  const hasMultipleBackends =
    new Set(props.data.compile_comparisons.map((c) => c.backend)).size > 1;
  // Are we currently comparing the same commit in the before/after toolchains?
  const comparesSameCommit = props.data.a.commit === props.data.b.commit;
  return hasMultipleBackends && comparesSameCommit;
});

function exportData() {
  exportToMarkdown(comparisons.value, filter.value.showRawData);
}

const benchmarkMap = createCompileBenchmarkMap(props.data);

const compileComparisons = computed(() => {
  // If requested, artificially restructure the data to create a comparison between backends
  if (selfCompareBackend.value) {
    return transformDataForBackendComparison(props.data.compile_comparisons);
  } else {
    return props.data.compile_comparisons;
  }
});
const allComparisons = computed(() =>
  computeCompileComparisonsWithNonRelevant(
    filter.value,
    compileComparisons.value,
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
    :info="benchmarkInfo"
    :default-filter="defaultCompileFilter"
    :initial-filter="filter"
    :can-compare-backends="canCompareBackends"
    @change="updateFilter"
    @export="exportData"
  />
  <OverallSummary :summary="filteredSummary" />
  <Aggregations :cases="comparisons" />
  <div class="warning" v-if="selfCompareBackend">
    Note: comparing results of the baseline LLVM backend to the Cranelift
    backend.
  </div>
  <Benchmarks
    :data="data"
    :test-cases="comparisons"
    :all-test-cases="allComparisons"
    :filter="filter"
    :stat="selector.stat"
    :benchmark-map="benchmarkMap"
    :show-backend="!selfCompareBackend"
  ></Benchmarks>
</template>
<style lang="scss" scoped>
.warning {
  color: red;
  font-weight: bold;
}
</style>
