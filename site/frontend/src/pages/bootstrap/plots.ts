import uPlot, {TypedArray} from "uplot";
import {BootstrapData, BootstrapSelector} from "./state";

function tooltipPlugin({onclick, commits, shiftX = 10, shiftY = 10}) {
  let tooltipLeftOffset = 0;
  let tooltipTopOffset = 0;

  const tooltip = document.createElement("div");
  tooltip.className = "u-tooltip";

  let seriesIdx = null;
  let dataIdx = null;

  const formatDate = uPlot.fmtDate("{M}/{D}/{YY} {h}:{mm}:{ss} {AA}");

  let over;

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

  function setTooltip(u) {
    showTooltip();

    let top = u.valToPos(u.data[seriesIdx][dataIdx], "y");
    let lft = u.valToPos(u.data[0][dataIdx], "x");

    tooltip.style.top = tooltipTopOffset + top + shiftX + "px";
    tooltip.style.left = tooltipLeftOffset + lft + shiftY + "px";

    tooltip.textContent =
      formatDate(new Date(u.data[0][dataIdx] * 1e3)) +
      " - " +
      commits[dataIdx][1].slice(0, 10) +
      "\n" +
      u.series[seriesIdx].label +
      ": " +
      u.data[seriesIdx][dataIdx] / 1e9 +
      " seconds";
  }

  return {
    hooks: {
      ready: [
        (u) => {
          over = u.root.querySelector(".u-over");

          tooltipLeftOffset = parseFloat(over.style.left);
          tooltipTopOffset = parseFloat(over.style.top);
          u.root.querySelector(".u-wrap").appendChild(tooltip);

          let clientX;
          let clientY;

          over.addEventListener("mousedown", (e) => {
            clientX = e.clientX;
            clientY = e.clientY;
          });

          over.addEventListener("mouseup", (e) => {
            // clicked in-place
            if (e.clientX == clientX && e.clientY == clientY) {
              if (seriesIdx != null && dataIdx != null) {
                onclick(u, seriesIdx, dataIdx);
              }
            }
          });
        },
      ],
      setCursor: [
        (u) => {
          let c = u.cursor;

          if (dataIdx != c.idx) {
            dataIdx = c.idx;

            if (seriesIdx != null) setTooltip(u);
          }
        },
      ],
      setSeries: [
        (u, sidx) => {
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

function genPlotOpts({
  title,
  height,
  yAxisLabel,
  series,
  commits,
  alpha = 0.3,
  prox = 5,
}) {
  return {
    title,
    width: window.innerWidth * 0.9,
    height,
    series,
    legend: {
      live: false,
    },
    focus: {
      alpha,
    },
    cursor: {
      focus: {
        prox,
      },
      drag: {
        x: true,
        y: true,
      },
    },
    axes: [
      {},
      {
        label: yAxisLabel,
        space: 24,
        values: (_self, splits) => {
          return splits.map((v) => v / 1e9 + " sec");
        },
      },
    ],
    plugins: [
      tooltipPlugin({
        onclick(_u, _seriesIdx, dataIdx) {
          let thisCommit = commits[dataIdx][1];
          let prevCommit = (commits[dataIdx - 1] || [null, null])[1];
          window.open(`/compare.html?start=${prevCommit}&end=${thisCommit}`);
        },
        commits,
      }),
    ],
  };
}

export function renderPlots(data: BootstrapData, selector: BootstrapSelector) {
  let byChartSeriesOpts = [{}];
  let xVals = data.commits.map((c) => c[0]);
  let byChartPlotData = [xVals];
  // https://sashamaps.net/docs/resources/20-colors/
  let colors = [
    "#e6194b",
    "#3cb44b",
    "#ffe119",
    "#4363d8",
    "#f58231",
    "#911eb4",
    "#46f0f0",
    "#f032e6",
    "#a09b13",
    "#0ab0be",
    "red",
    "green",
    "blue",
    "purple",
  ];
  let crates = Object.keys(data.by_crate_build_times).sort();
  for (let crate of crates) {
    byChartPlotData.push(data.by_crate_build_times[crate]);
    byChartSeriesOpts.push({
      label: crate,
      stroke: colors.length ? colors.pop() : "black",
    });
  }

  let byChartPlotOpts = genPlotOpts({
    title: `Bootstrap time for crates >= ${selector.min_seconds} seconds`,
    height: window.innerHeight * 0.56,
    yAxisLabel: "",
    series: byChartSeriesOpts,
    commits: data.commits,
  });

  new uPlot(
    byChartPlotOpts,
    byChartPlotData as any as TypedArray[],
    document.querySelector<HTMLElement>("#byCrateChart")
  );

  let totalPlotData = [xVals, data.total_build_times];
  let totalPlotOpts = genPlotOpts({
    title: "Total bootstrap time",
    height: window.innerHeight * 0.26,
    yAxisLabel: "",
    series: [{}, {label: "rustc", stroke: "#7cb5ec"}],
    commits: data.commits,
  });

  new uPlot(
    totalPlotOpts,
    totalPlotData as any as TypedArray[],
    document.querySelector<HTMLElement>("#totalChart")
  );
}
