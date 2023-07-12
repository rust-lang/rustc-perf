<script setup lang="tsx" generic="T">
import Tooltip from "./tooltip.vue";
import {TestCaseComparison} from "./data";
import {percentClass} from "./shared";
import {h} from "vue";

defineSlots<{
  // Header of the whole table
  header?: (props: {}) => any;
  // Headers for columns that describe the test case
  caseHeader?: (props: {}) => any;
  // Columns with values of the test case
  caseRow?: (props: {testCase: T}) => any;
}>();

const props = withDefaults(
  defineProps<{
    id: string;
    comparisons: TestCaseComparison<T>[];
    hasNonRelevant: boolean;
    showRawData: boolean;
    percentLink?: (testCase: T) => string | null;
    rawDataALink?: (testCase: T) => string | null;
    rawDataBLink?: (testCase: T) => string | null;
  }>(),
  {
    percentLink: () => null,
    rawDataALink: () => null,
    rawDataBLink: () => null,
  }
);

function prettifyRawNumber(number: number): string {
  return number.toLocaleString();
}

function WithLink({link}: {link: string | null}, {slots}) {
  if (link !== null) {
    return <a href={link}>{slots.default()}</a>;
  } else {
    return slots.default();
  }
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
          <slot name="case-header"></slot>
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
            <slot name="case-row" :test-case="comparison.testCase"></slot>
            <td>
              <div class="numeric-aligned">
                <div>
                  <WithLink :link="percentLink(comparison.testCase)">
                    <span :class="percentClass(comparison.percent)">
                      {{ comparison.percent.toFixed(2) }}%
                    </span>
                  </WithLink>
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
              <WithLink :link="rawDataALink(comparison.testCase)">
                <abbr :title="comparison.datumA.toString()">{{
                  prettifyRawNumber(comparison.datumA)
                }}</abbr>
              </WithLink>
            </td>
            <td v-if="showRawData" class="numeric">
              <WithLink :link="rawDataBLink(comparison.testCase)">
                <abbr :title="comparison.datumB.toString()">{{
                  prettifyRawNumber(comparison.datumB)
                }}</abbr>
              </WithLink>
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

.benches th,
:slotted(.benches th) {
  text-align: center;
  min-width: 50px;
}

.benches td,
:slotted(.benches td) {
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

.silent-link,
:slotted(.silent-link) {
  color: inherit;
}
</style>
