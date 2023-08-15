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
import {daysBetweenDates, getFutureDate, getPastDate} from "./utils";
import {GraphRenderOpts, renderPlots} from "../../../../graph/render";
import {GRAPH_RESOLVER} from "../../../../graph/resolver";
import {GraphKind} from "../../../../graph/data";
import uPlot from "uplot";

const props = defineProps<{
  testCase: CompileTestCase;
  metric: string;
  artifact: ArtifactDescription;
  benchmarkMap: CompileBenchmarkMap;
}>();

type GraphRange = {
  start: string;
  end: string;
  date: Date | null;
};

// How many days are shown in the graph
const DAY_RANGE = 30;

/**
 * Calculates the start and end range for a history graph for this benchmark
 * and artifact.
 */
function getGraphRange(artifact: ArtifactDescription): GraphRange {
  const date = new Date(artifact.date);

  // If this is a try commit, we don't know its future, so always we just display
  // the last `DAY_RANGE` days.
  if (artifact.type === "try") {
    return {
      start: getPastDate(date, DAY_RANGE),
      end: artifact.commit,
      date: null,
    };
  } else {
    // If this is a master commit, then we try to display `dayRange` days
    // "centered" around the commit date.

    // Calculate the end of the range, which is commit date + half of the
    // amount of days we want to show. If this date is in the future,
    // the server will clip the result at the current date.
    const end = getFutureDate(date, DAY_RANGE / 2);

    // Calculate how many days there have been from the commit date
    const daysInFuture = Math.min(
      DAY_RANGE / 2,
      daysBetweenDates(date, new Date())
    );

    // Calculate how many days we should go into the past, taking into account
    // the days that will be clipped by the server.
    const daysInPast = DAY_RANGE - daysInFuture;

    const start = getPastDate(date, daysInPast);
    return {
      start,
      end,
      date,
    };
  }
}

/**
 * Hook into the uPlot drawing machinery to draw a rectangle from the `date` to
 * the end of the plot, representing the region that is the date's future.
 */
function drawCurrentDate(opts: GraphRenderOpts, date: Date) {
  opts.hooks = {
    drawSeries: (u: uPlot) => {
      let ctx = u.ctx;
      const x = u.valToPos(date.getTime() / 1000, "x", true);

      // Draw a translucent rectangle representing the region that is more
      // recent than `date`.
      ctx.save();
      ctx.fillStyle = "rgba(0, 0, 0, 0.03)";
      ctx.rect(x, u.bbox.top, u.bbox.width - x + u.bbox.left, u.bbox.height);
      ctx.fill();
      ctx.restore();
    },
  };
}

async function renderGraph() {
  const {start, end, date} = graphRange.value;
  const selector = {
    benchmark: props.testCase.benchmark,
    profile: props.testCase.profile,
    scenario: props.testCase.scenario,
    stat: props.metric,
    start,
    end,
    // We want to be able to see noise "blips" vs. a previous artifact.
    // The "percent relative from previous commit" graph should be the best to
    // see these kinds of changes.
    kind: "percentrelative" as GraphKind,
  };
  const graphData = await GRAPH_RESOLVER.loadGraph(selector);
  const opts: GraphRenderOpts = {
    renderTitle: false,
  };
  if (date !== null) {
    drawCurrentDate(opts, date);
  }
  renderPlots(graphData, selector, chartElement.value, opts);
}

function getGraphTitle() {
  const {start, end, date} = graphRange.value;
  const msg = `${DAY_RANGE} day history`;
  if (date !== null) {
    return `${msg} (${start} - ${end})`;
  } else {
    return `${msg} (up to benchmarked commit)`;
  }
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
const graphRange = computed(() => getGraphRange(props.artifact));

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
        <div class="bold">{{ getGraphTitle() }}</div>
        <div style="font-size: 0.8em">
          Each plotted value is relative to its previous commit
        </div>
        <div style="font-size: 0.8em">
          The shaded region shows values that are more recent than the
          benchmarked commit
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
