<script lang="ts"></script>

<script setup lang="ts">
import {Ref, ref} from "vue";
import {CompareResponse, Tab} from "./types";
import {diffClass, percentClass} from "./shared";
import {SummaryGroup} from "./data";
import SummaryPercentValue from "./summary/percent-value.vue";
import SummaryRange from "./summary/range.vue";

const props = withDefaults(
  defineProps<{
    data: CompareResponse;
    compileTimeSummary: SummaryGroup;
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

const bootstrapA = props.data.a.bootstrap_total;
const bootstrapB = props.data.b.bootstrap_total;
const bootstrapValid = bootstrapA > 0.0 && bootstrapB > 0.0;
const compileTimeValid = props.compileTimeSummary.all.count > 0;

const activeTab: Ref<Tab> = ref(props.initialTab);
</script>

<template>
  <div class="wrapper">
    <div
      class="tab"
      title="Compilation time benchmarks: measure how long does it take to compile various crates using the compared rustc."
      :class="{selected: activeTab === Tab.CompileTime}"
      @click="changeTab(Tab.CompileTime)"
    >
      <div class="title">Compile-time</div>
      <div class="summary compile-time">
        <template v-if="compileTimeValid">
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
                  <SummaryRange :range="compileTimeSummary.all.range" />
                </td>
                <td>
                  <SummaryPercentValue
                    :class="percentClass(compileTimeSummary.all.average)"
                    :value="compileTimeSummary.all.average"
                  />
                </td>
              </tr>
            </thead>
          </table>
        </template>
        <template v-else>No relevant results</template>
      </div>
    </div>
    <div
      class="tab"
      title="Runtime benchmarks: measure how long does it take to execute (i.e. how fast are) programs compiled by the compared rustc."
      :class="{selected: activeTab === Tab.Runtime}"
      @click="changeTab(Tab.Runtime)"
    >
      <div class="title">Runtime</div>
      <div class="summary runtime"></div>
    </div>
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
  width: 200px;
  height: 60px;
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

.compile-time {
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
