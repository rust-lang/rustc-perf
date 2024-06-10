<script setup lang="ts">
import {TestCaseComparison} from "../data";
import Tooltip from "../tooltip.vue";
import {percentClass} from "../shared";
import {RuntimeTestCase} from "./common";
import {computed} from "vue";
import Accordion from "../../../components/accordion.vue";
import {testCaseKey} from "./common";
import {ArtifactDescription} from "../types";
import BenchmarkDetailGraph from "./benchmark-detail-graph.vue";

const props = defineProps<{
  comparisons: TestCaseComparison<RuntimeTestCase>[];
  hasNonRelevant: boolean;
  showRawData: boolean;
  metric: string;
  commitA: ArtifactDescription;
  commitB: ArtifactDescription;
}>();

function prettifyRawNumber(number: number): string {
  return number.toLocaleString();
}

const columnCount = computed(() => {
  const base = 5;
  if (props.showRawData) {
    return base + 2;
  }
  return base;
});

const unit = computed(() => {
  // The DB stored wall-time data in nanoseconds for runtime benchmarks, so it is
  // hardcoded here
  if (props.metric == "wall-time") {
    return "ns";
  } else {
    return null;
  }
});
</script>

<template>
  <div class="bench-table">
    <slot name="header"></slot>
    <div v-if="comparisons.length === 0" style="text-align: center">
      {{ hasNonRelevant ? "No relevant results" : "No results" }}
    </div>
    <table v-else class="benches compare">
      <thead>
        <tr>
          <th>Benchmark</th>
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
          <Accordion :id="testCaseKey(comparison.testCase)">
            <template v-slot:default>
              <td>
                {{ comparison.testCase.benchmark }}
              </td>
              <td>
                <div class="numeric-aligned">
                  <div>
                    <span v-bind:class="percentClass(comparison.percent)">
                      {{ comparison.percent.toFixed(2) }}%
                    </span>
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
                <abbr :title="comparison.datumA.toString()"
                  >{{ prettifyRawNumber(comparison.datumA) }}{{ unit }}</abbr
                >
              </td>
              <td v-if="showRawData" class="numeric">
                <abbr :title="comparison.datumB.toString()"
                  >{{ prettifyRawNumber(comparison.datumB) }}{{ unit }}</abbr
                >
              </td>
            </template>
            <template v-slot:expanded>
              <td :colspan="columnCount">
                <BenchmarkDetailGraph
                  :baseArtifact="commitA"
                  :artifact="commitB"
                  :metric="metric"
                  :testCase="comparison.testCase"
                />
              </td>
            </template>
          </Accordion>
        </template>
      </tbody>
    </table>
  </div>
</template>

<style scoped lang="scss">
.benches {
  font-size: medium;
  table-layout: fixed;
  border-collapse: collapse;

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

.columns {
  display: flex;
  flex-wrap: wrap;
  gap: 15px;
  margin: 10px 0;

  .grow {
    flex-grow: 1;
  }

  &.graphs {
    flex-wrap: nowrap;
  }
}

.rows {
  display: flex;
  flex-direction: column;
  gap: 10px;

  &.center-items {
    align-items: center;
  }
}

.graphs {
  margin-top: 15px;
}
</style>
