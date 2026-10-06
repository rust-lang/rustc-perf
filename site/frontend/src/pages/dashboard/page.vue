<script setup lang="tsx">
import {ref, Ref, onMounted} from "vue";

import {getUrlParams} from "../../utils/navigation";
import {DASHBOARD_DATA_URL} from "../../urls";
import {getJson} from "../../utils/requests";
import {
  BenchmarkInfo,
  loadBenchmarkInfo,
  DEFAULT_COMPILE_TARGET_TRIPLE,
} from "../../api";
import uPlot, {TypedArray} from "uplot";
import {wheelZoomUplotPlugin} from "../../utils/chart";

type ScaleKind = "linear" | "log";
type Profile = "check" | "debug" | "opt" | "doc";

interface DashboardCompileBenchmarkCases {
  clean_averages: number[];
  base_incr_averages: number[];
  clean_incr_averages: number[];
  println_incr_averages: number[];
}

interface DashboardData {
  versions: string[];
  check: DashboardCompileBenchmarkCases;
  debug: DashboardCompileBenchmarkCases;
  opt: DashboardCompileBenchmarkCases;
  doc: DashboardCompileBenchmarkCases;
  runtime: number[];
}

class CompileTarget {
  public constructor(public name: string, public url: string) {}
}

const windowLocation = `${window.location.origin}${window.location.pathname}`;

const scale: Ref<ScaleKind> = ref("linear");
const response: Ref<DashboardData | null> = ref(null);
const error: Ref<string | null> = ref(null);
const infoResponse: Ref<BenchmarkInfo | null> = ref(null);
const compileTargets: Ref<CompileTarget[]> = ref([]);

function clearError() {
  error.value = null;
}

function handleScaleChange(e: Event) {
  const value = (e.target as HTMLInputElement).value;
  if (scale.value !== value) {
    scale.value = value as ScaleKind;
    getDataAndRenderCharts();
  }
}

const charts = new Map<string, uPlot>();

function renderCompileTime(
  elementId: string,
  name: Profile,
  data: DashboardCompileBenchmarkCases,
  versions: string[]
) {
  const articles = {check: "a", debug: "a", opt: "an", doc: "a"};

  const commonCacheStateColors = {
    full: "#7cb5ec",
    "incr-full": "#434348",
    "incr-unchanged": "#90ed7d",
    "incr-patched: println": "#f7a35c",
  };
  const series = [
    {
      label: "full",
      width: devicePixelRatio,
      stroke: commonCacheStateColors["full"],
    },
    {
      label: "incremental full",
      width: devicePixelRatio,
      stroke: commonCacheStateColors["incr-full"],
    },
    {
      label: "incremental unchanged",
      width: devicePixelRatio,
      stroke: commonCacheStateColors["incr-unchanged"],
    },
    {
      label: "incremental patched: println",
      width: devicePixelRatio,
      stroke: commonCacheStateColors["incr-patched: println"],
    },
  ];

  const plotData = [
    data.clean_averages,
    data.clean_incr_averages,
    data.base_incr_averages,
    data.println_incr_averages,
  ];
  renderChart(
    elementId,
    plotData,
    series,
    versions,
    `Average time for ${articles[name]} ${name} build`,
    "Seconds"
  );
}

function renderRuntime(element: string, data: number[], versions: string[]) {
  // Remove null and convert nanoseconds to miliseconds
  // The null values, which indicate that the runtime data is missing, are only present at the beginning of the array.
  const formattedData = data
    .filter((data) => data != null)
    .map((data) => data / 1_000_000);
  const nullCount = data.length - formattedData.length;
  const versionsNormalized = versions.slice(nullCount);

  const series = [{width: devicePixelRatio, stroke: "#7cb5ec"}];
  renderChart(
    element,
    [formattedData],
    series,
    versionsNormalized,
    "Average time for a runtime benchmark",
    "Milliseconds",
    false
  );
}

