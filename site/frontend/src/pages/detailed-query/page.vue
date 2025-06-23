<script setup lang="ts">
import {ref, onMounted, Ref, computed} from "vue";
import {
  createUrlWithAppendedParams,
  getUrlParams,
  navigateToUrlParams,
} from "../../utils/navigation";
import {postMsgpack} from "../../utils/requests";
import {SELF_PROFILE_DATA_URL} from "../../urls";
import {openTraceInPerfetto} from "../../perfetto";
import {
  Selector,
  SelfProfileResponse,
  createTitleData,
  createDownloadLinksData,
  createTableData,
  createArtifactData,
} from "./utils";

const loading = ref(true);
const data: Ref<SelfProfileResponse | null> = ref(null);
const selector: Ref<Selector | null> = ref(null);
const showIncr = ref(true);
const showDelta = ref(true);

// Computed properties for UI data
const titleData = computed(() => createTitleData(selector.value));
const downloadLinksData = computed(() =>
  createDownloadLinksData(selector.value)
);
const tableData = computed(() => createTableData(data.value));
const artifactData = computed(() => createArtifactData(data.value));

function handlePerfettoClick(event: Event, link: string, title: string) {
  event.preventDefault();
  openTraceInPerfetto(link, title);
}

async function loadData() {
  const params = getUrlParams();
  const {commit, base_commit, benchmark, scenario, sort_idx} = params;
  const currentSelector: Selector = {
    commit,
    base_commit: base_commit ?? null,
    benchmark,
    scenario,
    sort_idx: sort_idx ?? "-2",
  };
  selector.value = currentSelector;

  const response = await postMsgpack<SelfProfileResponse>(
    SELF_PROFILE_DATA_URL,
    currentSelector
  );
  data.value = response;
  populateUIData(response, currentSelector);
  loading.value = false;
}

function populateUIData(responseData: SelfProfileResponse, state: Selector) {
  if (!responseData.base_profile_delta) {
    showDelta.value = false;
  }

  if (!state.scenario.includes("incr-")) {
    showIncr.value = false;
  }
}

function sortTable(columnIndex: number, defaultDirection: number) {
  if (!selector.value) return;

  let newSortIdx: number;
  if (Math.abs(Number(selector.value.sort_idx)) === columnIndex) {
    newSortIdx = -Number(selector.value.sort_idx);
  } else {
    newSortIdx = defaultDirection * columnIndex;
  }

  const newSelector = {
    ...selector.value,
    sort_idx: newSortIdx,
  };

  navigateToUrlParams(createUrlWithAppendedParams(newSelector).searchParams);
}

function getSortAttributes(columnIndex: number) {
  if (!selector.value) return {};

  const currentSortIdx = Number(selector.value.sort_idx);
  if (Math.abs(currentSortIdx) === columnIndex) {
    return {
      "data-sorted-by": currentSortIdx > 0 ? "desc" : "asc",
    };
  }
  return {};
}

onMounted(async () => {
  await loadData();
});
</script>

