<script setup lang="ts">
import {
  CargoProfileMetadata,
  CompileBenchmarkMap,
  CompileBenchmarkMetadata,
  CompileTestCase,
} from "../common";
import {computed, onMounted, Ref, ref} from "vue";
import Tooltip from "../../tooltip.vue";
import {ArtifactDescription} from "../../types";
import {getDateInPast} from "./utils";
import {renderPlots} from "../../../../graph/render";
import {GRAPH_RESOLVER} from "../../../../graph/resolver";
import {GraphKind} from "../../../../graph/data";

const props = defineProps<{
  testCase: CompileTestCase;
  metric: string;
  artifact: ArtifactDescription;
  benchmarkMap: CompileBenchmarkMap;
}>();

async function renderGraph() {
  const selector = {
    benchmark: props.testCase.benchmark,
    profile: props.testCase.profile,
    scenario: props.testCase.scenario,
    stat: props.metric,
    start: getDateInPast(props.artifact),
    end: props.artifact.commit,
    // We want to be able to see noise "blips" vs. a previous artifact.
    // The "percent relative from previous commit" graph should be the best to
    // see these kinds of changes.
    kind: "percentrelative" as GraphKind,
  };
  const graphData = await GRAPH_RESOLVER.loadGraph(selector);
  renderPlots(graphData, selector, chartElement.value, {
    renderTitle: false,
  });
}

const metadata = computed(
  (): CompileBenchmarkMetadata =>
    props.benchmarkMap[props.testCase.benchmark] ?? null
);
const cargoProfile = computed((): CargoProfileMetadata => {
  if (
    props.testCase.profile === "opt" &&
    metadata?.value.release_profile !== null
  ) {
    return metadata.value.release_profile;
  } else if (
    props.testCase.profile === "debug" &&
    metadata?.value.dev_profile !== null
  ) {
    return metadata?.value.dev_profile;
  }
});

const chartElement: Ref<HTMLElement | null> = ref(null);

onMounted(() => renderGraph());
</script>

<template>
  <div class="wrapper">
    <div>
      <div class="title info bold">Benchmark info</div>
      <table>
        <tbody>
          <tr>
            <td>Benchmark</td>
            <td>{{ testCase.benchmark }}</td>
          </tr>
          <tr>
            <td>Profile</td>
            <td>{{ testCase.profile }}</td>
          </tr>
          <tr>
            <td>Scenario</td>
            <td>{{ testCase.scenario }}</td>
          </tr>
          <tr>
            <td>Category</td>
            <td>{{ testCase.category }}</td>
          </tr>
          <tr v-if="(metadata?.binary ?? null) !== null">
            <td>Artifact</td>
            <td>{{ metadata.binary ? "binary" : "library" }}</td>
          </tr>
          <tr v-if="(metadata?.iterations ?? null) !== null">
            <td>
              Iterations<Tooltip>
                How many times is the benchmark executed?
              </Tooltip>
            </td>
            <td>{{ metadata.iterations }}</td>
          </tr>
          <tr v-if="(cargoProfile?.lto ?? null) !== null">
            <td>LTO</td>
            <td>{{ cargoProfile.lto }}</td>
          </tr>
          <tr v-if="(cargoProfile?.debug ?? null) !== null">
            <td>Debuginfo</td>
            <td>{{ cargoProfile.debug }}</td>
          </tr>
          <tr v-if="(cargoProfile?.codegen_units ?? null) !== null">
            <td>Codegen units</td>
            <td>{{ cargoProfile.codegen_units }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <div>
      <div class="title">
        <div class="bold">30 day history (up to benchmarked commit)</div>
        <div style="font-size: 0.8em">
          Each plotted value is relative to its previous commit
        </div>
      </div>
      <div ref="chartElement"></div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.wrapper {
  display: flex;
  margin: 10px 0;
}
.title {
  &.bold,
  .bold {
    font-weight: bold;
  }
  &.info {
    margin-bottom: 15px;
  }
}
table {
  align-self: flex-start;
  margin-right: 20px;

  td {
    text-align: left;

    &:first-child {
      font-weight: bold;
      padding-right: 10px;
    }
  }
}
</style>

<style>
.u-tooltip {
  font-size: 10pt;
  position: absolute;
  background: #fff;
  display: none;
  border: 2px solid black;
  padding: 4px;
  pointer-events: none;
  z-index: 100;
  white-space: pre;
  font-family: monospace;
}
</style>
