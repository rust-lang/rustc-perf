<script setup lang="tsx">
import {h, ref, Ref} from "vue";
import {parseISO, differenceInHours} from "date-fns";
import {formatISODate} from "../../utils/formatting";
import {
  CollectorConfig,
  BenchmarkJobStatus,
  isJobComplete,
  BenchmarkJob,
} from "./data";
import CommitSha from "./commit-sha.vue";

const props = defineProps<{
  collector: CollectorConfig;
}>();

const FILTERS: BenchmarkJobStatus[] = [
  "InProgress",
  "Queued",
  "Success",
  "Failed",
];
const ACTIVE_FILTERS: Ref<Record<BenchmarkJobStatus, boolean>> = ref({
  InProgress: true,
  Queued: true,
  Success: false,
  Failed: true,
});

const showJobs: Ref<boolean> = ref(false);

function filterJobByStatus(status: string) {
  ACTIVE_FILTERS.value[status] = !ACTIVE_FILTERS.value[status];
}

function formatJobStatus(status: BenchmarkJobStatus): string {
  switch (status) {
    case "InProgress":
      return "In progress";
    case "Success":
    case "Failed":
    case "Queued":
      return status;
    default:
      return "Unknown";
  }
}

function ActiveStatus({collector}: {collector: CollectorConfig}) {
  const now = new Date();
  const maxInactivityHours = 1;
  const lastHeartBeatAt = parseISO(collector.lastHeartbeatAt);
  const hourDiff = differenceInHours(now, lastHeartBeatAt);
  let statusText = "Waiting";
  let statusClass = "waiting";

  switch (collector.isActive) {
    case false:
      statusText = "Inactive";
      statusClass = "inactive";
      break;
    case true:
      if (hourDiff >= maxInactivityHours) {
        statusText = "Offline";
        statusClass = "offline";
      } else {
        const allJobsComplete = collector.jobs.every(isJobComplete);
        if (allJobsComplete) {
          statusText = "Waiting";
          statusClass = "waiting";
        } else {
          statusText = "Benchmarking";
          statusClass = "benchmarking";
        }
      }
      break;
  }

  return (
    <span class={`collector-sm-padding-left-right status ${statusClass}`}>
      {statusText}
    </span>
  );
}

function toggleShowJobs() {
  showJobs.value = !showJobs.value;
}

function formatBackend(job: BenchmarkJob): string {
  if (job.kind === "compiletime") {
    return job.backend;
  } else {
    return "";
  }
}
function formatProfile(job: BenchmarkJob): string {
  if (job.kind === "compiletime") {
    return job.profile;
  } else {
    return "";
  }
}
</script>

<template>
  <div class="collector-card">
    <div class="header">
      <div class="collector-name">
        <span>
          <strong class="collector-sm-padding-right">{{
            props.collector.name
          }}</strong>
          <span
            class="collector-sm-padding-left-right collector-left-divider"
            >{{ collector.target }}</span
          >
          <ActiveStatus :collector="collector" />
        </span>
      </div>
    </div>

    <div class="meta">
      <div class="collector-meta">
        <span class="collector-meta-name">
          <strong>Benchmark Set:</strong>
        </span>
        <span> #{{ collector.benchmarkSet }}</span>
      </div>

      <div class="collector-meta">
        <span class="collector-meta-name">
          <strong>Last Heartbeat:</strong>
        </span>
        <span>{{ formatISODate(collector.lastHeartbeatAt) }}</span>
      </div>

      <div class="collector-meta">
        <span class="collector-meta-name">
          <strong>Date Added:</strong>
        </span>
        <span>{{ formatISODate(collector.dateAdded) }}</span>
      </div>
      <button @click="toggleShowJobs" class="show-jobs">
        <template v-if="showJobs">Hide jobs</template>
        <template v-else>Show jobs</template>
      </button>
    </div>

    <div v-if="showJobs" class="table-collector-wrapper">
      <div class="table-collector-status-filter-wrapper">
        <div class="table-collector-status-filters">
          <strong>Filter by job status:</strong>
          <div class="table-collector-status-filter-btn-wrapper">
            <template v-for="filter in FILTERS">
              <button
                class="table-collector-status-filter-btn"
                @click="filterJobByStatus(filter)"
              >
                {{ formatJobStatus(filter) }}
                <input
                  type="checkbox"
                  value="filter"
                  :checked="ACTIVE_FILTERS[filter]"
                />
              </button>
            </template>
          </div>
        </div>
      </div>
      <table class="table-collector" style="border-collapse: collapse">
        <thead>
          <tr class="table-header-row">
            <th>Tag</th>
            <th>Status</th>
            <th>Started at</th>
            <th>Completed at</th>
            <th>Kind</th>
            <th>Backend</th>
            <th>Profile</th>
            <th>Attempts</th>
          </tr>
        </thead>
        <tbody>
          <template v-for="job in collector.jobs">
            <tr v-if="ACTIVE_FILTERS[job.status]">
              <td>
                <CommitSha :tag="job.requestTag"></CommitSha>
              </td>
              <td>
                {{ formatJobStatus(job.status) }}
              </td>
              <td>
                {{ formatISODate(job.startedAt) }}
              </td>
              <td>
                {{ formatISODate(job.completedAt) }}
              </td>
              <td>{{ job.kind }}</td>
              <td>{{ formatBackend(job) }}</td>
              <td>
                {{ formatProfile(job) }}
              </td>
              <td>
                {{ job.dequeCounter }}
              </td>
            </tr>
          </template>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style lang="scss" scoped>