function renderChart(
  elementId: string,
  data: number[][],
  series: any[],
  versions: string[],
  title: string,
  yAxisLabel: string,
  showLegend: boolean = true
) {
  // Clear the old chart, if present
  const oldChart = charts.get(elementId);
  if (oldChart !== undefined) {
    oldChart.destroy();
  }

  const element = document.getElementById(elementId)!;

  let columns = 2;
  const parentWidth = wrapperRef.value!.clientWidth;

  const smallDisplay = parentWidth < 1000;

  // Small display, reduce column count to 1
  if (smallDisplay) {
    columns = 1;
  }

  const width = Math.floor(parentWidth / columns) - 10;
  const height = 300;
  const yScale: {distr?: number; log?: 2 | 10} = {};
  if (scale.value === "log") {
    yScale["distr"] = 3; // logarithmic scale
    yScale["log"] = 10; // base 10
  }

  const plotOpts = {
    title,
    series: [{}, ...series],
    width,
    height,
    legend: {
      live: false,
      show: showLegend,
    },
    focus: {
      alpha: 0.3,
    },
    cursor: {
      focus: {
        prox: 5,
      },
      drag: {
        x: true,
        y: true,
      },
    },
    axes: [
      {
        label: "Version",
        splits: (_u: any) => {
          // Show every even version, plus the last beta
          // On small displays, show less versions
          let factor = 2;
          if (smallDisplay) {
            factor = 4;
          }
          const ticks = [];
          for (let i = 0; i < versions.length; i++) {
            if (i % factor == 0) {
              ticks.push(i);
            }
          }
          // Include last version (usually beta)
          if (ticks[-1] !== versions.length - 1) {
            ticks.push(versions.length - 1);
          }
          return ticks;
        },
        values: (_u: any, splits: number[]) => splits.map((i) => versions[i]),
        rotate: 45,
        size: 90, // to avoid cutting off the label
        grid: {
          show: false,
        },
      },
      {
        label: yAxisLabel,
      },
    ],
    scales: {
      x: {
        time: false, // not a timestamp axis
        range: (_u: any, min: number, max: number): [number, number] => [
          min - 0.5,
          max + 0.5,
        ], // padding at the edges
      },
      y: yScale,
    },
    plugins: [
      tooltipPlugin(versions, yAxisLabel),
      wheelZoomUplotPlugin({factor: 0.75}),
    ],
  };

  const versionIndices = versions.map((_, index) => index);
  const plotData = [versionIndices, ...data];
  charts.set(
    elementId,
    new uPlot(plotOpts, plotData as any as TypedArray[], element)
  );
}

function tooltipPlugin(
  versions: string[],
  unit: string,
  {shiftX = 10, shiftY = 10} = {}
) {
  let tooltipLeftOffset = 0;
  let tooltipTopOffset = 0;

  const tooltip = document.createElement("div");
  tooltip.className = "u-tooltip";

  let seriesIdx: number | null = null;
  let dataIdx: number | null = null;

  let over: any | null = null;

  let tooltipVisible = false;

  function showTooltip() {
    if (!tooltipVisible) {
      tooltip.style.display = "block";
      over.style.cursor = "pointer";
      tooltipVisible = true;
    }
  }

  function hideTooltip() {
    if (tooltipVisible) {
      tooltip.style.display = "none";
      over.style.cursor = null;
      tooltipVisible = false;
    }
  }

  function setTooltip(u: any) {
    showTooltip();

    let top = u.valToPos(u.data[seriesIdx!][dataIdx!], "y");
    let lft = u.valToPos(u.data[0][dataIdx!], "x");

    tooltip.style.top = tooltipTopOffset + top + shiftY + "px";
    tooltip.style.left = tooltipLeftOffset + lft + shiftX + "px";
    tooltip.style.borderColor = u.series[seriesIdx!].stroke(u, seriesIdx!);

    const value = u.data[seriesIdx!][dataIdx!];
    tooltip.textContent = `${versions[dataIdx!]}
${value.toFixed(3)} ${unit.toLowerCase()}`;
  }

  return {
    hooks: {
      ready: [
        (u: any) => {
          over = u.root.querySelector(".u-over");

          tooltipLeftOffset = parseFloat(over.style.left);
          tooltipTopOffset = parseFloat(over.style.top);
          u.root.querySelector(".u-wrap").appendChild(tooltip);
        },
      ],
      setCursor: [
        (u: any) => {
          let c = u.cursor;

          if (dataIdx != c.idx) {
            dataIdx = c.idx;

            if (seriesIdx != null) setTooltip(u);
          }
        },
      ],
      setSeries: [
        (u: any, sidx: number | null) => {
          if (seriesIdx != sidx) {
            seriesIdx = sidx;

            if (sidx == null) hideTooltip();
            else if (dataIdx != null) setTooltip(u);
          }
        },
      ],
    },
  };
}

