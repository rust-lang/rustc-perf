<script setup lang="ts">
import {loadBenchmarkInfo} from "../../api";
import AsOf from "../../components/as-of.vue";
import {getUrlParams} from "../../utils/navigation";
import {computed, Ref, ref} from "vue";
import {withLoading} from "../../utils/loading";
import {postMsgpack} from "../../utils/requests";
import {COMPARE_DATA_URL} from "../../urls";
import {ArtifactDescription, CompareResponse, CompareSelector} from "./state";
import BootstrapTable from "./bootstrap-table.vue";
import Header from "./header.vue";
import {formatDate} from "./shared";

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
      <BootstrapTable :data="data" />
    </div>
  </div>
  <!--  </div>-->
  <br>
  <AsOf :info="info" />
</template>
