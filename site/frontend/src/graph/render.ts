import uPlot, {TypedArray} from "uplot";
import {CompileGraphData, GraphsSelector, RuntimeGraphData} from "./data";

const commonCacheStateColors = {
  full: "#7cb5ec",
  "incr-full": "#434348",
  "incr-unchanged": "#90ed7d",
  "incr-patched: println": "#f7a35c",
};

const otherCacheStateColors = [
  "#8085e9",
  "#f15c80",
  "#e4d354",
  "#2b908f",
  "#f45b5b",
  "#91e8e1",
];
const interpolatedColor = "#fcb0f1";
const profiles = ["Check", "Debug", "Opt", "Doc"];

function tooltipPlugin({
  onclick,
  commits,
  isInterpolated,
  absoluteMode,
  shiftX = 10,
  shiftY = 10,
}) {
  let tooltipLeftOffset = 0;
  let tooltipTopOffset = 0;

  const tooltip = document.createElement("div");
  tooltip.className = "u-tooltip";

  let seriesIdx = null;
  let dataIdx = null;

  const fmtDate = uPlot.fmtDate("{M}/{D}/{YY} {h}:{mm}:{ss} {AA}");

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

    tooltip.style.borderColor = isInterpolated(dataIdx)
      ? interpolatedColor
      : u.series[seriesIdx].stroke;

    let trailer = "";
    if (absoluteMode) {
      let pctSinceStart = (
        ((u.data[seriesIdx][dataIdx] - u.data[seriesIdx][0]) /
          u.data[seriesIdx][0]) *
        100
      ).toFixed(2);
      trailer =
        uPlot.fmtNum(u.data[seriesIdx][dataIdx]) +
        " (" +
        pctSinceStart +
        "% since start)";
    } else {
      trailer = uPlot.fmtNum(u.data[seriesIdx][dataIdx]) + "% since start";
    }
    tooltip.textContent =
      fmtDate(new Date(u.data[0][dataIdx] * 1e3)) +
      " - " +
      commits[dataIdx][1].slice(0, 10) +
      "\n" +
      trailer;
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

// Taken from https://leeoniya.github.io/uPlot/demos/zoom-wheel.html
function wheelZoomPlugin(opts) {
  let factor = opts.factor || 0.75;

  let xMin, xMax, yMin, yMax, xRange, yRange;

  function clamp(nRange, nMin, nMax, fRange, fMin, fMax) {
    if (nRange > fRange) {
      nMin = fMin;
      nMax = fMax;
    } else if (nMin < fMin) {
      nMin = fMin;
      nMax = fMin + nRange;
    } else if (nMax > fMax) {
      nMax = fMax;
      nMin = fMax - nRange;
    }

    return [nMin, nMax];
  }

  return {
    hooks: {
      ready: (u) => {
        xMin = u.scales.x.min;
        xMax = u.scales.x.max;
        yMin = u.scales.y.min;
        yMax = u.scales.y.max;

        xRange = xMax - xMin;
        yRange = yMax - yMin;

        let over = u.over;
        let rect = over.getBoundingClientRect();

        // wheel drag pan
        over.addEventListener("mousedown", (e) => {
          if (e.button == 1) {
            //	plot.style.cursor = "move";
            e.preventDefault();

            let left0 = e.clientX;
            //	let top0 = e.clientY;

            let scXMin0 = u.scales.x.min;
            let scXMax0 = u.scales.x.max;

            let xUnitsPerPx = u.posToVal(1, "x") - u.posToVal(0, "x");

            function onmove(e) {
              e.preventDefault();

              let left1 = e.clientX;
              //	let top1 = e.clientY;

              let dx = xUnitsPerPx * (left1 - left0);

              u.setScale("x", {
                min: scXMin0 - dx,
                max: scXMax0 - dx,
              });
            }

            function onup(_e) {
              document.removeEventListener("mousemove", onmove);
              document.removeEventListener("mouseup", onup);
            }

            document.addEventListener("mousemove", onmove);
            document.addEventListener("mouseup", onup);
          }
        });

        // wheel scroll zoom
        over.addEventListener("wheel", (e) => {
          e.preventDefault();

          let {left, top} = u.cursor;

          let leftPct = left / rect.width;
          let btmPct = 1 - top / rect.height;
          let xVal = u.posToVal(left, "x");
          let yVal = u.posToVal(top, "y");
          let oxRange = u.scales.x.max - u.scales.x.min;
          let oyRange = u.scales.y.max - u.scales.y.min;

          let nxRange = e.deltaY < 0 ? oxRange * factor : oxRange / factor;
          let nxMin = xVal - leftPct * nxRange;
          let nxMax = nxMin + nxRange;
          [nxMin, nxMax] = clamp(nxRange, nxMin, nxMax, xRange, xMin, xMax);

          let nyRange = e.deltaY < 0 ? oyRange * factor : oyRange / factor;
          let nyMin = yVal - btmPct * nyRange;
          let nyMax = nyMin + nyRange;
          [nyMin, nyMax] = clamp(nyRange, nyMin, nyMax, yRange, yMin, yMax);

          u.batch(() => {
            u.setScale("x", {
              min: nxMin,
              max: nxMax,
            });

            u.setScale("y", {
              min: nyMin,
              max: nyMax,
            });
          });
        });
      },
    },
  };
}

function genPlotOpts({
  width,
  height,
  yAxisLabel,
  series,
  commits,
  stat,
  isInterpolated,
  alpha = 0.3,
  prox = 5,
  absoluteMode,
  hooks,
}) {
  return {
    width,
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
    scales: {
      y: {
        range: (_self, dataMin, dataMax) =>
          uPlot.rangeNum(absoluteMode ? 0 : dataMin, dataMax, 0.2, true),
      },
    },
    axes: [
      {
        grid: {
          show: false,
        },
      },
      {
        label: yAxisLabel,
        space: 24,
        values: (_self, splits) => {
          return splits.map((v) => {
            return v >= 1e12
              ? v / 1e12 + "T"
              : v >= 1e9
              ? v / 1e9 + "G"
              : v >= 1e6
              ? v / 1e6 + "M"
              : v >= 1e3
              ? v / 1e3 + "k"
              : v;
          });
        },
      },
    ],
    plugins: [
      {
        hooks: {
          drawAxes: [
            (u) => {
              let {ctx} = u;
              let {top, height} = u.bbox;

              const interpolatedColorWithAlpha = "#fcb0f15f";

              ctx.strokeStyle = interpolatedColorWithAlpha;
              ctx.beginPath();

              let [i0, i1] = u.series[0].idxs;

              for (let j = i0; j <= i1; j++) {
                let v = u.data[0][j];

                if (isInterpolated(j)) {
                  let cx = Math.round(u.valToPos(v, "x", true));
                  ctx.moveTo(cx, top);
                  ctx.lineTo(cx, top + height);
                }
              }

              ctx.closePath();
              ctx.stroke();
            },
          ],
          ...hooks,
        },
      },
      tooltipPlugin({
        onclick(_u, _seriesIdx, dataIdx) {
          // No earlier data to show
          if (dataIdx == 0) return;

          let thisCommit = commits[dataIdx][1];
          let prevCommit = commits[dataIdx - 1][1];
          window.open(
            `/compare.html?start=${prevCommit}&end=${thisCommit}&stat=${stat}`
          );
        },
        commits,
        isInterpolated,
        absoluteMode,
      }),
      wheelZoomPlugin({factor: 0.75}),
    ],
  };
}

function normalizeData(data: CompileGraphData) {
  function optInterpolated(profile) {
    for (const scenario in profile) {
      profile[scenario].interpolated_indices = new Set(
        profile[scenario].interpolated_indices
      );
    }

    return profile;
  }

  for (const name of Object.keys(data.benchmarks)) {
    for (const profile of profiles) {
      if (data.benchmarks[name].hasOwnProperty(profile)) {
        data.benchmarks[name][profile.toLowerCase()] = optInterpolated(
          data.benchmarks[name][profile]
        );
        delete data.benchmarks[name][profile];
      }
    }
  }
}

export type GraphRenderOpts = {
  // Width of the graph
  width: number;
  // Render a title above the graph
  renderTitle?: boolean;
  // Function that can be used to hook into the rendering process
  hooks?: {drawSeries: (uPlot, number) => void};
};

// Renders the plots data with the given parameters from the `selector` into
// the passed DOM element.
export function renderPlots(
  data: CompileGraphData,
  selector: GraphsSelector,
  plotElement: HTMLElement,
  opts: GraphRenderOpts
) {
  const renderTitle = opts.renderTitle ?? true;
  const hooks = opts.hooks ?? {};
  const width = opts.width;

  normalizeData(data);

  const benchNames = Object.keys(data.benchmarks).sort();

  for (const benchName of benchNames) {
    let profiles = data.benchmarks[benchName];

    let i = 0;

    for (let profile in profiles) {
      let scenarios = profiles[profile];
      let scenarioNames = Object.keys(scenarios);
      scenarioNames.sort();

      let yAxis = selector.stat;
      let yAxisUnit = null;

      switch (selector.stat) {
        case "instructions:u":
          yAxis = "CPU instructions";
          yAxisUnit = "count";
          break;
        case "cycles:u":
          yAxis = "CPU cycles";
          yAxisUnit = "count";
          break;
        case "cpu-clock":
          yAxis = "CPU clock";
          yAxisUnit = "seconds";
          break;
        case "task-clock":
          yAxis = "Task clock";
          yAxisUnit = "seconds";
          break;
        case "wall-time":
          yAxis = "Wall time";
          yAxisUnit = "seconds";
          break;
        case "max-rss":
          yAxis = "Maximum resident set size";
          yAxisUnit = "kB";
          break;
        case "faults":
          yAxis = "Faults";
          yAxisUnit = "count";
          break;
      }

      if (selector.kind == "raw" && benchName == "Summary") {
        yAxisUnit = "relative";
      } else if (selector.kind == "percentfromfirst") {
        yAxisUnit = "% change from first";
      } else if (selector.kind == "percentrelative") {
        yAxisUnit = "% change from previous";
      }

      yAxis = yAxisUnit ? `${yAxis} (${yAxisUnit})` : yAxis;
      let yAxisLabel = i == 0 ? yAxis : null;

      let seriesOpts = [{}];

      let xVals = data.commits.map((c) => c[0]);

      let plotData = [xVals];

      let otherColorIdx = 0;

      for (let scenarioName of scenarioNames) {
        let yVals = scenarios[scenarioName].points;
        let color =
          commonCacheStateColors[scenarioName] ||
          otherCacheStateColors[otherColorIdx++];

        plotData.push(yVals);

        seriesOpts.push({
          label: scenarioName,
          width: devicePixelRatio,
          stroke: color,
        });
      }

      let indices = scenarios[Object.keys(scenarios)[0]].interpolated_indices;

      let plotOpts = genPlotOpts({
        width,
        height: 300,
        yAxisLabel,
        series: seriesOpts,
        commits: data.commits,
        stat: selector.stat,
        isInterpolated(dataIdx: number) {
          return indices.has(dataIdx);
        },
        absoluteMode: selector.kind == "raw",
        hooks,
      });
      if (renderTitle) {
        plotOpts["title"] = `${benchName}-${profile}`;
      }

      new uPlot(plotOpts, plotData as any as TypedArray[], plotElement);

      i++;
    }
  }
}

export function renderRuntimePlots(
  data: RuntimeGraphData,
  selector: GraphsSelector,
  plotElement: HTMLElement,
  opts: GraphRenderOpts
) {
  const renderTitle = opts.renderTitle ?? true;
  const hooks = opts.hooks ?? {};
  const width = opts.width;

  const benchNames = Object.keys(data.benchmarks).sort();

  for (const benchName of benchNames) {
    const benchmark = data.benchmarks[benchName];
    let i = 0;

    let yAxis = selector.stat;
    let yAxisUnit = null;

    switch (selector.stat) {
      case "instructions:u":
        yAxis = "CPU instructions";
        yAxisUnit = "count";
        break;
      case "cycles:u":
        yAxis = "CPU cycles";
        yAxisUnit = "count";
        break;
      case "cpu-clock":
        yAxis = "CPU clock";
        yAxisUnit = "seconds";
        break;
      case "task-clock":
        yAxis = "Task clock";
        yAxisUnit = "seconds";
        break;
      case "wall-time":
        yAxis = "Wall time";
        yAxisUnit = "seconds";
        break;
      case "max-rss":
        yAxis = "Maximum resident set size";
        yAxisUnit = "kB";
        break;
      case "faults":
        yAxis = "Faults";
        yAxisUnit = "count";
        break;
    }

    if (selector.kind == "raw" && benchName == "Summary") {
      yAxisUnit = "relative";
    } else if (selector.kind == "percentfromfirst") {
      yAxisUnit = "% change from first";
    } else if (selector.kind == "percentrelative") {
      yAxisUnit = "% change from previous";
    }

    yAxis = yAxisUnit ? `${yAxis} (${yAxisUnit})` : yAxis;
    let yAxisLabel = i == 0 ? yAxis : null;

    let seriesOpts = [{}];

    let xVals = data.commits.map((c) => c[0]);

    let plotData = [xVals];

    let otherColorIdx = 0;

    let yVals = benchmark.points;
    let color = otherCacheStateColors[otherColorIdx++];

    plotData.push(yVals);

    seriesOpts.push({
      label: benchName,
      width: devicePixelRatio,
      stroke: color,
    });

    let indices = new Set(benchmark.interpolated_indices);

    let plotOpts = genPlotOpts({
      width,
      height: 300,
      yAxisLabel,
      series: seriesOpts,
      commits: data.commits,
      stat: selector.stat,
      isInterpolated(dataIdx: number) {
        return indices.has(dataIdx);
      },
      absoluteMode: selector.kind == "raw",
      hooks,
    });
    if (renderTitle) {
      plotOpts["title"] = `${benchName}`;
    }

    new uPlot(plotOpts, plotData as any as TypedArray[], plotElement);

    i++;
  }
}
