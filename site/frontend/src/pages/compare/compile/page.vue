<script setup lang="ts">
import QuickLinks from "./quick-links.vue";
import Filters from "./filters.vue";
import OverallTable from "../summary/overall-table.vue";
import Aggregations from "../summary/aggregations.vue";
import Benchmarks from "../benchmarks/benchmarks.vue";
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

const props = defineProps<{
  data: CompareResponse;
  selector: CompareSelector;
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

/**
 * When the filter changes, the URL is updated.
 * After that happens, we want to re-render the quick links component, because
 * it contains links that are "relative" to the current URL. Changing this
 * key ref will cause it to be re-rendered.
 */
function refreshQuickLinks() {
  quickLinksKey.value += 1;
}

function updateFilter(newFilter: CompileBenchmarkFilter) {
  storeFilterToUrl(newFilter, defaultCompileFilter, getUrlParams());
  filter.value = newFilter;
  refreshQuickLinks();
}

function exportData() {
  exportToMarkdown(testCases.value);
}

const urlParams = getUrlParams();

const quickLinksKey = ref(0);
const filter = ref(loadFilterFromUrl(urlParams, defaultCompileFilter));

const benchmarkMap = createCompileBenchmarkMap(props.data);
const allTestCases = computed(() =>
  computeCompileComparisonsWithNonRelevant(
    filter.value,
    props.data,
    benchmarkMap
  )
);
const testCases = computed(() =>
  filterNonRelevant(filter.value, allTestCases.value)
);
const filteredSummary = computed(() => computeSummary(testCases.value));
</script>

<template>
  <QuickLinks :stat="selector.stat" :key="quickLinksKey" />
  <Filters
    :defaultFilter="defaultCompileFilter"
    :initialFilter="filter"
    @change="updateFilter"
    @export="exportData"
  />
  <OverallTable :summary="filteredSummary" />
  <Aggregations :cases="testCases" />
  <Benchmarks
    :data="data"
    :test-cases="testCases"
    :all-test-cases="allTestCases"
    :filter="filter"
    :stat="selector.stat"
  ></Benchmarks>
</template>
