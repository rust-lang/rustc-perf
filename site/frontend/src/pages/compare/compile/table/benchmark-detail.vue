<script setup lang="ts">
import {
  CargoProfileMetadata,
  CompileBenchmarkMap,
  CompileBenchmarkMetadata,
  CompileTestCase,
} from "../common";
import {computed, onMounted, Ref, ref} from "vue";
import Tooltip from "../../tooltip.vue";
import {ArtifactDescription} from "../../types";
import {getPastDate, formatDate} from "./utils";
import {
  COMPILE_DETAIL_SECTIONS_RESOLVER,
  CompileDetailSections,
  CompileDetailSectionsSelector,
} from "./detail-resolver";
import CompileSectionsChart from "./sections-chart.vue";
import PerfettoLink from "../../../../components/perfetto-link.vue";
import ProfileShortcut from "./shortcuts/profile-shortcut.vue";
import BinarySizeShortcut from "./shortcuts/binary-size-shortcut.vue";
import BenchmarkDetailGraph from "./benchmark-detail-graph.vue";

const props = defineProps<{
  testCase: CompileTestCase;
  metric: string;
  artifact: ArtifactDescription;
  baseArtifact: ArtifactDescription;
  benchmarkMap: CompileBenchmarkMap;
}>();

const BINARY_SIZE_METRIC: string = "size:linked_artifact";

function createSectionsSelector(): CompileDetailSectionsSelector {
  return {
    benchmark: props.testCase.benchmark,
    profile: props.testCase.profile,
    scenario: props.testCase.scenario,
    start: props.baseArtifact.commit,
    end: props.artifact.commit,
  };
}

async function loadSections(): Promise<CompileDetailSections> {
  return await COMPILE_DETAIL_SECTIONS_RESOLVER.load(createSectionsSelector());
}

function benchmarkLink(benchmark: string): string {
  return `https://github.com/rust-lang/rustc-perf/tree/master/collector/compile-benchmarks/${benchmark}`;
}

function detailedQueryLink(
  commit: ArtifactDescription,
  baseCommit?: ArtifactDescription
): string {
  const {benchmark, profile, scenario} = props.testCase;
  let link = `/detailed-query.html?commit=${commit.commit}&benchmark=${benchmark}-${profile}&scenario=${scenario}`;
  if (baseCommit !== undefined) {
    link += `&base_commit=${baseCommit.commit}`;
  }
  return link;
}

function graphLink(
  commit: ArtifactDescription,
  metric: string,
  testCase: CompileTestCase
): string {
  // Move to `30 days ago` to display history of the test case
  const start = formatDate(getPastDate(new Date(commit.date), 30));
  const end = commit.commit;
  const {benchmark, profile, scenario} = testCase;
  return `/index.html?start=${start}&end=${end}&benchmark=${benchmark}&profile=${profile}&scenario=${scenario}&stat=${metric}`;
}

const metadata = computed(
  (): CompileBenchmarkMetadata =>
    props.benchmarkMap[props.testCase.benchmark] ?? null
);
const cargoProfile = computed((): CargoProfileMetadata => {
  if (
    props.testCase.profile === "opt" &&
    metadata?.value.release_profile !== null
  ) {
    return metadata.value.release_profile;
  } else if (
    props.testCase.profile === "debug" &&
    metadata?.value.dev_profile !== null
  ) {
    return metadata?.value.dev_profile;
  }
});

const sectionsDetail: Ref<CompileDetailSections | null> = ref(null);
onMounted(() => {
  loadSections().then((d) => {
    sectionsDetail.value = d;
  });
});
</script>

