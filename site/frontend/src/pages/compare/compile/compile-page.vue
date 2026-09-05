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
  SelfCompareData,
  transformDataForSelfComparison,
} from "./common";
import {BenchmarkInfo, DEFAULT_COMPILE_TARGET_TRIPLE} from "../../../api";
import {importantCompileMetrics} from "../metrics";
import {
  getBoolOrDefault,
  isSameStringArray,
  loadTargetsFromUrl,
  storeOrResetValue,
  storeOrResetStringArray,
  getStringOrDefault,
  getStringArrayOrDefault,
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

  const frontendThreads = getStringArrayOrDefault(
    urlParams,
    "frontendThreads",
    defaultFilter.frontendThreads
  );

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
      docJson: getBoolOrDefault(
        urlParams,
        "doc-json",
        defaultFilter.profile.docJson
      ),
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
    frontendThreads,
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
    selfCompareParameter: getStringOrDefault(
      urlParams,
      "selfCompareParameter",
      defaultFilter.selfCompareParameter
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
  storeOrResetValue(urlParams, "name", filter.name, defaultFilter.name);
  storeOrResetValue(
    urlParams,
    "nonRelevant",
    filter.nonRelevant,
    defaultFilter.nonRelevant
  );
  storeOrResetValue(
    urlParams,
    "showRawData",
    filter.showRawData,
    defaultFilter.showRawData
  );
  storeOrResetValue(
    urlParams,
    "check",
    filter.profile.check,
    defaultFilter.profile.check
  );
  storeOrResetValue(
    urlParams,
    "debug",
    filter.profile.debug,
    defaultFilter.profile.debug
  );
  storeOrResetValue(
    urlParams,
    "opt",
    filter.profile.opt,
    defaultFilter.profile.opt
  );
  storeOrResetValue(
    urlParams,
    "doc",
    filter.profile.doc,
    defaultFilter.profile.doc
  );
  storeOrResetValue(
    urlParams,
    "doc-json",
    filter.profile.docJson,
    defaultFilter.profile.docJson
  );
  storeOrResetValue(
    urlParams,
    "full",
    filter.scenario.full,
    defaultFilter.scenario.full
  );
  storeOrResetValue(
    urlParams,
    "incrFull",
    filter.scenario.incrFull,
    defaultFilter.scenario.incrFull
  );
  storeOrResetValue(
    urlParams,
    "incrUnchanged",
    filter.scenario.incrUnchanged,
    defaultFilter.scenario.incrUnchanged
  );
  storeOrResetValue(
    urlParams,
    "incrPatched",
    filter.scenario.incrPatched,
    defaultFilter.scenario.incrPatched
  );
  storeOrResetValue(
    urlParams,
    "backend-llvm",
    filter.backend.llvm,
    defaultFilter.backend.llvm
  );
  storeOrResetValue(
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
  storeOrResetStringArray(
    urlParams,
    "frontendThreads",
    filter.frontendThreads,
    defaultFilter.frontendThreads
  );
  storeOrResetValue(
    urlParams,
    "primary",
    filter.category.primary,
    defaultFilter.category.primary
  );
  storeOrResetValue(
    urlParams,
    "secondary",
    filter.category.secondary,
    defaultFilter.category.secondary
  );
  storeOrResetValue(
    urlParams,
    "binary",
    filter.artifact.binary,
    defaultFilter.artifact.binary
  );
  storeOrResetValue(
    urlParams,
    "library",
    filter.artifact.library,
    defaultFilter.artifact.library
  );
  storeOrResetValue(
    urlParams,
    "regressions",
    filter.changes.regressions,
    defaultFilter.changes.regressions
  );
  storeOrResetValue(
    urlParams,
    "improvements",
    filter.changes.improvements,
    defaultFilter.changes.improvements
  );
  storeOrResetValue(
    urlParams,
    "selfCompareParameter",
    filter.selfCompareParameter,
    defaultFilter.selfCompareParameter
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

const selfCompareCanBeEnabled = computed(() => {
  // Are we currently comparing the same commit in the before/after toolchains?
  return props.data.a.commit === props.data.b.commit;
});

// Should we use a given benchmark parameter as the source of before/after data?
const selfCompareData = computed((): SelfCompareData | null => {
  if (!selfCompareCanBeEnabled.value) return null;

  const selfCompare = filter.value.selfCompareParameter;
  if (selfCompare === null) return null;

  if (selfCompare === "backend") {
    return {
      parameter: "backend",
      baseline: "llvm",
    };
  } else if (selfCompare === "target") {
    return {
      parameter: "target",
      baseline: DEFAULT_COMPILE_TARGET_TRIPLE,
    };
  } else {
    return null;
  }
});

function exportData() {
  exportToMarkdown(comparisons.value, filter.value.showRawData);
}

const benchmarkMap = createCompileBenchmarkMap(props.data);

const compileComparisons = computed(() => {
  // If requested, artificially restructure the data to create a comparison
  // between the selected benchmark parameter
  const selfCompare = selfCompareData.value;
  if (selfCompare !== null) {
    return transformDataForSelfComparison(
      props.data.compile_comparisons,
      selfCompare
    );
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
    :self-compare-enabled="selfCompareCanBeEnabled"
    :all-comparisons="allComparisons"
    @change="updateFilter"
    @export="exportData"
  />
  <OverallSummary :summary="filteredSummary" />
  <Aggregations :cases="comparisons" />
  <div class="warning" v-if="selfCompareData !== null">
    Note: comparing results against the baseline {{ selfCompareData.baseline }}
    {{ selfCompareData.parameter }}.
  </div>
  <Benchmarks
    :data="data"
    :test-cases="comparisons"
    :all-test-cases="allComparisons"
    :filter="filter"
    :stat="selector.stat"
    :benchmark-map="benchmarkMap"
  ></Benchmarks>
</template>
<style lang="scss" scoped>
.warning {
  color: red;
  font-weight: bold;
}
</style>
