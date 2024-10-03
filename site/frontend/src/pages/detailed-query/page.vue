<script setup lang="tsx">
import {h, ref, Ref, computed} from "vue";
import {getUrlParams, changeUrl} from "../../utils/navigation";
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
  DeltaData,
} from "./utils";

const loading = ref(true);
const data: Ref<SelfProfileResponse | null> = ref(null);
const selector: Ref<Selector | null> = ref(null);
const showIncr = ref(true);
const showDelta = ref(true);

type SortDirection = "asc" | "desc";

// Client-side sorting state
const currentSortColumn = ref<string>("timeSeconds");
const currentSortDirection = ref<SortDirection>("desc");

// Computed properties for UI data
const titleData = computed(() => createTitleData(selector.value));
const downloadLinksData = computed(() =>
  createDownloadLinksData(selector.value)
);
const unsortedTableData = computed(() => createTableData(data.value));
const tableData = computed(() => {
  const rows = unsortedTableData.value;
  if (rows.length === 0) return rows;

  // Separate totals row from data rows
  const totalsRow = rows.find((row) => row.isTotal);
  const dataRows = rows.filter((row) => !row.isTotal);

  // Sort data rows based on current sort column and direction
  const sortedDataRows = dataRows.sort((a, b) => {
    let aValue: string | number;
    let bValue: string | number;
    let aSecondary: number | undefined;
    let bSecondary: number | undefined;

    // Map column name to data property
    switch (currentSortColumn.value) {
      case "label": // Query/Function
        aValue = a.label;
        bValue = b.label;
        break;
      case "timeSeconds": // Time (s)
        aValue = a.timeSeconds;
        bValue = b.timeSeconds;
        // Use percentage change as secondary sort for equal absolute values
        aSecondary =
          a.timeDelta !== null ? Math.abs(a.timeDelta.percentage) : 0;
        bSecondary =
          b.timeDelta !== null ? Math.abs(b.timeDelta.percentage) : 0;
        break;
      case "executions": // Executions
        aValue = a.executions;
        bValue = b.executions;
        // Use percentage change as secondary sort for equal absolute values
        aSecondary =
          a.executionsDelta !== null
            ? Math.abs(a.executionsDelta.percentage)
            : 0;
        bSecondary =
          b.executionsDelta !== null
            ? Math.abs(b.executionsDelta.percentage)
            : 0;
        break;
      case "incrementalLoading": // Incremental loading (s)
        aValue = a.incrementalLoading;
        bValue = b.incrementalLoading;
        // Use percentage change as secondary sort for equal absolute values
        aSecondary =
          a.incrementalLoadingDelta !== null
            ? Math.abs(a.incrementalLoadingDelta.percentage)
            : 0;
        bSecondary =
          b.incrementalLoadingDelta !== null
            ? Math.abs(b.incrementalLoadingDelta.percentage)
            : 0;
        break;
      case "timePercent": // Time (%)
        aValue = a.timePercent.value;
        bValue = b.timePercent.value;
        break;
      case "timeDelta": // Time delta
        aValue = a.timeDelta !== null ? a.timeDelta.delta : -Infinity;
        bValue = b.timeDelta !== null ? b.timeDelta.delta : -Infinity;
        // Use percentage as secondary sort for equal delta values
        aSecondary =
          a.timeDelta !== null ? Math.abs(a.timeDelta.percentage) : 0;
        bSecondary =
          b.timeDelta !== null ? Math.abs(b.timeDelta.percentage) : 0;
        break;
      case "executionsDelta": // Executions delta
        aValue =
          a.executionsDelta !== null ? a.executionsDelta.delta : -Infinity;
        bValue =
          b.executionsDelta !== null ? b.executionsDelta.delta : -Infinity;
        // Use percentage as secondary sort for equal delta values
        aSecondary =
          a.executionsDelta !== null
            ? Math.abs(a.executionsDelta.percentage)
            : 0;
        bSecondary =
          b.executionsDelta !== null
            ? Math.abs(b.executionsDelta.percentage)
            : 0;
        break;
      case "incrementalLoadingDelta": // Incremental loading delta
        aValue =
          a.incrementalLoadingDelta !== null
            ? a.incrementalLoadingDelta.delta
            : -Infinity;
        bValue =
          b.incrementalLoadingDelta !== null
            ? b.incrementalLoadingDelta.delta
            : -Infinity;
        // Use percentage as secondary sort for equal delta values
        aSecondary =
          a.incrementalLoadingDelta !== null
            ? Math.abs(a.incrementalLoadingDelta.percentage)
            : 0;
        bSecondary =
          b.incrementalLoadingDelta !== null
            ? Math.abs(b.incrementalLoadingDelta.percentage)
            : 0;
        break;
      default:
        aValue = a.label;
        bValue = b.label;
    }

    // Handle string vs number comparison
    let comparison: number;
    if (typeof aValue === "string" && typeof bValue === "string") {
      comparison = aValue.localeCompare(bValue);
    } else {
      comparison = (aValue as number) - (bValue as number);

      // If primary values are equal and we have secondary sort criteria, use percentage change
      if (
        comparison === 0 &&
        aSecondary !== undefined &&
        bSecondary !== undefined
      ) {
        comparison = bSecondary - aSecondary; // Higher percentage change comes first
      }
    }

    return currentSortDirection.value === "asc" ? comparison : -comparison;
  });

  // Return totals row first, then sorted data rows
  return totalsRow ? [totalsRow, ...sortedDataRows] : sortedDataRows;
});
const artifactData = computed(() => createArtifactData(data.value));