<template>
  <div>
    <div class="columns">
      <div class="rows grow">
        <div class="title bold">Benchmark info</div>
        <table>
          <tbody>
            <tr>
              <td>Benchmark</td>
              <td>{{ testCase.benchmark }}</td>
            </tr>
            <tr>
              <td>Profile</td>
              <td>{{ testCase.profile }}</td>
            </tr>
            <tr>
              <td>Scenario</td>
              <td>{{ testCase.scenario }}</td>
            </tr>
            <tr>
              <td>Category</td>
              <td>{{ testCase.category }}</td>
            </tr>
            <tr v-if="(metadata?.binary ?? null) !== null">
              <td>Artifact</td>
              <td>{{ metadata.binary ? "binary" : "library" }}</td>
            </tr>
            <tr v-if="(metadata?.iterations ?? null) !== null">
              <td>
                Iterations
                <Tooltip> How many times is the benchmark executed?</Tooltip>
              </td>
              <td>{{ metadata.iterations }}</td>
            </tr>
            <tr v-if="(cargoProfile?.lto ?? null) !== null">
              <td>LTO</td>
              <td>{{ cargoProfile.lto }}</td>
            </tr>
            <tr v-if="(cargoProfile?.debug ?? null) !== null">
              <td>Debuginfo</td>
              <td>{{ cargoProfile.debug }}</td>
            </tr>
            <tr v-if="(cargoProfile?.codegen_units ?? null) !== null">
              <td>Codegen units</td>
              <td>{{ cargoProfile.codegen_units }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div class="rows grow links">
        <div class="title bold">Links</div>
        <ul>
          <li>
            <a
              :href="detailedQueryLink(props.artifact, props.baseArtifact)"
              target="_blank"
            >
              Detailed results
            </a>
          </li>
          <li>
            Before:
            <a :href="detailedQueryLink(props.baseArtifact)" target="_blank">
              self-profile</a
            >,
            <PerfettoLink
              :artifact="props.baseArtifact"
              :test-case="props.testCase"
              >query trace
            </PerfettoLink>
          </li>
          <li>
            After:
            <a :href="detailedQueryLink(props.artifact)" target="_blank"
              >self-profile</a
            >,
            <PerfettoLink :artifact="props.artifact" :test-case="props.testCase"
              >query trace
            </PerfettoLink>
          </li>
          <li>
            <a
              :href="graphLink(props.artifact, props.metric, props.testCase)"
              target="_blank"
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
    <div class="columns" v-if="props.metric !== BINARY_SIZE_METRIC">
      <div class="rows center-items grow">
        <div class="title bold">
          Sections
          <Tooltip
            >Percentual duration of individual compilation sections. This is a
            rough estimate that might not necessarily contain all of the
            individual parts of the compilation. The sections are calculated
            based on the results of self-profile queries and they are measured
            based on wall-time.
          </Tooltip>
        </div>
        <div style="font-size: 0.8em">
          Note that the data for this chart is calculated from wall-time (it is
          metric agnostic)!
        </div>
        <div>
          <CompileSectionsChart
            v-if="
              (sectionsDetail?.before ?? null) !== null &&
              (sectionsDetail?.after ?? null) !== null
            "
            :before="sectionsDetail.before"
            :after="sectionsDetail.after"
          />
          <span v-else-if="sectionsDetail === null">Loading…</span>
          <span v-else>Not available</span>
        </div>
      </div>
    </div>
    <div class="shortcut">
      <template v-if="props.metric === BINARY_SIZE_METRIC">
        <BinarySizeShortcut
          v-if="testCase.profile === 'debug' || testCase.profile === 'opt'"
          :artifact="props.artifact"
          :base-artifact="props.baseArtifact"
          :test-case="props.testCase"
        />
      </template>
      <ProfileShortcut
        v-else
        :artifact="props.artifact"
        :base-artifact="props.baseArtifact"
        :test-case="props.testCase"
      />
    </div>
  </div>
</template>

<style scoped lang="scss">
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

.graphs {
  margin-top: 15px;
}

.rows {
  display: flex;
  flex-direction: column;
  gap: 10px;

  &.center-items {
    align-items: center;
  }
}

.shortcut {
  margin-top: 15px;
  text-align: left;
}

.title {
  &.bold,
  .bold {
    font-weight: bold;
  }

  &.info {
    margin-bottom: 15px;
  }
}

table {
  align-self: flex-start;
  margin-right: 20px;

  td {
    text-align: left;

    &:first-child {
      font-weight: bold;
      padding-right: 10px;
    }
  }
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
