<script setup lang="ts">
import {
  CargoProfileMetadata,
  CompileBenchmarkMap,
  CompileBenchmarkMetadata,
  CompileTestCase,
} from "../common";
import {capitalize, computed, onMounted, Ref, ref} from "vue";
import Tooltip from "../../tooltip.vue";
import {ArtifactDescription} from "../../types";
import {
  daysBetweenDates,
  getFutureDate,
  getPastDate,
  formatDate,
} from "./utils";
import {GraphRenderOpts, renderPlots} from "../../../../graph/render";
import {GraphData, GraphKind, GraphsSelector} from "../../../../graph/data";
import uPlot from "uplot";
import {
  COMPILE_DETAIL_GRAPHS_RESOLVER,
  COMPILE_DETAIL_SECTIONS_RESOLVER,
  CompileDetailGraphs,
  CompileDetailGraphsSelector,
  CompileDetailSections,
  CompileDetailSectionsSelector,
} from "./detail-resolver";
import CompileSectionsChart from "./sections-chart.vue";
import PerfettoLink from "../../../../components/perfetto-link.vue";
import ProfileShortcut from "./shortcuts/profile-shortcut.vue";
import BinarySizeShortcut from "./shortcuts/binary-size-shortcut.vue";

const props = defineProps<{
  testCase: CompileTestCase;
  metric: string;
  artifact: ArtifactDescription;
  baseArtifact: ArtifactDescription;
  benchmarkMap: CompileBenchmarkMap;
}>();

const BINARY_SIZE_METRIC: string = "size:linked_artifact";

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

function createGraphsSelector(): CompileDetailGraphsSelector {
  const {start, end} = graphRange.value;
  return {
    benchmark: props.testCase.benchmark,
    profile: props.testCase.profile,
    scenario: props.testCase.scenario,
    stat: props.metric,
    start,
    end,
    kinds: ["percentrelative", "percentfromfirst"] as GraphKind[],
  };
}

async function loadGraphs(): Promise<CompileDetailGraphs> {
  return await COMPILE_DETAIL_GRAPHS_RESOLVER.load(createGraphsSelector());
}

function createSectionsSelector(): CompileDetailSectionsSelector {
  return {
    benchmark: props.testCase.benchmark,
    profile: props.testCase.profile,
    scenario: props.testCase.scenario,
    start: props.baseArtifact.commit,
    end: props.artifact.commit,
  };
}

async function loadSections(): Promise<CompileDetailSections> {
  return await COMPILE_DETAIL_SECTIONS_RESOLVER.load(createSectionsSelector());
}

