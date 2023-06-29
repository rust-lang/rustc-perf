<script setup lang="ts">
import {computeSummary, SummaryGroup, TestCaseComparison} from "../data";
import Toggle from "../toggle.vue";
import SummaryTable from "../summary/summary-table.vue";
import {createPersistedRef} from "../../../storage";
import {PREF_AGGREGATIONS_OPENED} from "../prefs";
import {CompileTestCase} from "./common";

const props = defineProps<{
  cases: TestCaseComparison<CompileTestCase>[];
}>();

function calculateSummary(
  keyAttribute: keyof CompileTestCase,
  keyValue: string
): SummaryGroup {
  const benchmarks = [];
  for (const benchmark of props.cases) {
    if (benchmark.testCase[keyAttribute].startsWith(keyValue)) {
      benchmarks.push(benchmark);
    }
  }
  return computeSummary(benchmarks);
}

const opened = createPersistedRef(PREF_AGGREGATIONS_OPENED);
</script>

<template>
  <fieldset class="collapsible-section">
    <Toggle :opened="opened" @change="(value) => (opened = value)">
      <template #label>Aggregations</template>
      <template #content>
        <div>
          <div class="aggregation-section">
            <div class="header">Profile</div>
            <div class="groups">
              <div
                class="group"
                v-for="profile in ['check', 'debug', 'opt', 'doc']"
              >
                <div class="group-header">{{ profile }}</div>
                <SummaryTable
                  :summary="calculateSummary('profile', profile)"
                  :withLegend="false"
                ></SummaryTable>
              </div>
            </div>
          </div>
          <div class="aggregation-section">
            <div class="header">Scenario</div>
            <div class="groups">
              <div
                class="group"
                v-for="scenario in [
                  'full',
                  'incr-full',
                  'incr-unchanged',
                  'incr-patched',
                ]"
              >
                <div class="group-header">{{ scenario }}</div>
                <SummaryTable
                  :summary="calculateSummary('scenario', scenario)"
                  :withLegend="false"
                ></SummaryTable>
              </div>
            </div>
          </div>
          <div class="aggregation-section">
            <div class="header">Category</div>
            <div class="groups">
              <div class="group" v-for="category in ['primary', 'secondary']">
                <div class="group-header">{{ category }}</div>
                <SummaryTable
                  :summary="calculateSummary('category', category)"
                  :withLegend="false"
                ></SummaryTable>
              </div>
            </div>
          </div>
        </div>
      </template>
    </Toggle>
  </fieldset>
</template>

<style scoped lang="scss">
.aggregation-section > .header {
  margin-top: 5px;
  font-size: 16px;
  text-align: center;
}

.aggregation-section > .groups {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}

.aggregation-section .group {
  min-width: 45%;
  border: 1px dotted black;
  padding: 5px;
  margin: 10px;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.aggregation-section .group-header {
  margin-bottom: 5px;
  font-weight: bold;
}
</style>
