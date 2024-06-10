<script setup lang="ts">
import {nextTick, Ref, ref} from "vue";
import {withLoading} from "../../utils/loading";
import {CompileGraphData, GraphKind, GraphsSelector} from "../../graph/data";
import DataSelector, {SelectionParams} from "./data-selector.vue";
import {
  createUrlWithAppendedParams,
  getUrlParams,
  navigateToUrlParams,
} from "../../utils/navigation";
import {renderPlots} from "../../graph/render";
import {BenchmarkInfo, loadBenchmarkInfo} from "../../api";
import AsOf from "../../components/as-of.vue";
import {loadGraphs} from "../../graph/api";

function loadSelectorFromUrl(urlParams: Dict<string>): GraphsSelector {
  const start = urlParams["start"] ?? "";
  const end = urlParams["end"] ?? "";
  const kind: GraphKind = (urlParams["kind"] as GraphKind) ?? "raw";
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
  };
}

function filterBenchmarks(
  data: CompileGraphData,
  filter: (key: string) => boolean
): CompileGraphData {
  const benchmarks = Object.fromEntries(
    Object.entries(data.benchmarks).filter(([key, _]) => filter(key))
  );
  return {
    ...data,
    benchmarks,
  };
}

/*
 * Returns true if a specific subset of charts is selected by benchmark
 * name, profile or scenario. In that case, the regular aligned grid of charts
 * will not be shown.
 */
function hasSpecificSelection(selector: GraphsSelector): boolean {
  return (
    selector.benchmark !== null ||
    selector.profile !== null ||
    selector.scenario !== null
  );
}

async function loadGraphData(selector: GraphsSelector, loading: Ref<boolean>) {
  const graphData: CompileGraphData = await withLoading(
    loading,
    async () => await loadGraphs(selector)
  );

  // Wait for the UI to be updated, which also resets the plot HTML elements.
  // Then draw the plots.
  await nextTick();

  const width = Math.max(Math.floor(window.innerWidth / 4) - 40, 380);
  const opts = {width};

  // If we select a smaller subset of benchmarks, then just show them.
  if (hasSpecificSelection(selector)) {
    renderPlots(graphData, selector, document.getElementById("charts"), opts);
  } else {
    // If we select all of them, we expect that there will be a regular grid.

    // So, first render everything but the less important benchmarks about artifact sizes.
    // This keeps the grouping and alignment of 4 charts per row where all 4 charts are about a
    // given benchmark. So, we exclude the benchmarks ending in "-tiny".
    const withoutTiny = filterBenchmarks(
      graphData,
      (benchName) => !benchName.endsWith("-tiny")
    );
    renderPlots(withoutTiny, selector, document.getElementById("charts"), opts);

    // Then, render only the size-related ones in their own dedicated section as they are less
    // important than having the better grouping. So, we only include the benchmarks ending in
    // "-tiny" and render them in the appropriate section.
    const onlyTiny = filterBenchmarks(graphData, (benchName) =>
      benchName.endsWith("-tiny")
    );
    renderPlots(
      onlyTiny,
      selector,
      document.getElementById("size-charts"),
      opts
    );
  }
}

function updateSelection(params: SelectionParams) {
  navigateToUrlParams(
    createUrlWithAppendedParams({
      start: params.start,
      end: params.end,
      kind: params.kind,
      stat: params.stat,
    }).searchParams
  );
}

const info: BenchmarkInfo = await loadBenchmarkInfo();

const loading = ref(true);

const selector: GraphsSelector = loadSelectorFromUrl(getUrlParams());
loadGraphData(selector, loading);
</script>

<template>
  <DataSelector
    :start="selector.start"
    :end="selector.end"
    :kind="selector.kind"
    :stat="selector.stat"
    :info="info"
    @change="updateSelection"
  ></DataSelector>
  <div>
    See <a href="/compare.html">compare page</a> for descriptions of what the
    names mean.
  </div>
  <div>
    <strong>Note:</strong> pink in the graphs represent data points that are
    interpolated due to missing data. Interpolated data is simply the last known
    data point repeated until another known data point is found.
  </div>
  <div v-if="loading">
    <h2>Loading &amp; rendering data..</h2>
    <h3>This may take a while!</h3>
  </div>
  <div v-else>
    <div id="charts"></div>
    <div
      v-if="!hasSpecificSelection(selector)"
      style="margin-top: 50px; border-top: 1px solid #ccc"
    >
      <div style="padding: 20px 0">
        <strong>Benchmarks optimized for small binary size</strong>
      </div>
      <div id="size-charts"></div>
    </div>
    <AsOf :info="info" />
  </div>
</template>
