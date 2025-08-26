<script setup lang="tsx">
import {ref, Ref} from "vue";

import {getJson} from "../../utils/requests";
import {STATUS_DATA_NEW_URL} from "../../urls";
import {withLoading} from "../../utils/loading";
import {formatDuration} from "../../utils/formatting";
import {
  StatusResponse,
  CollectorJobMap,
  BenchmarkRequestType,
  BenchmarkRequest,
  ReleaseCommit,
  createCollectorJobMap,
  createTimeline,
  BenchmarkRequestCompleteStr,
} from "./data";
import Collector from "./collector.vue";

async function loadStatusNew(loading: Ref<boolean>) {
  dataNew.value = await withLoading(loading, async () => {
    let d: StatusResponse = await getJson<StatusResponse>(STATUS_DATA_NEW_URL);
    return {
      queueLength: d.queue.length,
      collectorJobMap: createCollectorJobMap(d.collectorConfigs, d.inProgress),
      timeline: createTimeline(d.completed, d.queue),
    };
  });
}

const loading = ref(true);
/* @TODO; redo type */
const dataNew: Ref<{
  queueLength: number;
  collectorJobMap: CollectorJobMap;
  timeline: BenchmarkRequest[];
} | null> = ref(null);

function pullRequestUrlAsHtml(reqType: BenchmarkRequestType): string {
  if (reqType.type === ReleaseCommit) {
    return "";
  }
  return `<a href="https://github.com/rust-lang/rust/pull/${reqType.pr}">#${reqType.pr}</a>`;
}

function getCreatedAt(request: BenchmarkRequest): string {
  if (request.status.state == BenchmarkRequestCompleteStr) {
    return request.status.completedAt;
  }
  return "";
}

function getDuration(request: BenchmarkRequest): string {
  if (request.status.state == BenchmarkRequestCompleteStr) {
    return formatDuration(request.status.duration);
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
              <tr>
                <td><PullRequestLink :requestType="req.requestType" /></td>
                <td>{{ req.requestType.type }}</td>
                <td>
                  {{ req.requestType.tag }}
                </td>
                <td>{{ req.status.state }}</td>
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
            :key="cc[0]"
            v-for="cc in Object.entries(dataNew.collectorJobMap)"
          >
            <Collector :collector="cc[1]" />
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
