<script setup lang="ts">
import {TestCaseComparison} from "../data";
import Tooltip from "../tooltip.vue";
import {ArtifactDescription} from "../types";
import {percentClass} from "../shared";
import {CompileTestCase} from "./common";

const props = defineProps<{
  id: string;
  comparisons: TestCaseComparison<CompileTestCase>[];
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
  comparison: TestCaseComparison<CompileTestCase>
): string {
  let date = new Date(commit.date);
  // Move to `30 days ago` to display history of the test case
  date.setUTCDate(date.getUTCDate() - 30);
  let year = date.getUTCFullYear();
  let month = (date.getUTCMonth() + 1).toString().padStart(2, "0");
  let day = date.getUTCDate().toString().padStart(2, "0");
  let start = `${year}-${month}-${day}`;

  let end = commit.commit;
  const {benchmark, profile, scenario} = comparison.testCase;
  return `/index.html?start=${start}&end=${end}&benchmark=${benchmark}&profile=${profile}&scenario=${scenario}&stat=${stat}`;
}

function detailedQueryPercentLink(
  commit: ArtifactDescription,
  baseCommit: ArtifactDescription,
  comparison: TestCaseComparison<CompileTestCase>
): string {
  const {benchmark, profile, scenario} = comparison.testCase;
  return `/detailed-query.html?commit=${commit.commit}&base_commit=${baseCommit.commit}&benchmark=${benchmark}-${profile}&scenario=${scenario}`;
}

function detailedQueryRawDataLink(
  commit: ArtifactDescription,
  comparison: TestCaseComparison<CompileTestCase>
) {
  const {benchmark, profile, scenario} = comparison.testCase;
  return `/detailed-query.html?commit=${commit.commit}&benchmark=${benchmark}-${profile}&scenario=${scenario}`;
}

function prettifyRawNumber(number: number): string {
  return number.toLocaleString();
}
</script>

<template>
  <div class="bench-table" :id="id">
    <slot name="header"></slot>
    <div v-if="comparisons.length === 0" style="text-align: center">
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
              The minimum % change that is considered significant. The higher
              the significance threshold, the noisier a test case is. You can
              see
              <a
                href="https://github.com/rust-lang/rustc-perf/blob/master/docs/comparison-analysis.md#what-makes-a-test-result-significant"
              >
                here</a
              >
              how the significance threshold is calculated.
            </Tooltip>
          </th>
          <th>
            Significance Factor
            <Tooltip>
              How much a particular result is over the significance threshold. A
              factor of 2.50x means the result is 2.5 times over the
              significance threshold.
            </Tooltip>
          </th>
          <th v-if="showRawData">Before</th>
          <th v-if="showRawData">After</th>
        </tr>
      </thead>
      <tbody>
        <template v-for="comparison in comparisons">
          <tr>
            <td>
              <a
                v-bind:href="benchmarkLink(comparison.testCase.benchmark)"
                class="silent-link"
                target="_blank"
              >
                {{ comparison.testCase.benchmark }}
              </a>
            </td>
            <td>
              <a
                v-bind:href="graphLink(commitB, stat, comparison)"
                target="_blank"
                class="silent-link"
              >
                {{ comparison.testCase.profile }}
              </a>
            </td>
            <td>{{ comparison.testCase.scenario }}</td>
            <td>
              <div class="numeric-aligned">
                <div>
                  <a
                    v-bind:href="
                      detailedQueryPercentLink(commitB, commitA, comparison)
                    "
                  >
                    <span v-bind:class="percentClass(comparison.percent)">
                      {{ comparison.percent.toFixed(2) }}%
                    </span>
                  </a>
                </div>
              </div>
            </td>
            <td>
              <div class="numeric-aligned">
                <div>
                  {{
                    comparison.significanceThreshold
                      ? comparison.significanceThreshold.toFixed(2) + "%"
                      : "-"
                  }}
                </div>
              </div>
            </td>
            <td>
              <div class="numeric-aligned">
                <div>
                  {{
                    comparison.significanceFactor
                      ? comparison.significanceFactor.toFixed(2) + "x"
                      : "-"
                  }}
                </div>
              </div>
            </td>
            <td v-if="showRawData" class="numeric">
              <a v-bind:href="detailedQueryRawDataLink(commitA, comparison)">
                <abbr :title="comparison.datumA.toString()">{{
                  prettifyRawNumber(comparison.datumA)
                }}</abbr>
              </a>
            </td>
            <td v-if="showRawData" class="numeric">
              <a v-bind:href="detailedQueryRawDataLink(commitB, comparison)">
                <abbr :title="comparison.datumB.toString()">{{
                  prettifyRawNumber(comparison.datumB)
                }}</abbr>
              </a>
            </td>
          </tr>
        </template>
      </tbody>
    </table>
  </div>
</template>

<style scoped lang="scss">
.benches {
  font-size: medium;
  table-layout: fixed;

  td,
  th {
    padding: 0.3em;
  }
}

.benches tbody::before {
  content: "";
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
  min-width: 50px;
}

.benches td {
  text-align: center;
  width: 25%;

  & > .numeric-aligned {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: right;

    & > div {
      width: 70px;
    }
  }

  &.numeric {
    text-align: right;
  }
}

.bench-table {
  margin-top: 10px;
}

.silent-link {
  color: inherit;
}
</style>
