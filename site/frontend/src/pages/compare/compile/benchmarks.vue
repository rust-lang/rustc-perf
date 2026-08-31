<script setup lang="tsx">
import {computed, h} from "vue";
import ComparisonsTable from "./table/comparisons-table.vue";
import {TestCaseComparison} from "../data";
import {CompareResponse} from "../types";
import {
  CompileBenchmarkFilter,
  CompileBenchmarkMap,
  CompileTestCase,
} from "./common";

export interface BenchmarkProps {
  data: CompareResponse;
  testCases: TestCaseComparison<CompileTestCase>[];
  allTestCases: TestCaseComparison<CompileTestCase>[];
  benchmarkMap: CompileBenchmarkMap;
  filter: CompileBenchmarkFilter;
  stat: string;
}

const props = defineProps<BenchmarkProps>();

function Section({
  title,
  link,
  linkUp,
}: {
  title: string;
  link: string;
  linkUp: boolean;
}) {
  return (
    <div class="category-title">
      {title} benchmarks
      <span title={`To ${link} benchmarks`}>
        &nbsp;
        <a href={`#${link}-benchmarks`}>{linkUp ? "⮭" : "⮯"}</a>
      </span>
    </div>
  );
}

const primaryCases = computed(() =>
  props.testCases.filter((c) => c.testCase.category === "primary")
);
const secondaryCases = computed(() =>
  props.testCases.filter((c) => c.testCase.category === "secondary")
);
const primaryHasNonRelevant = computed(
  () =>
    props.allTestCases.filter((c) => c.testCase.category === "primary").length >
    0
);
const secondaryHasNonRelevant = computed(
  () =>
    props.allTestCases.filter((c) => c.testCase.category === "secondary")
      .length > 0
);
</script>

<template>
  <div style="margin-top: 15px">
    <div v-if="data.new_errors.length">
      <p><b>Newly broken benchmarks</b>:</p>
      <details v-for="[crate, error] in data.new_errors">
        <summary>{{ crate }}</summary>
        <pre>{{ error }}</pre>
      </details>
      <hr />
    </div>
    <ComparisonsTable
      id="primary-benchmarks"
      :comparisons="primaryCases"
      :has-non-relevant="primaryHasNonRelevant"
      :show-raw-data="filter.showRawData"
      :commit-a="data.a"
      :commit-b="data.b"
      :stat="stat"
      :benchmark-map="benchmarkMap"
    >
      <template #header>
        <Section title="Primary" link="secondary" :linkUp="false"></Section>
      </template>
    </ComparisonsTable>
    <hr />
    <ComparisonsTable
      id="secondary-benchmarks"
      :comparisons="secondaryCases"
      :has-non-relevant="secondaryHasNonRelevant"
      :show-raw-data="filter.showRawData"
      :commit-a="data.a"
      :commit-b="data.b"
      :stat="stat"
      :benchmark-map="benchmarkMap"
    >
      <template #header>
        <Section title="Secondary" link="primary" :linkUp="true"></Section>
      </template>
    </ComparisonsTable>
    <br />
    <hr />
  </div>
</template>
