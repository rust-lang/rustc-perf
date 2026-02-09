<script setup lang="tsx">
import {ref, Ref, onMounted} from "vue";
import Highcharts from "highcharts";

import {getUrlParams} from "../../utils/navigation";
import {DASHBOARD_DATA_URL} from "../../urls";
import {getJson} from "../../utils/requests";

type ScaleKind = "linear" | "log";
type Profile = "check" | "debug" | "opt" | "doc";
type TargetUrl = {
  name: string;
  url: string;
};

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

const windowLocation = `${window.location.origin}${window.location.pathname}`;

const x86_64UnknownLinuxGnu = {
  name: "x86_64-unknown-linux-gnu",
  url: `${windowLocation}?target=x86_64-unknown-linux-gnu`,
};

const AArch64UnknownLinuxGnu = {
  name: "aarch64-unknown-linux-gnu",
  url: `${windowLocation}?target=aarch64-unknown-linux-gnu`,
};

const TargetUrls = [x86_64UnknownLinuxGnu, AArch64UnknownLinuxGnu];

const scale: Ref<ScaleKind> = ref("linear");
const response: Ref<DashboardData | null> = ref(null);
const error: Ref<string | null> = ref(null);

function clearError() {
  error.value = null;
}

function handleScaleChange(e: Event) {
  const value = (e.target as HTMLInputElement).value;
  if (value !== "linear" && value != "log") {
    console.error("Invalid scale");
  } else if (scale.value !== value) {
    scale.value = value;
    getDataAndRenderCharts();
  }
}

function render(
  element: string,
  name: Profile,
  data: DashboardCompileBenchmarkCases,
  versions: string[]
) {
  let articles = {check: "a", debug: "a", opt: "an", doc: "a"};

  Highcharts.chart({
    chart: {
      renderTo: element,
      zooming: {
        type: "xy",
      },
      type: "line",
    },
    title: {
      text: `Average time for ${articles[name]} ${name} build`,
    },
    yAxis: {
      title: {text: "Seconds"},
      min: scale.value === "linear" ? 0 : undefined,
      type: scale.value === "log" ? "logarithmic" : undefined,
    },
    xAxis: {
      categories: versions,
      title: {text: "Version"},
    },
    series: [
      {
        type: "line",
        name: "full",
        animation: false,
        data: data.clean_averages,
      },
      {
        type: "line",
        name: "incremental full",
        animation: false,
        data: data.base_incr_averages,
      },
      {
        type: "line",
        name: "incremental unchanged",
        animation: false,
        data: data.clean_incr_averages,
      },
      {
        type: "line",
        name: "incremental patched: println",
        animation: false,
        data: data.println_incr_averages,
      },
    ],
  });
}

function renderRuntime(element: string, data: number[], versions: string[]) {
  // Remove null and convert nanoseconds to miliseconds
  // The null values, which indicate that the runtime data is missing, are only present at the beginning of the array.
  const formattedData = data
    .filter((data) => data != null)
    .map((data) => data / 1_000_000);
  const nullCount = data.length - formattedData.length;

  Highcharts.chart({
    chart: {
      renderTo: element,
      zooming: {
        type: "xy",
      },
      type: "line",
    },
    title: {
      text: `Average time for a runtime benchmark`,
    },
    yAxis: {
      title: {text: "Miliseconds"},
      min: scale.value === "linear" ? 0 : undefined,
      type: scale.value === "log" ? "logarithmic" : undefined,
    },
    xAxis: {
      categories: versions.slice(nullCount),
      title: {text: "Version"},
    },
    series: [
      {
        showInLegend: false,
        type: "line",
        animation: false,
        data: formattedData,
      },
    ],
  });
}

function renderCharts() {
  const data = response.value;
  render("check-average-times", "check", data.check, data.versions);
  render("debug-average-times", "debug", data.debug, data.versions);
  render("opt-average-times", "opt", data.opt, data.versions);
  render("doc-average-times", "doc", data.doc, data.versions);
  renderRuntime("runtime-average-times", data.runtime, data.versions);
}

async function getDataAndRenderCharts() {
  clearError();
  if (!response.value) {
    const urlParams = getUrlParams();
    try {
      const apiResponse = await getJson<DashboardData>(
        DASHBOARD_DATA_URL,
        urlParams
      );
      response.value = apiResponse;
      renderCharts();
    } catch (e) {
      error.value = e.error;
    }
  }
}

onMounted(() => {
  getDataAndRenderCharts();
});

function getActiveClass(target: TargetUrl): string {
  const params = getUrlParams();
  const curTarget = params?.["target"];
  if (!curTarget) {
    return target.name === x86_64UnknownLinuxGnu.name ? "target-active" : "";
  }
  return target.name == curTarget ? "target-active" : "";
}
</script>

<template>
  <details style="margin-top: 10px">
    <summary>What data is in the dashboard?</summary>

    The dashboard shows performance results for all stable Rust releases going
    back to
    <code>1.28.0</code>, along with the latest <code>beta</code> release. The
    displayed duration is an arithmetic mean amongst all
    <a
      href="https://github.com/rust-lang/rustc-perf/tree/master/collector/compile-benchmarks#stable"
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
      <template v-for="target in TargetUrls">
        <span class="target-list-element">
          <a :class="getActiveClass(target)" :href="target.url"
            >{{ target.name }}
          </a>
        </span>
      </template>
    </div>
  </div>

  <div v-if="error == null" class="graphs">
    <div id="check-average-times"></div>
    <div id="debug-average-times"></div>
    <div id="opt-average-times"></div>
    <div id="doc-average-times"></div>
    <div id="runtime-average-times"></div>
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