$sm-padding: 8px;
$sm-radius: 8px;

.collector-card {
  border-radius: $sm-radius;
  flex-direction: column;
  justify-content: space-between;
  padding: 16px;
  display: flex;
  box-shadow: 0 1px 2px #0006;
  margin: 0px 8px 8px 0px;
}
.collector-name {
  font-size: 1.5em;
  padding: $sm-padding;
}

.meta {
  padding: $sm-padding;
}

.collector-meta {
  display: flex;
}

.collector-meta-name {
  display: block;
  min-width: 125px;
}

.collector-left-divider {
  border-left: 2px solid black;
}

.collector-right-divider {
  border-right: 2px solid black;
}

.collector-sm-padding-left-right {
  padding: 0px $sm-padding;
}
.collector-sm-padding-left {
  padding-left: $sm-padding;
}
.collector-sm-padding-right {
  padding-right: $sm-padding;
}

.table-collector-status-filter-wrapper {
  padding: $sm-padding 0px;
}

.table-collector-status-filters {
  display: flex;
  flex-direction: column;
}

.table-collector-status-filter-btn-wrapper {
  padding-top: $sm-padding;
  display: flex;
  flex-direction: row;
}

.table-collector-status-filter-btn {
  border: 1px solid #333;
  border-radius: $sm-radius;
  width: 100%;
  margin-right: $sm-padding;
}

.table-collector-status-filter-btn:hover {
  transition: 250ms;
}

.status {
}

.status.benchmarking {
  background: #117411;
  color: white;
  font-weight: bold;
}
.status.waiting {
  background: #1b45e4;
  color: white;
  font-weight: bold;
}
.status.inactive {
  background: #ccc;
  color: white;
  font-weight: bold;
}
.status.offline {
  background: red;
  color: white;
  font-weight: bold;
}

.table-collector-wrapper {
  padding: $sm-padding;
  margin: $sm-padding 0px;
  background-color: #eee;
  border-radius: $sm-radius;

  table {
    font-size: 1em;
    border-collapse: collapse;
    width: 100%;
  }
}
.table-collector {
  caption {
    caption-side: top;
    text-align: left;
    font-variant: small-caps;
    font-weight: bold;
    font-size: 1.5em;
  }

  .table-header-row {
    border-bottom: 1px solid black;
  }

  .table-header-padding {
    padding: $sm-padding $sm-padding 0px $sm-padding;
    text-align: left;
  }

  .table-cell-padding {
    padding: $sm-padding $sm-padding 1px 0px;
    text-align: left;
  }
}

.collector-no-work {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 40px;
  background-color: #eee;
  margin: $sm-padding;
  padding: $sm-padding;
  border-radius: $sm-radius;

  h3 {
    font-variant: small-caps;
    font-weight: 700;
    font-size: 1.5em;
  }
}
</style>
