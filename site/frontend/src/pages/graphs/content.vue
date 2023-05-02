<script setup lang="ts">
import {getRequest} from "../../api";
import {nextTick, Ref, ref} from "vue";
import {withLoading} from "../../utils/loading";
import {BenchmarkInfo, GraphData, GraphKind, GraphsSelector} from "./state";
import {GRAPH_DATA_URL, INFO_URL} from "../../urls";
import DataSelector, {SelectionParams} from "./data-selector.vue";
import {getUrlParams} from "../../utils/document";
import {renderPlots} from "./plots";
import {generateUrlParams, navigateToUrlParams} from "../../utils/navigation";

function loadSelectorFromUrl(urlParams: Dict<string>): GraphsSelector {
  const start = urlParams["start"] ?? "";
  const end = urlParams["end"] ?? "";
  const kind: GraphKind = urlParams["kind"] as GraphKind ?? "raw";
  const stat = urlParams["stat"] ?? "instructions:u";
  const benchmark = urlParams["benchmark"] ?? null;
  const scenario = urlParams["scenario"] ?? null;
  const profile = urlParams["profile"] ?? null;
  return {
    start,
    end,
    kind,
    stat,
    benchmark,
    scenario,
    profile,
    exclude_suffix: null,
    include_suffix: null,
  };
}

async function loadGraphData(selector: GraphsSelector, loading: Ref<boolean>) {
  const graphData: GraphData = await withLoading(loading, async () => {
    const params = {
      start: selector.start,
      end: selector.end,
      kind: selector.kind as string,
      stat: selector.stat,
      benchmark: selector.benchmark,
      scenario: selector.scenario,
      profile: selector.profile,
    };
    return await getRequest<GraphData>(GRAPH_DATA_URL, params);
  });

  // Wait for the UI to be updated, which also resets the plot HTML elements.
  // Then draw the plots.
  await nextTick();
  renderPlots(graphData, selector);
}

function updateSelection(params: SelectionParams) {
  navigateToUrlParams(generateUrlParams({
    start: params.start,
    end: params.end,
    kind: params.kind,
    stat: params.stat
  }));
}

const info: BenchmarkInfo = await getRequest<BenchmarkInfo>(INFO_URL);

let loading = ref(true);

const selector: GraphsSelector = loadSelectorFromUrl(getUrlParams());
loadGraphData(selector, loading);
</script>

<template>
  <DataSelector :start="selector.start" :end="selector.end" :kind="selector.kind"
                :stat="selector.stat" :info="info" @change="updateSelection"></DataSelector>
  <div>
    See <a href="/compare.html">compare page</a> for descriptions of what
    the names mean.
  </div>
  <div>
    <strong>Note:</strong> pink in the graphs represent data points that are interpolated
    due to missing data. Interpolated data is simply the last known data point repeated until
    another known data point is found.
  </div>
  <div v-if="loading">
    <h2>Loading &amp; rendering data..</h2>
    <h3>This may take a while!</h3>
  </div>
  <div v-else>
    <div id="charts"></div>
    <div id="as-of">
      Updated as of: {{ new Date(info.as_of).toLocaleString() }}
    </div>
  </div>
  <a href="https://github.com/rust-lang-nursery/rustc-perf">
    <img
      style="position: absolute; top: 0; right: 0; border: 0; clip-path: polygon(8% 0%, 100% 92%, 100% 0%);"
      src="https://camo.githubusercontent.com/38ef81f8aca64bb9a64448d0d70f1308ef5341ab/68747470733a2f2f73332e616d617a6f6e6177732e636f6d2f6769746875622f726962626f6e732f666f726b6d655f72696768745f6461726b626c75655f3132313632312e706e67"
      alt="Fork me on GitHub"
      data-canonical-src="https://s3.amazonaws.com/github/ribbons/forkme_right_darkblue_121621.png">
  </a>
</template>