<template>
  <div>
    <div v-if="loading">
      <p>Loading...</p>
    </div>
    <div v-else id="content">
      <h3 id="title">
        {{ titleData.text }}
        <template v-if="selector?.base_commit">
          <br />diff vs base {{ selector.base_commit.substring(0, 10) }},
          <a :href="titleData.baseHref">query info for just base commit</a>
          <br />
          <a :href="titleData.selfHref">query info for just this commit</a>
        </template>
      </h3>

      <div id="raw-urls">
        <template v-if="downloadLinksData.baseLinks">
          Download/view
          <a :href="downloadLinksData.baseLinks.raw">raw</a>,
          <a :href="downloadLinksData.baseLinks.flamegraph">flamegraph</a>,
          <a :href="downloadLinksData.baseLinks.crox">crox</a>,
          <a :href="downloadLinksData.baseLinks.codegen">codegen-schedule</a>
          (<a
            href="#"
            @click="
              handlePerfettoClick(
                $event,
                downloadLinksData.baseLinks.perfetto.link,
                downloadLinksData.baseLinks.perfetto.traceTitle
              )
            "
            >Perfetto</a
          >,
          <a :href="downloadLinksData.baseLinks.firefox">Firefox profiler</a>)
          results for {{ selector?.base_commit?.substring(0, 10) }} (base
          commit)
          <br />
        </template>

        Download/view
        <a :href="downloadLinksData.newLinks.raw">raw</a>,
        <a :href="downloadLinksData.newLinks.flamegraph">flamegraph</a>,
        <a :href="downloadLinksData.newLinks.crox">crox</a>,
        <a :href="downloadLinksData.newLinks.codegen">codegen-schedule</a>
        (<a
          href="#"
          @click="
            handlePerfettoClick(
              $event,
              downloadLinksData.newLinks.perfetto.link,
              downloadLinksData.newLinks.perfetto.traceTitle
            )
          "
          >Perfetto</a
        >, <a :href="downloadLinksData.newLinks.firefox">Firefox profiler</a>)
        results for {{ selector?.commit?.substring(0, 10) }} (new commit)

        <template v-if="downloadLinksData.diffLink">
          <br />
          Diff: <a :href="downloadLinksData.diffLink">codegen-schedule</a>
        </template>

        <template v-if="downloadLinksData.localCommands.base">
          <br />
          Local profile (base):
          <code>{{ downloadLinksData.localCommands.base }}</code>
        </template>

        <br />
        Local profile (new):
        <code>{{ downloadLinksData.localCommands.new }}</code>

        <template v-if="downloadLinksData.localCommands.diff">
          <br />
          Local profile (diff):
          <code>{{ downloadLinksData.localCommands.diff }}</code>
        </template>
      </div>

      <h4>Artifact Size</h4>
      <table id="artifact-table">
        <thead>
          <tr>
            <th>Artifact</th>
            <th>Size</th>
            <th>Size delta</th>
          </tr>
        </thead>
        <tbody id="artifact-body">
          <tr v-for="artifact in artifactData" :key="artifact.name">
            <td style="text-align: center">{{ artifact.name }}</td>
            <td>{{ artifact.size }}</td>
            <td>{{ artifact.sizeDelta }}</td>
          </tr>
        </tbody>
      </table>

      <p>
        'Time (%)' is the percentage of the cpu-clock time spent on this query
        (we do not use wall-time as we want to account for parallelism).
      </p>
      <p>Executions do not include cached executions.</p>

      <table :class="{'hide-incr': !showIncr, 'hide-delta': !showDelta}">
        <thead>
          <tr id="table-header">
            <th
              v-bind="getSortAttributes(1)"
              data-sort-idx="1"
              data-default-sort-dir="1"
            >
              <a href="#" @click.prevent="sortTable(1, 1)">Query/Function</a>
            </th>
            <th
              v-bind="getSortAttributes(10)"
              data-sort-idx="10"
              data-default-sort-dir="-1"
            >
              <a href="#" @click.prevent="sortTable(10, -1)">Time (%)</a>
            </th>
            <th
              v-bind="getSortAttributes(2)"
              data-sort-idx="2"
              data-default-sort-dir="-1"
            >
              <a href="#" @click.prevent="sortTable(2, -1)">Time (s)</a>
            </th>
            <th
              v-bind="getSortAttributes(11)"
              class="delta"
              data-sort-idx="11"
              data-default-sort-dir="-1"
            >
              <a href="#" @click.prevent="sortTable(11, -1)">Time delta</a>
            </th>
            <th
              v-bind="getSortAttributes(5)"
              data-sort-idx="5"
              data-default-sort-dir="-1"
            >
              <a href="#" @click.prevent="sortTable(5, -1)">Executions</a>
            </th>
            <th
              v-bind="getSortAttributes(12)"
              class="delta"
              data-sort-idx="12"
              data-default-sort-dir="-1"
            >
              <a href="#" @click.prevent="sortTable(12, -1)"
                >Executions delta</a
              >
            </th>
            <th
              v-bind="getSortAttributes(7)"
              class="incr"
              data-sort-idx="7"
              data-default-sort-dir="-1"
              title="Incremental loading time"
            >
              <a href="#" @click.prevent="sortTable(7, -1)"
                >Incremental loading (s)</a
              >
            </th>
            <th
              v-bind="getSortAttributes(13)"
              class="incr delta"
              data-sort-idx="13"
              data-default-sort-dir="-1"
            >
              <a href="#" @click.prevent="sortTable(13, -1)"
                >Incremental loading delta</a
              >
            </th>
          </tr>
        </thead>
        <tbody id="primary-table">
          <tr
            v-for="(row, index) in tableData"
            :key="index"
            :class="{'total-row': row.isTotal}"
          >
            <td>{{ row.label }}</td>
            <td :title="row.timePercent.title">{{ row.timePercent.value }}</td>
            <td>{{ row.timeSeconds }}</td>
            <td class="delta" v-html="row.timeDelta"></td>
            <td>{{ row.executions }}</td>
            <td class="delta" v-html="row.executionsDelta"></td>
            <td class="incr">{{ row.incrementalLoading }}</td>
            <td class="incr delta" v-html="row.incrementalLoadingDelta"></td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style lang="scss" scoped>
table {
  border-collapse: collapse;
}

thead th {
  padding-left: 1em;
}

:global(body.hide-incr) .incr {
  display: none;
}

:global(body.hide-delta) .delta {
  display: none;
}

.hide-incr .incr {
  display: none;
}

.hide-delta .delta {
  display: none;
}

.positive {
  color: red;
  font-weight: bold;
}

.negative {
  color: green;
  font-weight: bold;
}

.neutral {
  color: #666;
}

.total-row {
  font-weight: bold;
  background-color: #eee !important;
  border-top: 1px solid black;
  border-bottom: 1px solid black;
}

#primary-table td,
#primary-table th {
  padding-left: 1.5em;
  white-space: pre;
}

#primary-table tr:nth-child(2n + 1) {
  background-color: #f9f9f9;
}

#primary-table tr:nth-child(1) {
  background-color: #eee;
  font-weight: bold;
  border-top: 1px solid black;
  border-bottom: 1px solid black;
}

[data-sort-idx]::after {
  content: "⇕";
}

[data-sorted-by="desc"]::after {
  content: "▲";
}

[data-sorted-by="asc"]::after {
  content: "▼";
}

code {
  background-color: #eee;
  border-radius: 3px;
  user-select: all;
}

#artifact-table th {
  text-align: center;
}

#artifact-table td {
  padding: 0 0 0 20px;
}

button {
  margin-right: 10px;
  padding: 5px 10px;
  background-color: #f0f0f0;
  border: 1px solid #ccc;
  border-radius: 3px;
  cursor: pointer;

  &:hover {
    background-color: #e0e0e0;
  }
}
</style>
