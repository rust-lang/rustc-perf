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
import {computeSummary, filterNonRelevant, SummaryGroup} from "./data";
import Tabs from "./tabs.vue";
import CompileBenchmarksPage from "./compile/compile-page.vue";
import {
  computeCompileComparisonsWithNonRelevant,
  createCompileBenchmarkMap,
  defaultCompileFilter as defaultCompileFilter,
} from "./compile/common";
import RuntimeBenchmarksPage from "./runtime/runtime-page.vue";
import {
  computeRuntimeComparisonsWithNonRelevant,
  defaultRuntimeFilter,
} from "./runtime/common";
import ArtifactSizeTable from "./artifact-size/artifact-size-table.vue";

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
    ["artifact-size"]: Tab.ArtifactSize,
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
      computeCompileComparisonsWithNonRelevant(
        defaultCompileFilter,
        response.compile_comparisons,
        createCompileBenchmarkMap(response)
      )
    )
  );
  runtimeSummary.value = computeSummary(
    filterNonRelevant(
      defaultRuntimeFilter,
      computeRuntimeComparisonsWithNonRelevant(
        defaultRuntimeFilter,
        response.runtime_comparisons
      )
    )
  );
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

// Include all relevant changes in the compile-time and runtime tab summaries.
// We do not wrap these summaries in `computed`, because they should be loaded
// only once, after the compare data is downloaded.
const compileSummary: Ref<SummaryGroup | null> = ref(null);
const runtimeSummary: Ref<SummaryGroup | null> = ref(null);

const loading = ref(false);

const selector = loadSelectorFromUrl(urlParams);

const initialTab: Tab = loadTabFromUrl(urlParams) ?? Tab.CompileTime;
const tab: Ref<Tab> = ref(initialTab);
const activeTab = computed((): Tab => {
  if (tab.value === Tab.ArtifactSize && !artifactSizeAvailable.value) {
    return Tab.CompileTime;
  }
  return tab.value;
});

const artifactSizeAvailable = computed(
  () =>
    data.value != null &&
    (Object.keys(data.value.a.component_sizes).length > 0 ||
      Object.keys(data.value.b.component_sizes).length > 0)
);

function changeTab(newTab: Tab) {
  tab.value = newTab;
  storeTabToUrl(getUrlParams(), newTab);
}

const data: Ref<CompareResponse | null> = ref(null);
loadCompareData(selector, loading);
let info = await loadBenchmarkInfo();
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
        :runtime-summary="runtimeSummary"
      />
      <template v-if="activeTab === Tab.CompileTime">
        <CompileBenchmarksPage
          :data="data"
          :selector="selector"
          :benchmark-info="info"
        />
      </template>
      <template v-if="activeTab === Tab.Runtime">
        <RuntimeBenchmarksPage
          :data="data"
          :selector="selector"
          :benchmark-info="info"
        />
      </template>
      <BootstrapTable v-if="activeTab === Tab.Bootstrap" :data="data" />
      <template v-if="artifactSizeAvailable && activeTab === Tab.ArtifactSize">
        <ArtifactSizeTable :a="data.a" :b="data.b" />
      </template>
    </div>
  </div>
  <br />
  <AsOf :info="info" />
</template>
