<script setup lang="ts">
import {CompileTestCase} from "../common";
import {computed} from "vue";
import {ArtifactDescription} from "../../types";
import {getPastDate, formatDate} from "./utils";
import ProfileShortcut from "./shortcuts/profile-shortcut.vue";
import BenchmarkDetailGraph from "./benchmark-detail-graph.vue";

const props = defineProps<{
  testCase: CompileTestCase;
  metric: string;
  artifact: ArtifactDescription;
  baseArtifact: ArtifactDescription;
}>();

const historyGraphTarget = computed(() => {
  // Redirect if in iframe, o/w open in new tab
  return window.parent === window ? "_blank" : "_self";
});

function benchmarkLink(benchmark: string): string {
  return `https://github.com/JuliaCI/BaseBenchmarks.jl/tree/master/src/${
    benchmark.split(".")[0]
  }`;
}

function graphLink(
  commit: ArtifactDescription,
  metric: string,
  testCase: CompileTestCase
): string {
  // Move to `$2*DAY_RANGE days ago` to display history of the test case
  const start = formatDate(getPastDate(new Date(commit.date), 2 * 30));
  const requested_end = new URL(window.location.toString()).searchParams.get(
    "end"
  );
  let end;
  if (!requested_end) {
    end = "";
  } else {
    end = commit.commit;
  }
  const {benchmark} = testCase;
  return `/graphs.html?start=${start}&end=${end}&benchmark=${benchmark}&stat=${metric}`;
}
</script>

<template>
  <div>
    <div class="columns">
      <div class="rows grow">
        <div class="title bold">Benchmark info</div>
        <div class="benchmark-container">
          <div class="benchmark-label">Benchmark</div>
          <div class="benchmark-value">{{ testCase.benchmark }}</div>
        </div>
      </div>
      <div class="rows grow links">
        <div class="title bold">Links</div>
        <ul>
          <li>
            <a
              :href="graphLink(props.artifact, props.metric, props.testCase)"
              :target="historyGraphTarget"
            >
              History graph
            </a>
          </li>
          <li>
            <a :href="benchmarkLink(testCase.benchmark)" target="_blank">
              Benchmark source code
            </a>
          </li>
        </ul>
      </div>
    </div>
    <BenchmarkDetailGraph
      :test-case="testCase"
      :metric="metric"
      :artifact="artifact"
      :base-artifact="baseArtifact"
    />
    <div class="shortcut">
      <ProfileShortcut :test-case="props.testCase" />
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../benchmark-detail.scss";

.shortcut {
  margin-top: 15px;
  text-align: left;
}

.benchmark-container {
  display: flex;
  align-items: center;
}

.benchmark-label {
  flex: 0;
  text-align: left;
  font-weight: bold;
  padding-right: 10px;
}

.benchmark-value {
  flex: 1;
  text-align: center;
  overflow-wrap: anywhere;
}

.links {
  li {
    text-align: left;
  }
}
</style>

<style>
.u-tooltip {
  font-size: 10pt;
  position: absolute;
  background: #fff;
  display: none;
  border: 2px solid black;
  padding: 4px;
  pointer-events: none;
  z-index: 100;
  white-space: pre;
  font-family: monospace;
}
</style>