function handlePerfettoClick(link: string, title: string) {
  openTraceInPerfetto(link, title);
}

function loadSortFromUrl(urlParams: Dict<string>) {
  const sort = urlParams["sort"] ?? "-timeSeconds"; // Default to descending timeSeconds
  // Handle sort format: either "columnName" for asc or "-columnName" for desc
  if (sort.startsWith("-")) {
    currentSortColumn.value = sort.substring(1);
    currentSortDirection.value = "desc";
  } else {
    currentSortColumn.value = sort;
    currentSortDirection.value = "asc";
  }
}

function storeSortToUrl() {
  const params = getUrlParams();
  const sortValue =
    currentSortDirection.value === "desc"
      ? `-${currentSortColumn.value}`
      : currentSortColumn.value;
  params["sort"] = sortValue;
  changeUrl(params);
}

async function loadData() {
  const params = getUrlParams();
  const {commit, base_commit, benchmark, scenario} = params;

  // Load sort state from URL
  loadSortFromUrl(params);

  const currentSelector: Selector = {
    commit,
    base_commit: base_commit ?? null,
    benchmark,
    scenario,
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
  showDelta.value =
    responseData.base_profile_delta !== undefined &&
    responseData.base_profile_delta !== null;
  showIncr.value = state.scenario.includes("incr-");
}

function changeSortParameters(
  columnName: string,
  defaultDirection: SortDirection
) {
  // Toggle direction if clicking the same column, otherwise use default direction
  if (currentSortColumn.value === columnName) {
    currentSortDirection.value =
      currentSortDirection.value === "asc" ? "desc" : "asc";
  } else {
    currentSortColumn.value = columnName;
    currentSortDirection.value = defaultDirection;
  }

  // Update URL with new sort state
  storeSortToUrl();
}

function getHeaderClass(columnName: string): string {
  if (columnName === currentSortColumn.value) {
    if (currentSortDirection.value === "asc") {
      return "header-sort-asc";
    } else {
      return "header-sort-desc";
    }
  }
  return "header-sort";
}

function DeltaComponent({delta}: {delta: DeltaData | null}) {
  if (delta === null) {
    return <span>-</span>;
  }

  let {from, percentage, isIntegral} = delta;
  const to = from + delta.delta;

  let classes: string;
  if (percentage > 1) {
    classes = "positive";
  } else if (percentage < -1) {
    classes = "negative";
  } else {
    classes = "neutral";
  }
  if (Math.abs(delta.delta) <= 0.05) {
    classes = "neutral";
  }
  let text: string;
  if (isIntegral) {
    text = delta.delta.toString();
  } else {
    text = delta.delta.toFixed(3);
  }
  if (percentage != Infinity && percentage != -Infinity) {
    text += `(${percentage.toFixed(1)}%)`.padStart(10, " ");
  } else {
    text += `-`.padStart(10, " ");
  }

  const title = `${from.toFixed(3)} to ${to.toFixed(3)} ≈ ${delta.delta.toFixed(
    3
  )}`;
  return (
    <span class={classes} title={title}>
      {text}
    </span>
  );
}

loadData();
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
            @click.prevent="
              handlePerfettoClick(
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
          @click.prevent="
            handlePerfettoClick(
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
        'Instructions (%)' is the percentage of instructions executed on this
        query (we do not use wall-time as we want to account for parallelism).
      </p>
      <p>
        <b
          >Note: self-profile measurements have been
          <a href="https://github.com/rust-lang/rustc-perf/pull/1984"
            >recently switched</a
          >
          from wall-time to HW counters (instruction count). If comparing with
          an older artifact, the timings might not be directly comparable.</b
        >
      </p>
      <p>Executions do not include cached executions.</p>

      <table>
        <thead>
          <tr id="table-header">
            <th :class="getHeaderClass('label')">
              <a href="#" @click.prevent="changeSortParameters('label', 'asc')"
                >Query/Function</a
              >
            </th>
            <th :class="getHeaderClass('timePercent')">
              <a
                href="#"
                @click.prevent="changeSortParameters('timePercent', 'desc')"
                >Instructions (%)</a
              >
            </th>
            <th :class="getHeaderClass('timeSeconds')">
              <a
                href="#"
                @click.prevent="changeSortParameters('timeSeconds', 'desc')"
                >Instructions</a
              >
            </th>
            <th v-if="showDelta" :class="getHeaderClass('timeDelta')">
              <a
                href="#"
                @click.prevent="changeSortParameters('timeDelta', 'desc')"
                >Instructions delta</a
              >
            </th>
            <th :class="getHeaderClass('executions')">
              <a
                href="#"
                @click.prevent="changeSortParameters('executions', 'desc')"
                >Executions</a
              >
            </th>
            <th v-if="showDelta" :class="getHeaderClass('executionsDelta')">
              <a
                href="#"
                @click.prevent="changeSortParameters('executionsDelta', 'desc')"
                >Executions delta</a
              >
            </th>
            <th
              v-if="showIncr"
              :class="getHeaderClass('incrementalLoading')"
              title="Incremental loading instructions"
            >
              <a
                href="#"
                @click.prevent="
                  changeSortParameters('incrementalLoading', 'desc')
                "
                >Incremental loading (icounts)</a
              >
            </th>
            <th
              v-if="showIncr && showDelta"
              :class="getHeaderClass('incrementalLoadingDelta')"
            >
              <a
                href="#"
                @click.prevent="
                  changeSortParameters('incrementalLoadingDelta', 'desc')
                "
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
            <td :title="row.timePercent.title">
              {{ row.timePercent.formatted }}
            </td>
            <td>{{ row.timeSeconds.toFixed(3) }}</td>
            <td v-if="showDelta">
              <DeltaComponent :delta="row.timeDelta" />
            </td>
            <td>{{ row.executions }}</td>
            <td v-if="showDelta">
              <DeltaComponent :delta="row.executionsDelta" />
            </td>
            <td v-if="showIncr">{{ row.incrementalLoading.toFixed(3) }}</td>
            <td v-if="showDelta && showIncr">
              <DeltaComponent :delta="row.incrementalLoadingDelta" />
            </td>
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

.header-sort::after {
  content: "⇕";
}

.header-sort-desc::after {
  content: "▼";
}

.header-sort-asc::after {
  content: "▲";
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
</style>
