<script setup lang="ts">
import {TestCaseComparison} from "../data";
import {ArtifactDescription} from "../types";
import {CompileBenchmarkMap, CompileTestCase} from "./common";
import {default as Table} from "../comparisons-table.vue";

const props = defineProps<{
  id: string;
  comparisons: TestCaseComparison<CompileTestCase>[];
  benchmarkMap: CompileBenchmarkMap;
  hasNonRelevant: boolean;
  showRawData: boolean;
  commitA: ArtifactDescription;
  commitB: ArtifactDescription;
  stat: string;
}>();

function benchmarkLink(benchmark: string): string {
  return `https://github.com/rust-lang/rustc-perf/tree/master/collector/compile-benchmarks/${benchmark}`;
}

function graphLink(
  commit: ArtifactDescription,
  stat: string,
  testCase: CompileTestCase
): string {
  let date = new Date(commit.date);
  // Move to `30 days ago` to display history of the test case
  date.setUTCDate(date.getUTCDate() - 30);
  let year = date.getUTCFullYear();
  let month = (date.getUTCMonth() + 1).toString().padStart(2, "0");
  let day = date.getUTCDate().toString().padStart(2, "0");
  let start = `${year}-${month}-${day}`;

  let end = commit.commit;
  const {benchmark, profile, scenario} = testCase;
  return `/index.html?start=${start}&end=${end}&benchmark=${benchmark}&profile=${profile}&scenario=${scenario}&stat=${stat}`;
}

function detailedQueryPercentLink(
  commit: ArtifactDescription,
  baseCommit: ArtifactDescription,
  testCase: CompileTestCase
): string {
  const {benchmark, profile, scenario} = testCase;
  return `/detailed-query.html?commit=${commit.commit}&base_commit=${baseCommit.commit}&benchmark=${benchmark}-${profile}&scenario=${scenario}`;
}

function detailedQueryRawDataLink(
  commit: ArtifactDescription,
  testCase: CompileTestCase
) {
  const {benchmark, profile, scenario} = testCase;
  return `/detailed-query.html?commit=${commit.commit}&benchmark=${benchmark}-${profile}&scenario=${scenario}`;
}

function generateBenchmarkTooltip(testCase: CompileTestCase): string {
  const metadata = props.benchmarkMap[testCase.benchmark] ?? null;
  if (metadata === null) {
    return "<No metadata found>";
  }
  let tooltip = `Benchmark: ${testCase.benchmark}
Category: ${metadata.category}
`;
  if (metadata.binary !== null) {
    tooltip += `Artifact: ${metadata.binary ? "binary" : "library"}\n`;
  }
  if (metadata.iterations !== null) {
    tooltip += `Iterations: ${metadata.iterations}\n`;
  }
  if (testCase.profile === "opt" && metadata.release_profile !== null) {
    const {lto, debug, codegen_units} = metadata.release_profile;
    if (lto !== null) {
      tooltip += `LTO: ${lto}\n`;
    }
    if (debug !== null) {
      tooltip += `Debuginfo: ${debug}\n`;
    }
    if (codegen_units !== null) {
      tooltip += `Codegen units: ${codegen_units}\n`;
    }
  }

  return tooltip;
}
</script>

<template>
  <Table
    :id="id"
    :has-non-relevant="hasNonRelevant"
    :comparisons="comparisons"
    :show-raw-data="showRawData"
    :percent-link="
      (testCase) => detailedQueryPercentLink(commitB, commitA, testCase)
    "
    :raw-data-a-link="(testCase) => detailedQueryRawDataLink(commitA, testCase)"
    :raw-data-b-link="(testCase) => detailedQueryRawDataLink(commitB, testCase)"
  >
    <template #header>
      <slot name="header"></slot>
    </template>
    <template #case-header>
      <th>Benchmark</th>
      <th>Profile</th>
      <th>Scenario</th>
    </template>
    <template #case-row="{testCase}">
      <td :title="generateBenchmarkTooltip(testCase)">
        <a
          v-bind:href="benchmarkLink(testCase.benchmark)"
          class="silent-link"
          target="_blank"
        >
          {{ testCase.benchmark }}
        </a>
      </td>
      <td>
        <a
          v-bind:href="graphLink(commitB, stat, testCase)"
          target="_blank"
          class="silent-link"
        >
          {{ testCase.profile }}
        </a>
      </td>
      <td>{{ testCase.scenario }}</td>
    </template>
  </Table>
</template>
