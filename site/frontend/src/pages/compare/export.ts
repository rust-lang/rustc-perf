import {computeSummary, TestCase} from "./data";

export function exportToMarkdown(testCases: TestCase[]) {
  function changesTable(cases) {
    let data = "| Benchmark | Profile | Scenario | % Change | Significance Factor |\n";
    data += "|:---:|:---:|:---:|:---:|:---:|\n"

    for (const testCase of cases) {
      data += `| ${testCase.benchmark} | ${testCase.profile} | ${testCase.scenario} `;
      data += `| ${testCase.percent.toFixed(2)}% | ${testCase.significanceFactor.toFixed(2)}x\n`;
    }

    return data;
  }

  const summary = computeSummary(testCases);
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
  content += "| | Range | Mean | Count |\n";
  content += "|:---:|:---:|:---:|:---:|\n";
  content += `| Regressions | ${formatRange(regressions.range)} | ${regressions.average.toFixed(2)}% | ${regressions.count} |\n`;
  content += `| Improvements | ${formatRange(improvements.range)} | ${improvements.average.toFixed(2)}% | ${improvements.count} |\n`;
  content += `| All | ${formatRange(all.range)} | ${all.average.toFixed(2)}% | ${all.count} |\n\n`;

  content += "# Primary benchmarks\n";
  content += changesTable(testCases.filter(testCase => testCase.category === "primary"));

  content += "\n# Secondary benchmarks\n";
  content += changesTable(testCases.filter(testCase => testCase.category === "secondary"));

  downloadFile(content, "perf-summary.md");
}

function downloadFile(content, name) {
  const blob = new Blob([content], {
    type: "text/markdown"
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
