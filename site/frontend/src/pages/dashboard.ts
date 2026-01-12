import Highcharts from "highcharts";
import {createApp} from "vue";
import {DASHBOARD_DATA_URL} from "../urls";
import Dashboard from "./dashboard/page.vue";
import WithSuspense from "../components/with-suspense.vue";
import {getUrlParams} from "../utils/navigation";
import {getApi} from "../utils/requests";

type ScaleKind = "linear" | "log";
let scale: ScaleKind = "linear";

const buttons = Array.from(
  document.querySelectorAll<HTMLInputElement>("#scale-select-form input")
);

buttons.map((button) => {
  button.addEventListener("change", () => {
    if (button.checked) {
      scale = button.value as ScaleKind;
      make_data();
    }
  });
});

interface DashboardCompileBenchmarkCases {
  clean_averages: [number];
  base_incr_averages: [number];
  clean_incr_averages: [number];
  println_incr_averages: [number];
}

interface DashboardResponse {
  versions: [string];
  check: DashboardCompileBenchmarkCases;
  debug: DashboardCompileBenchmarkCases;
  opt: DashboardCompileBenchmarkCases;
  doc: DashboardCompileBenchmarkCases;
  runtime: [number];
}

type Profile = "check" | "debug" | "opt" | "doc";

function render(
  element: string,
  name: Profile,
  data: DashboardCompileBenchmarkCases,
  versions: [string]
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
      min: scale === "linear" ? 0 : undefined,
      type: scale === "log" ? "logarithmic" : undefined,
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

function renderRuntime(element: string, data: [number], versions: [string]) {
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
      min: scale === "linear" ? 0 : undefined,
      type: scale === "log" ? "logarithmic" : undefined,
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

function populate_data(response: DashboardResponse) {
  const data = response;
  render("check-average-times", "check", data.check, data.versions);
  render("debug-average-times", "debug", data.debug, data.versions);
  render("opt-average-times", "opt", data.opt, data.versions);
  render("doc-average-times", "doc", data.doc, data.versions);
  renderRuntime("runtime-average-times", data.runtime, data.versions);
}

let response: DashboardResponse | null = null;

async function make_data() {
  // As we are not really rendering HTML on this page in the traditional VUE
  // sense - to handle an error we render and empty chart where the title of
  // the chart is the error.
  if (!response) {
    const urlParams = getUrlParams();
    const apiResponse = await getApi(DASHBOARD_DATA_URL, urlParams);
    if (apiResponse.ok) {
      response = (await apiResponse.json()) as DashboardResponse;
      populate_data(response);
    } else {
      const responseText = await apiResponse.text();
      Highcharts.chart({
        chart: {
          renderTo: "check-average-times",
          zooming: {
            type: "xy",
          },
          type: "line",
        },
        title: {
          text: `${responseText}`,
        },
        yAxis: {
          title: {text: "Error"},
          min: scale === "linear" ? 0 : undefined,
          type: scale === "log" ? "logarithmic" : undefined,
        },
        xAxis: {
          categories: [],
          title: {text: "Error"},
        },
        series: [
          {
            showInLegend: false,
            type: "line",
            animation: false,
            data: [],
          },
        ],
      });
    }
  }
}

make_data();

const app = createApp(WithSuspense, {
  component: Dashboard,
});
app.mount("#app");
