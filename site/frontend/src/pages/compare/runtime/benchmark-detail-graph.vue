<script setup lang="ts">
import {computed, onMounted, Ref, ref} from "vue";
import {GraphKind, GraphsSelector, RuntimeGraphData} from "../../../graph/data";
import {GraphRenderOpts, renderRuntimePlots} from "../../../graph/render";
import uPlot from "uplot";
import {
  daysBetweenDates,
  formatDate,
  getFutureDate,
  getPastDate,
} from "../compile/table/utils";
import {ArtifactDescription} from "../types";
import {RuntimeTestCase} from "./common";
import {
  RUNTIME_DETAIL_GRAPHS_RESOLVER,
  RuntimeDetailGraphs,
  RuntimeDetailGraphsSelector,
} from "./runtime-detail-resolver";

const props = defineProps<{
  testCase: RuntimeTestCase;
  metric: string;
  artifact: ArtifactDescription;
  baseArtifact: ArtifactDescription;
}>();

// How many days are shown in the graph
const DAY_RANGE = 30;

function createGraphsSelector(): RuntimeDetailGraphsSelector {
  const {start, end} = graphRange.value;

  return {
    benchmark: props.testCase.benchmark,
    stat: props.metric,
    start,
    end,
    kinds: ["percentrelative", "percentfromfirst"] as GraphKind[],
  };
}

// Render both relative and absolute graphs
async function renderGraphs(detail: RuntimeDetailGraphs) {
  const selector = createGraphsSelector();
  const date = graphRange.value.date;
  if (detail.commits.length === 0) {
    return;
  }

  function buildGraph(
    index: number,
    kind: GraphKind
  ): [RuntimeGraphData, GraphsSelector] {
    const data: RuntimeGraphData = {
      commits: detail.commits,
      benchmarks: {
        [selector.benchmark]: detail.graphs[index],
      },
    };

    const graphSelector = {
      benchmark: selector.benchmark,
      stat: selector.stat,
      start: selector.start,
      end: selector.end,
      profile: null,
      scenario: null,
      kind,
    };

    return [data, graphSelector];
  }

  const [percentRelativeData, percentRelativeSelector] = buildGraph(
    0,
    "percentrelative"
  );
  const [percentFromFirstData, percentFromFirstSelector] = buildGraph(
    1,
    "percentfromfirst"
  );

  // We want to be able to see noise "blips" vs. a previous artifact.
  // The "percent relative from previous commit" graph should be the best to
  // see these kinds of changes.
  renderGraph(
    percentRelativeData,
    percentRelativeSelector,
    date,
    relativeToPreviousChartElement
  );
  // We also want to see whether a change maintained its value or whether it was noise and has since
  // returned to steady state. Here, an absolute graph ("raw") would be best, however small changes
  // are hard to observe in it. Therefore, we use "percent from first commit" graph instead, which
  // is essentialy a "zoomed-in" version of the raw graph.
  renderGraph(
    percentFromFirstData,
    percentFromFirstSelector,
    date,
    relativeToFirstChartElement
  );
}

async function renderGraph(
  graphData: RuntimeGraphData,
  selector: GraphsSelector,
  date: Date | null,
  chartRef: Ref<HTMLElement | null>
) {
  const opts: GraphRenderOpts = {
    width: Math.min(window.innerWidth - 40, 465),
    renderTitle: false,
  };
  if (date !== null) {
    drawCurrentDate(opts, date);
  }

  renderRuntimePlots(graphData, selector, chartRef.value, opts);
}

async function loadGraphs(): Promise<RuntimeDetailGraphs> {
  return await RUNTIME_DETAIL_GRAPHS_RESOLVER.load(createGraphsSelector());
}

function getGraphTitle() {
  const {start, end, date} = graphRange.value;
  const days = date
    ? daysBetweenDates(new Date(start), new Date(end))
    : DAY_RANGE;
  const msg = `${days} day history`;
  if (date !== null) {
    return `${msg} (${start} → ${end})`;
  } else {
    return `${msg} (up to benchmarked commit)`;
  }
}

type GraphRange = {
  start: string;
  end: string;
  date: Date | null;
};

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
      ctx.fillStyle = "rgba(0, 0, 0, 0.07)";
      ctx.rect(x, u.bbox.top, u.bbox.width - x + u.bbox.left, u.bbox.height);
      ctx.fill();
      ctx.restore();
    },
  };
}

/**
 * Calculates the start and end range for a history graph for this benchmark
 * and artifact.
 */
function getGraphRange(
  artifact: ArtifactDescription,
  baseArtifact: ArtifactDescription
): GraphRange {
  // If this is a try commit, we don't know its future, so always we just display
  // the last `DAY_RANGE` days.
  if (artifact.type === "try") {
    const date = new Date(artifact.date);
    return {
      start: formatDate(getPastDate(date, DAY_RANGE)),
      end: artifact.commit,
      date: null,
    };
  } else {
    let [start_date, end_date] = [baseArtifact, artifact].map(
      (c) => new Date(c.date)
    );
    // If this is a master commit, we attempt to display more than the full history for commit
    // ranges. If the commit range is not larger than the `dayRange`, the display will likely be
    // "centered" around the commit date.

    // Calculate the end of the range, which is the earlier date between
    // current date and the commit date + half of the amount of days we
    // want to show.
    let centered_end = getFutureDate(end_date, DAY_RANGE / 2);
    const today = new Date().setUTCHours(0, 0, 0, 0);
    if (centered_end.getTime() > today) {
      centered_end = new Date(today);
    }
    const end = formatDate(centered_end);

    // Calculate the start of the range, which is the earlier date between
    // the base artifact date and the calculated end date - the amount of days
    // we want to show.
    const centered_start = getPastDate(centered_end, DAY_RANGE);
    const start = formatDate(
      start_date < centered_start ? start_date : centered_start
    );

    return {
      start,
      end,
      date: end_date,
    };
  }
}

const relativeToPreviousChartElement: Ref<HTMLElement | null> = ref(null);
const relativeToFirstChartElement: Ref<HTMLElement | null> = ref(null);
const graphRange = computed(() =>
  getGraphRange(props.artifact, props.baseArtifact)
);

onMounted(() => {
  loadGraphs().then((d) => {
    renderGraphs(d);
  });
});
</script>

<template>
  <div class="columns graphs">
    <div class="rows center-items grow">
      <div class="title">
        <div class="bold">{{ getGraphTitle() }}</div>
        <div style="font-size: 0.8em">
          Each plotted value is relative to the first commit of the commit range
        </div>
        <div style="font-size: 0.8em">
          The shaded region shows values that are more recent than the
          benchmarked commit
        </div>
      </div>
      <div ref="relativeToFirstChartElement"></div>
    </div>
    <div class="rows center-items grow">
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
      <div ref="relativeToPreviousChartElement"></div>
    </div>
  </div>
</template>
