<script setup lang="ts">
import {loadBenchmarkInfo} from "../../api";
import AsOf from "../../components/as-of.vue";
import {createUrlParams, getUrlParams, navigateToUrlParams} from "../../utils/navigation";
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
import {computeBenchmarkMap, computeSummary, computeTestCases} from "./data";
import OverallTable from "./summary/overall-table.vue";
import Aggregations from "./summary/aggregations.vue";

// TODO: reset defaults
function loadSelectorFromUrl(urlParams: Dict<string>): CompareSelector {
  const start = urlParams["start"] ?? "2021-05-01";
  const end = urlParams["end"] ?? "2021-06-01";
  const stat = urlParams["stat"] ?? "instructions:u";
  return {
    start,
    end,
    stat
  };
}

async function loadCompareData(selector: CompareSelector, loading: Ref<boolean>) {
  const response: CompareResponse = await withLoading(loading, async () => {
    const params = {
      start: selector.start,
      end: selector.end,
      stat: selector.stat
    };
    return await postMsgpack<CompareResponse>(COMPARE_DATA_URL, params);
  });
  data.value = response;
}

function updateSelection(params: SelectionParams) {
  navigateToUrlParams(createUrlParams({
    start: params.start,
    end: params.end,
    stat: params.stat
  }));
}

function exportData() {
  exportToMarkdown(testCases.value);
}

const defaultFilter: DataFilter = {
  name: null,
  nonRelevant: false,
  showRawData: false,
  profile: {
    check: true,
    debug: true,
    opt: true,
    doc: true
  },
  scenario: {
    full: true,
    incrFull: true,
    incrUnchanged: true,
    incrPatched: true
  },
  category: {
    primary: true,
    secondary: true
  }
};
const benchmarkMap = computed(() => computeBenchmarkMap(data.value));
const testCases = computed(() => computeTestCases(filter.value, data.value, benchmarkMap.value));
const filteredSummary = computed(() => computeSummary(testCases.value));

const loading = ref(false);

const info = await loadBenchmarkInfo();
const selector = loadSelectorFromUrl(getUrlParams());
const filter = ref({...defaultFilter});

const data: Ref<CompareResponse | null> = ref(null);
loadCompareData(selector, loading);
</script>

<template>
  <div>
    <Header :data="data" :selector="selector" />
    <div v-if="loading || data === null">
      <p>Loading ...</p>
    </div>
    <div v-else>
      <DataSelector :start="selector.start" :end="selector.end" :stat="selector.stat"
                    :info="info" @change="updateSelection" />
      <QuickLinks :stat="selector.stat" />
      <Filters :defaultFilter="defaultFilter" @change="f => filter = f"
               @export="exportData" />
      <OverallTable :summary="filteredSummary" />
      <Aggregations :cases="testCases" />
      <BootstrapTable :data="data" />
    </div>
  </div>
  <!--  </div>-->
  <br>
  <AsOf :info="info" />
</template>
