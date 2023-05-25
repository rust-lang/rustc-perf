<script setup lang="ts">
import {loadBenchmarkInfo} from "../../api";
import AsOf from "../../components/as-of.vue";
import {
  createUrlFromParams,
  createUrlWithAppendedParams,
  getUrlParams,
  navigateToUrlParams,
} from "../../utils/navigation";
import {computed, Ref, ref} from "vue";
import {withLoading} from "../../utils/loading";
import {postMsgpack} from "../../utils/requests";
import {COMPARE_DATA_URL} from "../../urls";
import {CompareResponse, CompareSelector, DataFilter} from "./types";
import BootstrapTable from "./bootstrap-table.vue";
import Header from "./header/header.vue";
import DataSelector, {SelectionParams} from "./header/data-selector.vue";
import QuickLinks from "./header/quick-links.vue";
import Filters from "./header/filters.vue";
import {exportToMarkdown} from "./export";
import {
  computeBenchmarkMap,
  computeSummary,
  computeTestCasesWithNonRelevant,
  filterNonRelevant,
} from "./data";
import OverallTable from "./summary/overall-table.vue";
import Aggregations from "./summary/aggregations.vue";
import Benchmarks from "./benchmarks/benchmarks.vue";

function loadSelectorFromUrl(urlParams: Dict<string>): CompareSelector {
  const start = urlParams["start"] ?? "";
  const end = urlParams["end"] ?? "";
  const stat = urlParams["stat"] ?? "instructions:u";
  return {
    start,
    end,
    stat,
  };
}

function loadFilterFromUrl(
  urlParams: Dict<string>,
  defaultFilter: DataFilter
): DataFilter {
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
  filter: DataFilter,
  defaultFilter: DataFilter,
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

  // Change URL without creating a history entry
  if (history.replaceState) {
    history.replaceState({}, null, createUrlFromParams(urlParams).toString());
  }
}

async function loadCompareData(
  selector: CompareSelector,
  loading: Ref<boolean>
) {
  const response: CompareResponse = await withLoading(loading, async () => {
    const params = {
      start: selector.start,
      end: selector.end,
      stat: selector.stat,
    };
    return await postMsgpack<CompareResponse>(COMPARE_DATA_URL, params);
  });
  data.value = response;
}

function updateSelection(params: SelectionParams) {
  navigateToUrlParams(
    createUrlWithAppendedParams({
      start: params.start,
      end: params.end,
      stat: params.stat,
    }).searchParams
  );
}

function updateFilter(newFilter: DataFilter) {
  filter.value = newFilter;
  storeFilterToUrl(newFilter, defaultFilter, getUrlParams());
}

function exportData() {
  exportToMarkdown(testCases.value);
}

const urlParams = getUrlParams();

const defaultFilter: DataFilter = {
  name: null,
  nonRelevant: false,
  showRawData: false,
  profile: {
    check: true,
    debug: true,
    opt: true,
    doc: true,
  },
  scenario: {
    full: true,
    incrFull: true,
    incrUnchanged: true,
    incrPatched: true,
  },
  category: {
    primary: true,
    secondary: true,
  },
};
const benchmarkMap = computed(() => computeBenchmarkMap(data.value));
const allTestCases = computed(() =>
  computeTestCasesWithNonRelevant(filter.value, data.value, benchmarkMap.value)
);
const testCases = computed(() =>
  filterNonRelevant(filter.value, allTestCases.value)
);
const filteredSummary = computed(() => computeSummary(testCases.value));

const loading = ref(false);

const info = await loadBenchmarkInfo();
const selector = loadSelectorFromUrl(urlParams);
const filter = ref(loadFilterFromUrl(urlParams, defaultFilter));

const data: Ref<CompareResponse | null> = ref(null);
loadCompareData(selector, loading);
</script>

<template>
  <div>
    <Header :data="data" :selector="selector" />
    <DataSelector
      :start="selector.start"
      :end="selector.end"
      :stat="selector.stat"
      :info="info"
      @change="updateSelection"
    />
    <div v-if="loading">
      <p>Loading ...</p>
    </div>
    <div v-if="data !== null">
      <QuickLinks :stat="selector.stat" />
      <Filters
        :defaultFilter="defaultFilter"
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
      <BootstrapTable :data="data" />
    </div>
  </div>
  <br />
  <AsOf :info="info" />
</template>
