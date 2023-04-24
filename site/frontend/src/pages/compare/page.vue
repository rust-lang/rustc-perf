<script setup lang="ts">
import {loadBenchmarkInfo} from "../../api";
import AsOf from "../../components/as-of.vue";
import {getUrlParams} from "../../utils/navigation";
import {Ref, ref} from "vue";
import {withLoading} from "../../utils/loading";
import {postMsgpack} from "../../utils/requests";
import {COMPARE_DATA_URL} from "../../urls";
import {CompareResponse} from "./state";
import BootstrapTable from "./bootstrap-table.vue";

interface CompareSelector {
  start: string;
  end: string;
  stat: string;
}

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
  data.value = await withLoading(loading, async () => {
    const params = {
      start: selector.start,
      end: selector.end,
      stat: selector.stat
    };
    return await postMsgpack<CompareResponse>(COMPARE_DATA_URL, params);
  });
}

let loading = ref(false);

const info = await loadBenchmarkInfo();
const selector = loadSelectorFromUrl(getUrlParams());

const before = "TODO";
const after = "TODO";

let data: Ref<CompareResponse | null> = ref(null);
loadCompareData(selector, loading);
</script>

<template>
  <div id="app">
    <h2>Comparing <span id="stat-header">{{selector.stat}}</span> between <span id="before">{{before}}</span> and
      <span id="after">{{after}}</span>
    </h2>
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