// Render both relative and absolute graphs
async function renderGraphs(detail: CompileDetailGraphs) {
  const selector = createGraphsSelector();
  const date = graphRange.value.date;
  if (detail.commits.length === 0) {
    return;
  }

  function buildGraph(
    index: number,
    kind: GraphKind
  ): [GraphData, GraphsSelector] {
    const data = {
      commits: detail.commits,
      benchmarks: {
        [selector.benchmark]: {
          // The server returns profiles capitalized, so we need to match that
          // here, so that the graph code can find the expected profile.
          [capitalize(selector.profile)]: {
            [selector.scenario]: detail.graphs[index],
          },
        },
      },
    };
    const graphSelector = {
      benchmark: selector.benchmark,
      profile: selector.profile,
      scenario: selector.scenario,
      stat: selector.stat,
      start: selector.start,
      end: selector.end,
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
  graphData: GraphData,
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
  renderPlots(graphData, selector, chartRef.value, opts);
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

function benchmarkLink(benchmark: string): string {
  return `https://github.com/rust-lang/rustc-perf/tree/master/collector/compile-benchmarks/${benchmark}`;
}

function detailedQueryLink(
  commit: ArtifactDescription,
  baseCommit?: ArtifactDescription
): string {
  const {benchmark, profile, scenario} = props.testCase;
  let link = `/detailed-query.html?commit=${commit.commit}&benchmark=${benchmark}-${profile}&scenario=${scenario}`;
  if (baseCommit !== undefined) {
    link += `&base_commit=${baseCommit.commit}`;
  }
  return link;
}

function graphLink(
  commit: ArtifactDescription,
  metric: string,
  testCase: CompileTestCase
): string {
  // Move to `30 days ago` to display history of the test case
  const start = formatDate(getPastDate(new Date(commit.date), 30));
  const end = commit.commit;
  const {benchmark, profile, scenario} = testCase;
  return `/index.html?start=${start}&end=${end}&benchmark=${benchmark}&profile=${profile}&scenario=${scenario}&stat=${metric}`;
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

const relativeToPreviousChartElement: Ref<HTMLElement | null> = ref(null);
const relativeToFirstChartElement: Ref<HTMLElement | null> = ref(null);
const graphRange = computed(() =>
  getGraphRange(props.artifact, props.baseArtifact)
);

const sectionsDetail: Ref<CompileDetailSections | null> = ref(null);
onMounted(() => {
  loadGraphs().then((d) => {
    renderGraphs(d);
  });
  loadSections().then((d) => {
    sectionsDetail.value = d;
  });
});
</script>

<template>
  <div>
    <div class="columns">
      <div class="rows grow">
        <div class="title bold">Benchmark info</div>
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
                Iterations
                <Tooltip> How many times is the benchmark executed?</Tooltip>
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
      <div class="rows grow links">
        <div class="title bold">Links</div>
        <ul>
          <li>
            <a
              :href="detailedQueryLink(props.artifact, props.baseArtifact)"
              target="_blank"
            >
              Detailed results
            </a>
          </li>
          <li>
            Before:
            <a :href="detailedQueryLink(props.baseArtifact)" target="_blank">
              self-profile</a
            >,
            <PerfettoLink
              :artifact="props.baseArtifact"
              :test-case="props.testCase"
              >query trace
            </PerfettoLink>
          </li>
          <li>
            After:
            <a :href="detailedQueryLink(props.artifact)" target="_blank"
              >self-profile</a
            >,
            <PerfettoLink :artifact="props.artifact" :test-case="props.testCase"
              >query trace
            </PerfettoLink>
          </li>
          <li>
            <a
              :href="graphLink(props.artifact, props.metric, props.testCase)"
              target="_blank"
            >
              History graph
            </a>
          </li>
          <li>
            <a :href="benchmarkLink(testCase.benchmark)" target="_blank">
              Benchmark source code
            </a>
          </li>
        </ul>
      </div>
    </div>
    <div class="columns graphs">
      <div class="rows center-items grow">
        <div class="title">
          <div class="bold">{{ getGraphTitle() }}</div>
          <div style="font-size: 0.8em">
            Each plotted value is relative to the first commit of the commit
            range
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
    <div class="columns" v-if="props.metric !== BINARY_SIZE_METRIC">
      <div class="rows center-items grow">
        <div class="title bold">
          Sections
          <Tooltip
            >Percentual duration of individual compilation sections. This is a
            rough estimate that might not necessarily contain all of the
            individual parts of the compilation. The sections are calculated
            based on the results of self-profile queries and they are measured
            based on wall-time.
          </Tooltip>
        </div>
        <div style="font-size: 0.8em">
          Note that the data for this chart is calculated from wall-time (it is
          metric agnostic)!
        </div>
        <div>
          <CompileSectionsChart
            v-if="
              (sectionsDetail?.before ?? null) !== null &&
              (sectionsDetail?.after ?? null) !== null
            "
            :before="sectionsDetail.before"
            :after="sectionsDetail.after"
          />
          <span v-else-if="sectionsDetail === null">Loading…</span>
          <span v-else>Not available</span>
        </div>
      </div>
    </div>
    <div class="shortcut">
      <template v-if="props.metric === BINARY_SIZE_METRIC">
        <BinarySizeShortcut
          v-if="testCase.profile === 'debug' || testCase.profile === 'opt'"
          :artifact="props.artifact"
          :base-artifact="props.baseArtifact"
          :test-case="props.testCase"
        />
      </template>
      <ProfileShortcut
        v-else
        :artifact="props.artifact"
        :base-artifact="props.baseArtifact"
        :test-case="props.testCase"
      />
    </div>
  </div>
</template>

<style scoped lang="scss">
.columns {
  display: flex;
  flex-wrap: wrap;
  gap: 15px;
  margin: 10px 0;

  .grow {
    flex-grow: 1;
  }

  &.graphs {
    flex-wrap: nowrap;
  }
}

.graphs {
  margin-top: 15px;
}

.rows {
  display: flex;
  flex-direction: column;
  gap: 10px;

  &.center-items {
    align-items: center;
  }
}

.shortcut {
  margin-top: 15px;
  text-align: left;
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

.links {
  li {
    text-align: left;
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
