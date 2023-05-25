<script setup lang="ts">
import DataSelector, {SelectionParams} from "./data-selector.vue";
import {nextTick, Ref, ref} from "vue";
import {withLoading} from "../../utils/loading";
import {renderPlots} from "./plots";
import {BootstrapData, BootstrapSelector} from "./state";
import {BOOTSTRAP_DATA_URL} from "../../urls";
import {
  createUrlWithAppendedParams,
  getUrlParams,
  navigateToUrlParams,
} from "../../utils/navigation";

import {postJson} from "../../utils/requests";

async function loadBootstrapData(
  selector: BootstrapSelector,
  loading: Ref<boolean>
) {
  const bootstrapData: BootstrapData = await withLoading(loading, () =>
    postJson<BootstrapData>(BOOTSTRAP_DATA_URL, selector)
  );

  // Wait for the UI to be updated, which also resets the plot HTML elements.
  // Then draw the plots.
  await nextTick();
  renderPlots(bootstrapData, selector);
}

function loadSelectorFromUrl(urlParams: Dict<string>): BootstrapSelector {
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

const selector: BootstrapSelector = loadSelectorFromUrl(getUrlParams());
loadBootstrapData(selector, loading);
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
    <div id="byCrateChart"></div>
    <div id="totalChart"></div>
  </div>
</template>
