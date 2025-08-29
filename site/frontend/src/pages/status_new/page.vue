<script setup lang="tsx">
import {h, ref, Ref} from "vue";

import {getJson} from "../../utils/requests";
import {STATUS_DATA_NEW_URL} from "../../urls";
import {withLoading} from "../../utils/loading";
import {formatSecondsAsDuration} from "../../utils/formatting";
import {
  StatusResponse,
  BenchmarkRequestType,
  BenchmarkRequest,
  BenchmarkJob,
  CollectorInfo,
  ReleaseCommit,
  BenchmarkRequestCompleteStr,
  BenchmarkRequestInProgressStr,
} from "./data";
import Collector from "./collector.vue";

const loading = ref(true);

const dataNew: Ref<{
  queueLength: number;
  timeline: BenchmarkRequestWithWaterLine[];
  requestsMap: Dict<BenchmarkRequest>;
  jobMap: Dict<BenchmarkJob>;
  collectorWorkMap: Dict<CollectorInfo>;
  tagToJobs: Dict<number[]>;
} | null> = ref(null);

type BenchmarkRequestWithWaterLine = BenchmarkRequest & {isWaterLine: boolean};

function requestIsInProgress(req: BenchmarkRequest, tagToJobs: Dict<number[]>) {
  switch (req.status.state) {
    case BenchmarkRequestCompleteStr:
      if (req.requestType.tag in tagToJobs) {
        return true;
      }
      return false;
    case BenchmarkRequestInProgressStr:
      return true;
    default:
      return false;
  }
}

function getRequestRowClassName(
  req: BenchmarkRequestWithWaterLine,
  tagToJobs: Dict<number[]>
) {
  const inProgress = requestIsInProgress(req, tagToJobs);
  if (inProgress && req.isWaterLine) {
    return "timeline-waterline";
  } else if (inProgress) {
    return "timeline-row-bold";
  }
  return "";
}

async function loadStatusNew(loading: Ref<boolean>) {
  dataNew.value = await withLoading(loading, async () => {
    let d: StatusResponse = await getJson<StatusResponse>(STATUS_DATA_NEW_URL);
    let timeline: BenchmarkRequestWithWaterLine[] = [];
    // figure out where to draw the line.
    for (let i = 1; i < d.queueRequestTags.length; ++i) {
      let req = d.requestsMap[d.queueRequestTags[i - 1]];
      let nextReq = d.requestsMap[d.queueRequestTags[i]];
      let isWaterLine = false;
      if (
        requestIsInProgress(req, d.tagToJobs) &&
        !requestIsInProgress(nextReq, d.tagToJobs)
      ) {
        isWaterLine = true;
      }
      timeline.push({
        ...req,
        isWaterLine,
      });
    }
    return {
      queueLength: d.queueRequestTags.length,
      timeline,
      requestsMap: d.requestsMap,
      jobMap: d.jobMap,
      collectorWorkMap: d.collectorWorkMap,
      tagToJobs: d.tagToJobs,
    };
  });
}

function getCreatedAt(request: BenchmarkRequest): string {
  if (request.status.state == BenchmarkRequestCompleteStr) {
    return request.status.completedAt;
  }
  return "";
}

function getDuration(request: BenchmarkRequest): string {
  if (request.status.state == BenchmarkRequestCompleteStr) {
    return formatSecondsAsDuration(request.status.duration_s);
  }
  return "";
}

function PullRequestLink({requestType}: {requestType: BenchmarkRequestType}) {
  if (requestType.type === ReleaseCommit) {
    return "";
  }
  return (
    <a href={`https://github.com/rust-lang/rust/pull/${requestType.pr}`}>
      #{requestType.pr}
    </a>
  );
}

loadStatusNew(loading);
</script>

<template>
  <div v-if="dataNew !== null">
    <div class="status-page-wrapper">
      <div class="timeline-wrapper">
        <h1>Timeline</h1>
        <div style="margin-bottom: 10px">
          Queue length: {{ dataNew.queueLength }}
        </div>
        <table>
          <thead>
            <tr>
              <th>Pr</th>
              <th>Kind</th>
              <th>Sha / Tag</th>
              <th>Status</th>
              <th>Completed At</th>
              <th>Duration</th>
              <th>Errors</th>
            </tr>
          </thead>
          <tbody>
            <template v-for="req in dataNew.timeline">
              <tr :class="getRequestRowClassName(req, dataNew.tagToJobs)">
                <td><PullRequestLink :requestType="req.requestType" /></td>
                <td>{{ req.requestType.type }}</td>
                <td>
                  {{ req.requestType.tag }}
                </td>
                <td>
                  {{
                    req.status.state === BenchmarkRequestCompleteStr &&
                    req.requestType.tag in dataNew.tagToJobs
                      ? `${req.status.state}*`
                      : `${req.status.state}`
                  }}
                </td>
                <td v-html="getCreatedAt(req)"></td>
                <td v-html="getDuration(req)"></td>
                <td>
                  <pre>{{ req.errors.join("\n") }}</pre>
                </td>
              </tr>
            </template>
          </tbody>
        </table>
      </div>
      <div class="collector-wrapper">
        <h1>Collectors</h1>
        <div class="collectors-grid">
          <div
            v-for="cc in Object.values(dataNew.collectorWorkMap)"
            :key="cc.config.name"
          >
            <Collector :jobMap="dataNew.jobMap" :collector="cc" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.status-page-wrapper {
  display: flex;
  @media screen and (max-width: 1450px) {
    flex-direction: column;
  }
}

.collector-wrapper {
  width: 100%;
  display: flex;
  justify-content: center;
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
  grid-template-columns: repeat(auto-fill, minmax(500px, 1fr));
  gap: 20px;
}
</style>
