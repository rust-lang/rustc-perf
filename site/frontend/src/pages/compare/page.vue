<script setup lang="ts">
import {loadBenchmarkInfo} from "../../api";
import AsOf from "../../components/as-of.vue";
import {
  changeUrl,
  createUrlWithAppendedParams,
  getUrlParams,
  navigateToUrlParams,
} from "../../utils/navigation";
import {computed, Ref, ref} from "vue";
import {withLoading} from "../../utils/loading";
import {postMsgpack} from "../../utils/requests";
import {COMPARE_DATA_URL} from "../../urls";
import {CompareResponse, CompareSelector, Tab} from "./types";
import BootstrapTable from "./bootstrap/bootstrap-table.vue";
import Header from "./header/header.vue";
import DataSelector, {SelectionParams} from "./header/data-selector.vue";
import {
  computeSummary,
  computeTestCasesWithNonRelevant,
  createCompileBenchmarkMap,
  filterNonRelevant,
  SummaryGroup,
} from "./data";
import Tabs from "./tabs.vue";
import CompileBenchmarksPage from "./compile/page.vue";
import {defaultFilter as defaultCompileFilter} from "./compile/common";

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

function loadTabFromUrl(urlParams: Dict<string>): Tab | null {
  const tab = urlParams["tab"] ?? "";
  const tabs = {
    compile: Tab.CompileTime,
    runtime: Tab.Runtime,
    bootstrap: Tab.Bootstrap,
  };
  return tabs[tab] ?? null;
}

function storeTabToUrl(urlParams: Dict<string>, tab: Tab) {
  urlParams["tab"] = tab as string;
  changeUrl(urlParams);
}

async function loadCompareData(
  selector: CompareSelector,
  loading: Ref<boolean>
) {
  const response = await withLoading(loading, async () => {
    const params = {
      start: selector.start,
      end: selector.end,
      stat: selector.stat,
    };
    return await postMsgpack<CompareResponse>(COMPARE_DATA_URL, params);
  });
  data.value = response;

  compileSummary.value = computeSummary(
    filterNonRelevant(
      defaultCompileFilter,
      computeTestCasesWithNonRelevant(
        defaultCompileFilter,
        response,
        createCompileBenchmarkMap(response)
      )
    )
  );
  runtimeDataAvailable.value = response.runtime_comparisons.length > 0;
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

const urlParams = getUrlParams();

// Include all relevant changes in the compile-time tab summary.
// We do not wrap it in computed, because it should be loaded only once, after
// the data is downloaded.
const compileSummary: Ref<SummaryGroup | null> = ref(null);

const loading = ref(false);

const info = await loadBenchmarkInfo();
const selector = loadSelectorFromUrl(urlParams);

const initialTab: Tab = loadTabFromUrl(urlParams) ?? Tab.CompileTime;
const tab: Ref<Tab> = ref(initialTab);
const activeTab = computed((): Tab => {
  if (tab.value === Tab.Runtime && !runtimeDataAvailable.value) {
    return Tab.CompileTime;
  }
  return tab.value;
});

const runtimeDataAvailable = ref(false);

function changeTab(newTab: Tab) {
  tab.value = newTab;
  storeTabToUrl(getUrlParams(), newTab);
}

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
      <Tabs
        @change-tab="changeTab"
        :data="data"
        :initial-tab="initialTab"
        :compile-time-summary="compileSummary"
      />
      <template v-if="activeTab === Tab.CompileTime">
        <CompileBenchmarksPage :data="data" :selector="selector" />
      </template>
      <BootstrapTable v-if="activeTab === Tab.Bootstrap" :data="data" />
      <template v-if="runtimeDataAvailable && activeTab === Tab.Runtime">
        Runtime data
      </template>
    </div>
  </div>
  <br />
  <AsOf :info="info" />
</template>
