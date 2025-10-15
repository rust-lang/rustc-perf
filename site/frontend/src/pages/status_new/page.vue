<script setup lang="tsx">
import {h, ref, Ref} from "vue";

import {getJson} from "../../utils/requests";
import {STATUS_DATA_NEW_URL} from "../../urls";
import {withLoading} from "../../utils/loading";
import {
  formatSecondsAsDuration,
  formatISODate,
  shortenTag,
} from "../../utils/formatting";
import {useExpandedStore} from "../../utils/expansion";
import {
  BenchmarkRequest,
  BenchmarkRequestStatus,
  CollectorConfig,
  StatusResponse,
} from "./data";
import Collector from "./collector.vue";

const loading = ref(true);

const data: Ref<{
  timeline: BenchmarkRequestWithWaterLine[];
  queueLength: number;
  collectors: CollectorConfig[];
} | null> = ref(null);

type BenchmarkRequestWithWaterLine = BenchmarkRequest & {
  isLastInProgress: boolean;
  hasPendingJobs: boolean;
};

function getRequestRowClassName(req: BenchmarkRequestWithWaterLine) {
  const inProgress = req.status === "InProgress";
  if (inProgress && req.isLastInProgress) {
    return "timeline-waterline";
  } else if (inProgress) {
    return "timeline-row-bold";
  }
  return "";
}

async function loadStatusData(loading: Ref<boolean>) {
  data.value = await withLoading(loading, async () => {
    let resp: StatusResponse = await getJson<StatusResponse>(
      STATUS_DATA_NEW_URL
    );
    let timeline: BenchmarkRequestWithWaterLine[] = [];

    let queueLength = 0;

    let requests_with_pending_jobs = new Set();
    for (const job of resp.collectors.flatMap((c) => c.jobs)) {
      if (job.status === "Queued" || job.status === "InProgress") {
        requests_with_pending_jobs.add(job.requestTag);
      }
    }

    // Figure out where to draw the line.
    for (let i = 0; i < resp.requests.length; i++) {
      let request = resp.requests[i];
      let isLastInProgress =
        request.status === "InProgress" &&
        (i == resp.requests.length - 1 ||
          resp.requests[i + 1].status !== "InProgress");
      timeline.push({
        ...request,
        isLastInProgress,
        hasPendingJobs: requests_with_pending_jobs.has(request.tag),
      });

      if (request.status !== "Completed") {
        queueLength += 1;
      }
    }

    return {
      timeline,
      collectors: resp.collectors,
      queueLength,
    };
  });
}

function getDuration(request: BenchmarkRequest): string {
  if (request.status === "Completed") {
    return formatSecondsAsDuration(request.durationS);
  }
  return "";
}

function formatStatus(status: BenchmarkRequestStatus): string {
  if (status === "Completed") {
    return "Finished";
  } else if (status === "InProgress") {
    return "In progress";
  } else if (status === "Queued") {
    return "Queued";
  } else {
    return "Unknown";
  }
}

function hasErrors(errors: Dict<string>) {
  return Object.keys(errors).length !== 0;
}

function getErrorsLength(errors: Dict<string>) {
  const errorsLen = Object.keys(errors).length;
  return `${errorsLen} ${errorsLen > 1 ? "s" : ""}`;
}

function PullRequestLink({request}: {request: BenchmarkRequest}) {
  if (request.requestType === "Release") {
    return "";
  }
  return (
    <a href={`https://github.com/rust-lang/rust/pull/${request.pr}`}>
      #{request.pr}
    </a>
  );
}

const {toggleExpanded: toggleExpandedErrors, isExpanded: hasExpandedErrors} =
  useExpandedStore();

loadStatusData(loading);
</script>

