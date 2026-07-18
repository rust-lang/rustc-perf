import uPlot, {AlignedData} from "uplot";
import {ToolchainData, ToolchainSelector} from "./state";

function tooltipPlugin({onclick, commits, kind, shiftX = 10, shiftY = 10}) {
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

    const data = u.data[seriesIdx][dataIdx];
    let dataLabel = "";
    if (kind === "time") {
      dataLabel = `${normalizeValue(data, kind)} seconds`;
    } else if (kind === "size") {
      dataLabel = `${normalizeValue(data, kind)} MiB`;
    }

    tooltip.textContent = `${formatDate(
      new Date(u.data[0][dataIdx] * 1e3)
    )} - ${commits[dataIdx][1].slice(0, 10)}
${u.series[seriesIdx].label}: ${dataLabel}`;
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

function normalizeValue(data: number, kind: "size" | "time"): string {
  if (kind === "time") {
    return (data / 1e9).toFixed(2);
  } else if (kind === "size") {
    return (data / (1024 * 1024)).toFixed(2);
  }
  return data.toFixed(2);
}

function genPlotOpts({
  title,
  height,
  yAxisLabel,
  series,
  commits,
  alpha = 0.3,
  prox = 5,
  kind = "time",
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
          if (kind === "time") {
            return splits.map((v) => `${normalizeValue(v, kind)} sec`);
          } else if (kind === "size") {
            return splits.map((v) => `${normalizeValue(v, kind)} MiB`);
          } else {
            throw new Error(`Unknown chart kind ${kind}`);
          }
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
        kind,
      }),
    ],
  };
}

export function renderPlots(data: ToolchainData, selector: ToolchainSelector) {
  let xVals = data.commits.map((c) => c[0]);
  // https://sashamaps.net/docs/resources/20-colors/
  const colors = [
    "purple",
    "blue",
    "green",
    "red",
    "#0ab0be",
    "#a09b13",
    "#f032e6",
    "#46f0f0",
    "#911eb4",
    "#f58231",
    "#4363d8",
    "#ffe119",
    "#3cb44b",
    "#e6194b",
  ];

  let artifactSizeSeriesOpts = [{}];
  let artifactSizePlotData = [xVals];

  let components = Object.keys(data.artifact_sizes).sort();
  const artifactSizeTotal = [];
  for (let i = 0; i < xVals.length; i++) {
    let total = 0;
    for (const component of components) {
      const size = data.artifact_sizes[component][i];
      if (size !== null && size !== undefined) {
        total += size;
      }
    }
    artifactSizeTotal.push(total);
  }
  artifactSizePlotData.push(artifactSizeTotal);
  artifactSizeSeriesOpts.push({
    label: "Total",
    stroke: "black",
  });

  for (const [colorIndex, component] of components.entries()) {
    artifactSizePlotData.push(data.artifact_sizes[component]);
    artifactSizeSeriesOpts.push({
      label: component,
      stroke: colorIndex < colors.length ? colors[colorIndex] : "black",
    });
  }

  let artifactSizePlotOpts = genPlotOpts({
    title: `Toolchain size`,
    height: window.innerHeight * 0.4,
    yAxisLabel: "",
    series: artifactSizeSeriesOpts,
    commits: data.commits,
    kind: "size",
  });

  new uPlot(
    artifactSizePlotOpts,
    artifactSizePlotData as AlignedData,
    document.querySelector<HTMLElement>("#artifactSizeChart")
  );

  let byChartSeriesOpts = [{}];
  let byChartPlotData = [xVals];
  let crates = Object.keys(data.by_crate_build_times).sort();
  for (const [colorIndex, crate] of crates.entries()) {
    byChartPlotData.push(data.by_crate_build_times[crate]);
    byChartSeriesOpts.push({
      label: crate,
      stroke: colorIndex < colors.length ? colors[colorIndex] : "black",
    });
  }

  let byChartPlotOpts = genPlotOpts({
    title: `Bootstrap time for crates >= ${selector.min_seconds} seconds`,
    height: window.innerHeight * 0.56,
    yAxisLabel: "",
    series: byChartSeriesOpts,
    commits: data.commits,
    kind: "time",
  });

  new uPlot(
    byChartPlotOpts,
    byChartPlotData as AlignedData,
    document.querySelector<HTMLElement>("#bootstrapByCrateChart")
  );

  let totalPlotData = [xVals, data.total_build_times];
  let totalPlotOpts = genPlotOpts({
    title: "Total bootstrap time",
    height: window.innerHeight * 0.26,
    yAxisLabel: "",
    series: [{}, {label: "rustc", stroke: "#7cb5ec"}],
    commits: data.commits,
    kind: "time",
  });

  new uPlot(
    totalPlotOpts,
    totalPlotData as AlignedData,
    document.querySelector<HTMLElement>("#bootstrapTotalChart")
  );
}
