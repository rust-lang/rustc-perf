import Highcharts from "highcharts";
import {DASHBOARD_DATA_URL} from "../urls";

import {getJson} from "../utils/requests";

interface DashboardCases {
  clean_averages: [number];
  base_incr_averages: [number];
  clean_incr_averages: [number];
  println_incr_averages: [number];
}

interface DashboardResponse {
  Ok: {
    versions: [string];
    check: DashboardCases;
    debug: DashboardCases;
    opt: DashboardCases;
    doc: DashboardCases;
  };
}

type Profile = "check" | "debug" | "opt" | "doc";

function render(
  element: string,
  name: Profile,
  data: DashboardCases,
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
      min: 0,
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

function populate_data(response: DashboardResponse) {
  const data = response.Ok;
  render("check-average-times", "check", data.check, data.versions);
  render("debug-average-times", "debug", data.debug, data.versions);
  render("opt-average-times", "opt", data.opt, data.versions);
  render("doc-average-times", "doc", data.doc, data.versions);
}

async function make_data() {
  const response = await getJson<DashboardResponse>(DASHBOARD_DATA_URL);
  populate_data(response);
}

make_data();
