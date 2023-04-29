<script setup lang="ts">
import {TestCase} from "../data";
import Tooltip from "../tooltip.vue";
import {ArtifactDescription} from "../types";
import {percentClass} from "../shared";

const props = defineProps<{
  id: string;
  cases: TestCase[];
  hasNonRelevant: boolean;
  showRawData: boolean;
  commitA: ArtifactDescription;
  commitB: ArtifactDescription;
  stat: string;
}>();

function benchmarkLink(benchmark: string): string {
  return `https://github.com/rust-lang/rustc-perf/tree/master/collector/compile-benchmarks/${benchmark}`;
}

function graphLink(commit: ArtifactDescription, stat: string, testCase: TestCase): string {
  let date = new Date(commit.date);
  // Move to `30 days ago` to display history of the test case
  date.setUTCDate(date.getUTCDate() - 30);
  let year = date.getUTCFullYear();
  let month = (date.getUTCMonth() + 1).toString().padStart(2, "0");
  let day = date.getUTCDate().toString().padStart(2, "0");
  let start = `${year}-${month}-${day}`;

  let end = commit.commit;
  return `/index.html?start=${start}&end=${end}&benchmark=${testCase.benchmark}&profile=${testCase.profile}&scenario=${testCase.scenario}&stat=${stat}`;
}

function detailedQueryPercentLink(commit: ArtifactDescription, baseCommit: ArtifactDescription, testCase: TestCase): string {
  return `/detailed-query.html?commit=${commit.commit}&base_commit=${baseCommit.commit}&benchmark=${testCase.benchmark}-${testCase.profile}&scenario=${testCase.scenario}`;
}

function detailedQueryRawDataLink(commit: ArtifactDescription, testCase: TestCase) {
  return `/detailed-query.html?commit=${commit.commit}&benchmark=${testCase.benchmark}-${testCase.profile}&scenario=${testCase.scenario}`;
}

function prettifyRawNumber(number: number): string {
  return number.toLocaleString();
}
</script>

<template>
  <div class="bench-table" :id="id">
    <slot name="header"></slot>
    <div v-if="cases.length === 0" style="text-align: center;">
      {{ hasNonRelevant ? "No relevant results" : "No results" }}
    </div>
    <table v-else class="benches compare">
      <thead>
      <tr>
        <th>Benchmark</th>
        <th>Profile</th>
        <th>Scenario</th>
        <th>% Change</th>
        <th>
          Significance Threshold
          <Tooltip>
            The minimum % change that is considered significant. The higher the significance
            threshold, the noisier a test case is.
            You can see <a
            href="https://github.com/rust-lang/rustc-perf/blob/master/docs/comparison-analysis.md#what-makes-a-test-result-significant">
            here</a> how the significance threshold is calculated.
          </Tooltip>
        </th>
        <th>
          Significance Factor
          <Tooltip>
            How much a particular result is over the significance threshold. A factor of 2.50x
            means the result is 2.5 times over the significance threshold.
          </Tooltip>
        </th>
        <th v-if="showRawData">Before</th>
        <th v-if="showRawData">After</th>
      </tr>
      </thead>
      <tbody>
      <template v-for="testCase in cases">
        <tr>
          <td>
            <a v-bind:href="benchmarkLink(testCase.benchmark)"
               class="silent-link"
               target="_blank">
              {{ testCase.benchmark }}
            </a>
          </td>
          <td>
            <a v-bind:href="graphLink(commitB, stat, testCase)" target="_blank" class="silent-link">
              {{ testCase.profile }}
            </a>
          </td>
          <td>{{ testCase.scenario }}</td>
          <td>
            <a v-bind:href="detailedQueryPercentLink(commitB, commitA, testCase)">
              <span v-bind:class="percentClass(testCase.percent)">
                  {{ testCase.percent.toFixed(2) }}%
              </span>
            </a>
          </td>
          <td>
            {{ testCase.significanceThreshold ? testCase.significanceThreshold.toFixed(2) + "%" : "-"
            }}
          </td>
          <td>
            {{ testCase.significanceFactor ? testCase.significanceFactor.toFixed(2) + "x" : "-" }}
          </td>
          <td v-if="showRawData" class="numeric">
            <a v-bind:href="detailedQueryRawDataLink(commitA, testCase)">
              <abbr :title="testCase.datumA">{{ prettifyRawNumber(testCase.datumA) }}</abbr>
            </a>
          </td>
          <td v-if="showRawData" class="numeric">
            <a v-bind:href="detailedQueryRawDataLink(commitB, testCase)">
              <abbr :title="testCase.datumB">{{ prettifyRawNumber(testCase.datumB) }}</abbr>
            </a>
          </td>
        </tr>
      </template>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.benches {
    font-size: medium;
}

.benches tbody::before {
    content: '';
    display: block;
    height: 10px;
}

.benches tbody:first-child th {
    text-align: center;
}

.benches tbody:not(:first-child) th {
    border-right: dotted 1px;
}

.benches th {
    text-align: center;
    width: 25%;
    min-width: 50px;
}

.benches td {
    text-align: center;
    width: 25%;
}

.benches td.numeric {
    text-align: right;
}

.bench-table {
    margin-top: 10px;
}

.silent-link {
    color: inherit;
}
</style>
