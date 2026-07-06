import {computeSummary, TestCaseComparison} from "../data";
import {CompileTestCase} from "./common";

export function exportToMarkdown(
  comparisons: TestCaseComparison<CompileTestCase>[]
) {
  function changesTable(comparisons: TestCaseComparison<CompileTestCase>[]) {
    let columns = ["Benchmark", "% Change", "Significance Factor"];

    const toMarkdownRow = (cells: string[]) => {
      return `| ${cells.join(" | ")} |\n`;
    };

    let data = toMarkdownRow(columns);
    data += toMarkdownRow(Array(columns.length).fill(":---:")).replace(
      / /g,
      ""
    );

    for (const comparison of comparisons) {
      let cells = [
        comparison.test_case.benchmark,
        `${comparison.percent.toFixed(2)}%`,
        `${comparison.comparison.significance_factor.toFixed(2)}x`,
      ];

      data += toMarkdownRow(cells);
    }

    return data;
  }

  const summary = computeSummary(comparisons);
  const regressions = summary.regressions;
  const improvements = summary.improvements;
  const all = summary.all;

  function formatRange(range) {
    if (range.length === 0) {
      return "-";
    }
    return `${range[0].toFixed(2)}%, ${range[1].toFixed(2)}%`;
  }

  let content = "# Summary\n";
  content += "| | Range | Geo mean | Median | Count |\n";
  content += "|:---:|:---:|:---:|:---:|:---:|\n";
  content += `| Regressions | ${formatRange(
    regressions.range
  )} | ${regressions.geomean.toFixed(2)}% | ${regressions.median.toFixed(
    2
  )}% | ${regressions.count} |\n`;
  content += `| Improvements | ${formatRange(
    improvements.range
  )} | ${improvements.geomean.toFixed(2)}% | ${improvements.median.toFixed(
    2
  )}% | ${improvements.count} |\n`;
  content += `| All | ${formatRange(all.range)} | ${all.geomean.toFixed(
    2
  )}% | ${all.median.toFixed(2)}% | ${all.count} |\n\n`;

  content += "# Benchmarks\n";
  content += changesTable(comparisons);

  downloadFile(content, "perf-summary.md");
}

function downloadFile(content, name) {
  const blob = new Blob([content], {
    type: "text/markdown",
  });

  const url = window.URL.createObjectURL(blob);

  // Create a fake link (taken from https://stackoverflow.com/a/9834261/1107768)
  const a = document.createElement("a");
  a.style.display = "none";
  a.href = url;
  a.download = name;

  const inserted = document.body.appendChild(a);
  a.click();

  window.URL.revokeObjectURL(url);
  inserted.remove();
}
