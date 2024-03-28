<script setup lang="tsx">
import {h, ref, Ref} from "vue";
import {CompareResponse, Tab} from "./types";
import {
  diffClass,
  formatPercentChange,
  formatSize,
  percentClass,
} from "./shared";
import {SummaryGroup} from "./data";
import SummaryPercentValue from "./summary/percent-value.vue";
import SummaryRange from "./summary/range.vue";
import TabComponent from "../../components/tab.vue";

const props = withDefaults(
  defineProps<{
    data: CompareResponse;
    compileTimeSummary: SummaryGroup;
    runtimeSummary: SummaryGroup;
    initialTab?: Tab;
  }>(),
  {
    initialTab: Tab.CompileTime,
  }
);
const emit = defineEmits<{
  (e: "changeTab", tab: Tab): void;
}>();

function changeTab(tab: Tab) {
  activeTab.value = tab;
  emit("changeTab", tab);
}

function formatBootstrap(value: number): string {
  if (value > 0.0) {
    return (value / 10e8).toFixed(3);
  }
  return "???";
}

function SummaryTable({summary}: {summary: SummaryGroup}) {
  const valid = summary.all.count > 0;
  if (valid) {
    return (
      <table>
        <thead>
          <tr>
            <th>Range</th>
            <th>Mean</th>
          </tr>
        </thead>
        <thead>
          <tr>
            <td>
              <SummaryRange range={summary.all.range} />
            </td>
            <td>
              <SummaryPercentValue
                class={percentClass(summary.all.average)}
                value={summary.all.average}
              />
            </td>
          </tr>
        </thead>
      </table>
    );
  }
  return <div>No results</div>;
}

function formatArtifactSize(size: number): string {
  if (size === 0) {
    return "???";
  }
  return formatSize(size);
}

const bootstrapA = props.data.a.bootstrap_total;
const bootstrapB = props.data.b.bootstrap_total;
const bootstrapValid = bootstrapA > 0.0 && bootstrapB > 0.0;

const totalSizeA = Object.values(props.data.a.component_sizes).reduce(
  (a, b) => a + b,
  0
);
const totalSizeB = Object.values(props.data.b.component_sizes).reduce(
  (a, b) => a + b,
  0
);
const sizesAvailable = totalSizeA > 0 || totalSizeB > 0;
const bothSizesAvailable = totalSizeA > 0 && totalSizeB > 0;

const activeTab: Ref<Tab> = ref(props.initialTab);
</script>

<template>
  <div class="wrapper">
    <TabComponent
      :extra-info="'Compilation time benchmarks: measure how long does it take to compile various crates using the compared rustc.'"
      :title="'Compile-time'"
      :selected="activeTab === Tab.CompileTime"
      @click="changeTab(Tab.CompileTime)"
    >
      <template v-slot:summary>
        <SummaryTable :summary="compileTimeSummary" />
      </template>
    </TabComponent>
    <TabComponent
      :extra-info="'Runtime benchmarks: measure how long does it take to execute (i.e. how fast are) programs compiled by the compared rustc.'"
      :title="'Runtime'"
      :selected="activeTab === Tab.Runtime"
      @click="changeTab(Tab.Runtime)"
    >
      <template v-slot:summary>
        <SummaryTable :summary="runtimeSummary" />
      </template>
    </TabComponent>
    <div
      class="tab"
      title="Bootstrap duration: measures how long does it take to compile rustc by itself."
      :class="{selected: activeTab === Tab.Bootstrap}"
      @click="changeTab(Tab.Bootstrap)"
    >
      <div class="title">Bootstrap</div>
      <div class="summary">
        <div>
          {{ formatBootstrap(bootstrapA) }} ->
          {{ formatBootstrap(bootstrapB) }}
        </div>
        <div
          v-if="bootstrapValid"
          :class="{[diffClass(bootstrapB - bootstrapA)]: bootstrapValid}"
        >
          {{ ((bootstrapB - bootstrapA) / 10e8).toFixed(1) }}s ({{
            (((bootstrapB - bootstrapA) / bootstrapA) * 100).toFixed(3)
          }}%)
        </div>
      </div>
    </div>
    <div
      class="tab"
      title="Artifact size: sizes of individual components of the two artifacts."
      v-if="sizesAvailable"
      :class="{selected: activeTab === Tab.ArtifactSize}"
      @click="changeTab(Tab.ArtifactSize)"
    >
      <div class="title">Artifact size</div>
      <div class="summary">
        <div>
          {{ formatArtifactSize(totalSizeA) }} ->
          {{ formatArtifactSize(totalSizeB) }}
        </div>
        <div
          v-if="bothSizesAvailable"
          :class="diffClass(totalSizeB - totalSizeA)"
        >
          {{ totalSizeB < totalSizeA ? "-" : ""
          }}{{ formatSize(Math.abs(totalSizeB - totalSizeA)) }} ({{
            formatPercentChange(totalSizeA, totalSizeB)
          }})
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px 0;

  @media (min-width: 600px) {
    justify-content: center;
    flex-direction: row;
    align-items: normal;
  }
}

.tab {
  display: flex;
  flex-direction: column;
  position: relative;
  min-width: 200px;
  min-height: 60px;
  padding: 5px;
  text-align: center;
  border: 2px dotted #cccccc;
  border-radius: 5px;
  cursor: pointer;

  &:hover {
    @extend .selected;
  }

  @media (min-width: 600px) {
    &:not(:first-child) {
      margin-left: 30px;
    }

    &:not(:last-child) {
      :after {
        content: "";
        position: absolute;
        right: -17px;
        border-right: 1px solid black;
        top: 20%;
        bottom: 20%;
      }
    }
  }
}

.title {
  font-weight: bold;
  font-size: 1.1em;
  margin-bottom: 5px;
}

.selected {
  border-style: solid;
  border-color: black;
}

.table-wrapper {
  table {
    width: 100%;
    table-layout: auto;
  }

  th {
    font-weight: normal;
  }
}

.summary {
  display: flex;
  flex-direction: column;
  justify-content: center;
  font-size: 0.9em;
  flex-grow: 1;
}
</style>
