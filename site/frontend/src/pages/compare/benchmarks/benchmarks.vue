<script setup lang="tsx">
import {computed, h} from "vue";
import TestCasesTable from "./test-cases-table.vue";
import {TestCase} from "../data";
import {CompareResponse, DataFilter} from "../types";

export interface BenchmarkProps {
  data: CompareResponse;
  testCases: TestCase[];
  allTestCases: TestCase[];
  filter: DataFilter;
  stat: string;
}

const props = defineProps<BenchmarkProps>();

function Section({title, link, linkUp}: {title: string, link: string, linkUp: boolean}) {
  return (
    <div class="category-title">
      {title} benchmarks
      <span title={`To ${link} benchmarks`}>&nbsp;
        <a href={`#${link}-benchmarks`}>{linkUp ? "⮭" : "⮯"}</a>
      </span>
    </div>
  );
}

const primaryCases = computed(() => props.testCases.filter(c => c.category === "primary"));
const secondaryCases = computed(() => props.testCases.filter(c => c.category === "secondary"));
const primaryHasNonRelevant = computed(() => props.allTestCases.filter(c => c.category === "primary").length > 0);
const secondaryHasNonRelevant = computed(() => props.allTestCases.filter(c => c.category === "secondary").length > 0);
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
    <TestCasesTable id="primary-benchmarks"
                    :cases="primaryCases"
                    :has-non-relevant="primaryHasNonRelevant"
                    :show-raw-data="filter.showRawData"
                    :commit-a="data.a"
                    :commit-b="data.b"
                    :stat="stat"
    >
      <template #header>
        <Section title="Primary" link="secondary" :linkUp="false"></Section>
      </template>
    </TestCasesTable>
    <hr />
    <TestCasesTable id="secondary-benchmarks"
                    :cases="secondaryCases"
                    :has-non-relevant="secondaryHasNonRelevant"
                    :show-raw-data="filter.showRawData"
                    :commit-a="data.a"
                    :commit-b="data.b"
                    :stat="stat"
    >
      <template #header>
        <Section title="Secondary" link="primary" :linkUp="true"></Section>
      </template>
    </TestCasesTable>
    <!--      :cases="primaryCases"-->
    <!--      :has-non-relevant="testCasesWithNonRelevant.filter(c => c.category === 'primary').length > 0"-->
    <!--      :show-raw-data="filter.showRawData"-->
    <!--      :commit-a="data.a"-->
    <!--      :commit-b="data.b"-->
    <!--      :stat="stat"-->
    <!--      :before="before"-->
    <!--      :after="after"-->
    <!--      id="primary-benchmarks"-->
    <!--    <hr />-->
    <!--    <test-cases-table-->
    <!--      :cases="secondaryCases"-->
    <!--      :has-non-relevant="testCasesWithNonRelevant.filter(c => c.category === 'secondary').length > 0"-->
    <!--      :show-raw-data="filter.showRawData"-->
    <!--      :commit-a="data.a"-->
    <!--      :commit-b="data.b"-->
    <!--      :stat="stat"-->
    <!--      :before="before"-->
    <!--      :after="after"-->
    <!--      id="secondary-benchmarks"-->
    <!--    ></test-cases-table>-->
    <br />
    <hr />
  </div>
</template>

<style scoped>

</style>