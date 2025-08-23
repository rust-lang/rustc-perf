<script setup lang="tsx">
import {ref, Ref} from "vue";

import {getJson} from "../../utils/requests";
import {STATUS_DATA_NEW_URL} from "../../urls";
import {withLoading} from "../../utils/loading";
import {
  StatusResponse,
  CollectorJobMap,
  BenchmarkRequestType,
  ReleaseCommit,
  createCollectorJobMap,
} from "./data";
import Collector from "./collector.vue";

async function loadStatusNew(loading: Ref<boolean>) {
  dataNew.value = await withLoading(loading, async () => {
    let d: StatusResponse = await getJson<StatusResponse>(STATUS_DATA_NEW_URL);
    return {
      ...d,
      collectorJobMap: createCollectorJobMap(d.collectorConfigs, d.inProgress),
    };
  });
}

const loading = ref(true);
const dataNew: Ref<
  (StatusResponse & {collectorJobMap: CollectorJobMap}) | null
> = ref(null);

function PullRequestLink({request}: {request: BenchmarkRequestType}) {
  if (request.type === ReleaseCommit) {
    return "";
  }
  return (
    <a href={`https://github.com/rust-lang/rust/pull/${request.pr}`}>
      #{request.pr}
    </a>
  );
}

function formatDuration(milliseconds: number): string {
  let seconds = milliseconds / 1000;
  let secs = seconds % 60;
  let mins = Math.trunc(seconds / 60);
  let hours = Math.trunc(mins / 60);
  mins -= hours * 60;

  let s = "";
  if (hours > 0) {
    s = `${hours}h ${mins < 10 ? "0" + mins : mins}m ${
      secs < 10 ? "0" + secs : secs
    }s`;
  } else {
    s = `${mins < 10 ? " " + mins : mins}m ${secs < 10 ? "0" + secs : secs}s`;
  }
  return s;
}

loadStatusNew(loading);
</script>

<template>
  <div id="app" class="container">
    <div v-if="dataNew !== null">
      <span>
        <div class="timeline">
          <h1>Previous</h1>
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
              <template v-for="req in dataNew.completed">
                <tr>
                  <td><PullRequestLink :request="req.requestType" /></td>
                  <td>{{ req.requestType.type }}</td>
                  <td>{{ req.requestType.tag }}</td>
                  <td>{{ req.status.state }}</td>
                  <td>{{ req.status.completedAt }}</td>
                  <td v-html="formatDuration(req.status.duration)"></td>
                  <td>{{ req.errors.join("\n") }}</td>
                </tr>
              </template>
            </tbody>
          </table>
        </div>
        <div class="timeline">
          <h1>Queue</h1>
          <table>
            <thead>
              <tr>
                <th>Pr</th>
                <th>Kind</th>
                <th>Sha / Tag</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              <template v-for="req in dataNew.queue">
                <tr>
                  <td><PullRequestLink :request="req.requestType" /></td>
                  <td>{{ req.requestType.type }}</td>
                  <td>{{ req.requestType.tag }}</td>
                  <td>{{ req.status.state }}</td>
                </tr>
              </template>
            </tbody>
          </table>
        </div>
      </span>
      <h1>Collectors</h1>
      <div class="grid">
        <div :key="cc[0]" v-for="cc in Object.entries(dataNew.collectorJobMap)">
          <Collector :collector="cc[1]" />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.timeline {
  max-width: 100%;
  width: fit-content;

  table {
    border-collapse: collapse;
    font-size: 1.1em;

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

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(500px, 1fr));
  gap: 20px;
}
</style>
