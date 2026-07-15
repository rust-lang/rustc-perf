<script setup lang="ts">
import DataSelector, {SelectionParams} from "./data-selector.vue";
import {nextTick, Ref, ref} from "vue";
import {withLoading} from "../../utils/loading";
import {renderPlots} from "./plots";
import {ToolchainData, ToolchainSelector} from "./state";
import {TOOLCHAIN_DATA_URL} from "../../urls";
import {
  createUrlWithAppendedParams,
  getUrlParams,
  navigateToUrlParams,
} from "../../utils/navigation";

import {postJson} from "../../utils/requests";

async function loadToolchainData(
  selector: ToolchainSelector,
  loading: Ref<boolean>
) {
  const data: ToolchainData = await withLoading(loading, () =>
    postJson<ToolchainData>(TOOLCHAIN_DATA_URL, selector)
  );

  // Wait for the UI to be updated, which also resets the plot HTML elements.
  // Then draw the plots.
  await nextTick();
  renderPlots(data, selector);
}

function loadSelectorFromUrl(urlParams: Dict<string>): ToolchainSelector {
  const start = urlParams["start"] ?? "";
  const end = urlParams["end"] ?? "";
  return {
    start,
    end,
    min_seconds: 25,
  };
}

function updateSelection(params: SelectionParams) {
  navigateToUrlParams(
    createUrlWithAppendedParams({
      start: params.start,
      end: params.end,
    }).searchParams
  );
}

const loading = ref(true);

const selector: ToolchainSelector = loadSelectorFromUrl(getUrlParams());
loadToolchainData(selector, loading);
</script>

<template>
  <DataSelector
    :start="selector.start"
    :end="selector.end"
    @change="updateSelection"
  ></DataSelector>
  <div v-if="loading" id="loading">
    <h2>Loading &amp; rendering data..</h2>
    <h3>This may take a while!</h3>
  </div>
  <div v-else>
    <div>
      See <a href="/compare.html">compare page</a> for descriptions of what the
      names mean.
    </div>
    <div id="artifactSizeChart"></div>
    <div id="bootstrapByCrateChart"></div>
    <div id="bootstrapTotalChart"></div>
  </div>
</template>