function renderCharts(data: DashboardData) {
  renderCompileTime("check-average-times", "check", data.check, data.versions);
  renderCompileTime("debug-average-times", "debug", data.debug, data.versions);
  renderCompileTime("opt-average-times", "opt", data.opt, data.versions);
  renderCompileTime("doc-average-times", "doc", data.doc, data.versions);
  renderRuntime("runtime-average-times", data.runtime, data.versions);
}

async function getDataAndRenderCharts() {
  clearError();
  if (response.value === null) {
    const urlParams = getUrlParams();
    try {
      const apiResponse = await getJson<DashboardData>(
        DASHBOARD_DATA_URL,
        urlParams
      );
      response.value = apiResponse;
      renderCharts(apiResponse);
      return;
    } catch (e) {
      error.value = e.error;
    }
  } else {
    renderCharts(response.value);
  }
}

async function getCompileTargets() {
  clearError();
  if (!infoResponse.value) {
    try {
      const info = await loadBenchmarkInfo();
      infoResponse.value = info;
      const apiCompileTargets = info.compile_targets ?? [];
      const targets: CompileTarget[] = [];
      for (const target of apiCompileTargets) {
        const compileTarget = new CompileTarget(
          target,
          `${windowLocation}?target=${target}`
        );
        targets.push(compileTarget);
      }
      compileTargets.value = targets;
    } catch (e) {
      error.value = e.error;
    }
  }
}

const wrapperRef: Ref<HTMLElement | null> = ref(null);

onMounted(async () => {
  await Promise.all([getCompileTargets(), getDataAndRenderCharts()]);
});

function getActiveClass(target: CompileTarget): string {
  const params = getUrlParams();
  const curTarget = params?.["target"];
  if (!curTarget) {
    return target.name === DEFAULT_COMPILE_TARGET_TRIPLE ? "target-active" : "";
  }
  return target.name === curTarget ? "target-active" : "";
}
</script>

<template>
  <details style="margin-top: 10px">
    <summary>What data is in the dashboard?</summary>

    The dashboard shows performance results for all stable Rust releases going
    back to
    <code>1.26.0</code>, along with the latest <code>beta</code> release. The
    displayed duration is an arithmetic mean amongst all
    <a
      href="https://github.com/rust-lang/rustc-perf/tree/main/collector/compile-benchmarks#stable"
      >stable</a
    >
    benchmarks. The dashboard also shows the average duration of runtime
    benchmarks, which measure the performance of Rust programs compiled by a
    given version of the Rust compiler.
  </details>

  <form id="scale-select-form">
    <label for="linear-scale-input">
      <input
        id="linear-scale-input"
        type="radio"
        name="scale-select"
        value="linear"
        v-model="scale"
        @input="handleScaleChange"
      />
      Linear-scale
    </label>
    <label for="log-scale-input">
      <input
        id="log-scale-input"
        type="radio"
        name="scale-select"
        value="log"
        v-model="scale"
        @input="handleScaleChange"
      />
      Log-scale
    </label>
  </form>

  <div class="target-wrapper">
    <strong>Targets: </strong>
    <div class="target-list-wrapper">
      <template v-for="target in compileTargets">
        <span class="target-list-element">
          <a :class="getActiveClass(target)" :href="target.url"
            >{{ target.name }}
          </a>
        </span>
      </template>
    </div>
  </div>

  <div v-if="error == null" class="graphs" ref="wrapperRef">
    <div id="check-average-times" class="graph"></div>
    <div id="debug-average-times" class="graph"></div>
    <div id="opt-average-times" class="graph"></div>
    <div id="doc-average-times" class="graph"></div>
    <div id="runtime-average-times" class="graph"></div>
  </div>
  <h2 v-else>Error: {{ error }}</h2>
</template>

<style scoped lang="scss">
.graphs {
  display: grid;
  grid-template-columns: repeat(2, 1fr);

  @media screen and (max-width: 768px) {
    grid-template-columns: 1fr;
  }

  .graph {
    margin-top: 20px;
  }
}

.target-wrapper {
  padding-top: 5px;
  display: flex;
  flex-direction: column;
}

.target-list-wrapper {
  display: flex;
}

.target-active {
  font-weight: bold;
  text-decoration: underline;
}

.target-list-element {
  padding-right: 5px;
}
</style>