<template>
  <div v-if="data !== null">
    <div class="status-page-wrapper">
      <div class="timeline-wrapper">
        <h1>Timeline</h1>
        <div style="margin-bottom: 10px">
          Queue length: {{ data.queueLength }}
        </div>
        <table>
          <thead>
            <tr>
              <th>PR</th>
              <th>Kind</th>
              <th>Tag</th>
              <th>Status</th>
              <th>Completed At</th>
              <th>Duration</th>
              <th>Errors</th>
            </tr>
          </thead>
          <tbody>
            <template v-for="req in data.timeline">
              <tr :class="getRequestRowClassName(req)">
                <td><PullRequestLink :request="req" /></td>
                <td>{{ req.requestType }}</td>
                <td>
                  {{ shortenTag(req.tag) }}
                </td>
                <td>
                  {{ formatStatus(req.status)
                  }}{{
                    req.status === "Completed" && req.hasPendingJobs ? "*" : ""
                  }}
                </td>
                <td>
                  {{ formatISODate(req.completedAt) }}
                  <span v-if="req.endEstimated">(est.)</span>
                </td>
                <td>
                  {{ getDuration(req) }}
                </td>

                <td v-if="hasErrors(req.errors)">
                  <button @click="toggleExpandedErrors(req.tag)">
                    {{ hasExpandedErrors(req.tag) ? "Hide" : "Show" }}
                    {{ getErrorsLength(req.errors) }}
                  </button>
                </td>
                <td v-else></td>
              </tr>

              <tr v-if="hasExpandedErrors(req.tag)">
                <td colspan="7" style="padding: 10px 0">
                  <div v-for="benchmark in Object.entries(req.errors)">
                    <div>
                      <details open>
                        <summary>{{ benchmark[0] }}</summary>
                        <pre class="error">{{ benchmark[1] }}</pre>
                      </details>
                    </div>
                  </div>
                </td>
              </tr>
            </template>
          </tbody>
        </table>
      </div>
      <div class="collector-wrapper">
        <h1>Collectors</h1>
        <div class="collectors-grid">
          <div v-for="collector in data.collectors" :key="collector.name">
            <Collector :collector="collector" />
          </div>
        </div>
      </div>
    </div>
  </div>
  <div v-else>Loading status…</div>
</template>

<style scoped lang="scss">
.status-page-wrapper {
  display: flex;
  flex-direction: column;
}

.collector-wrapper {
  width: 100%;
  display: flex;
  align-items: center;
  flex-direction: column;
  padding-left: 8px;
}

.timeline-waterline {
  border-bottom: 1px solid black;
  font-weight: bold;
}

.timeline-row-bold {
  font-weight: bold;
}

.timeline-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  height: fit-content;
  flex-direction: column;
  width: 100%;
  padding-right: 8px;

  table {
    border-collapse: collapse;
    font-size: 1.1em;
    width: 100%;

    th,
    td {
      padding: 0.2em;
    }

    th {
      text-align: center;
    }
    td {
      text-align: left;
      padding: 0 0.5em;

      &.centered {
        text-align: center;
      }
      &.right-align {
        text-align: right;
      }
    }
    tr.active {
      font-weight: bold;
    }
  }

  @media screen and (min-width: 1440px) {
    width: 100%;
  }
}

.wrapper {
  display: grid;
  column-gap: 100px;
  grid-template-columns: 1fr;

  @media screen and (min-width: 1440px) {
    grid-template-columns: 4fr 6fr;
  }
}
.current {
  max-width: 100%;
  width: fit-content;

  .benchmark {
    margin-bottom: 10px;
    font-size: 1.2em;
  }
}
.column-centered {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.current-table {
  border-collapse: collapse;
  font-size: 1.1em;

  td,
  th {
    padding: 0 10px;
  }
  tbody > tr {
    td {
      padding-top: 5px;
      text-align: center;
    }
  }
}
.aligned {
  text-align: right;
}
.error {
  padding: 10px;
  background-color: #f7f7f7;
  max-width: 100%;
  white-space: pre-wrap;
  word-break: break-word;
}

.collectors-grid {
  width: 100%;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(500px, 100%));
  gap: 20px;
}
</style>
