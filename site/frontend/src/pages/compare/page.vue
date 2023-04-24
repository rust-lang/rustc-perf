<script setup lang="ts">
import {loadBenchmarkInfo} from "../../api";
import AsOf from "../../components/as-of.vue";
import {generateUrlParams, getUrlParams, navigateToUrlParams} from "../../utils/navigation";
import {Ref, ref} from "vue";
import {withLoading} from "../../utils/loading";
import {postMsgpack} from "../../utils/requests";
import {COMPARE_DATA_URL} from "../../urls";
import {CompareResponse, CompareSelector} from "./state";
import BootstrapTable from "./bootstrap-table.vue";
import Header from "./header.vue";
import DataSelector, {SelectionParams} from "./data-selector.vue";

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
  navigateToUrlParams(generateUrlParams({
    start: params.start,
    end: params.end,
    stat: params.stat
  }));
}

let loading = ref(false);

const info = await loadBenchmarkInfo();
const selector = loadSelectorFromUrl(getUrlParams());

let data: Ref<CompareResponse | null> = ref(null);
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
      <BootstrapTable :data="data" />
    </div>
  </div>
  <!--  </div>-->
  <br>
  <AsOf :info="info" />
</template>
