<script setup lang="ts">
import {getJson} from "../../utils/requests";
import {STATUS_DATA_URL} from "../../urls";
import {withLoading} from "../../utils/loading";
import {computed, ref, Ref} from "vue";
import {
  Artifact,
  BenchmarkError,
  Commit,
  FinishedRun,
  MissingReason,
  StatusResponse,
} from "./data";
import {
  addSeconds,
  differenceInSeconds,
  format,
  fromUnixTime,
  subSeconds,
} from "date-fns";
import {useExpandedStore} from "./expansion";

async function loadStatus(loading: Ref<boolean>) {
  data.value = await withLoading(loading, () =>
    getJson<StatusResponse>(STATUS_DATA_URL)
  );
}

function formatDuration(seconds: number): string {
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

function getArtifactPr(reason: MissingReason): number {
  if (reason.hasOwnProperty("InProgress")) {
    return getArtifactPr(reason["InProgress"]);
  } else if (reason.hasOwnProperty("Master")) {
    return reason["Master"].pr;
  } else if (reason.hasOwnProperty("Try")) {
    return reason["Try"].pr;
  } else {
    throw new Error(`Unknown missing reason ${reason}`);
  }
}

function getArtifactSha(artifact: Artifact): string {
  if (artifact.hasOwnProperty("Commit")) {
    return artifact["Commit"].sha;
  } else if (artifact.hasOwnProperty("Tag")) {
    return artifact["Tag"];
  } else {
    throw new Error(`Unknown artifact ${artifact}`);
  }
}

function commitUrlAsHtml(sha: string): string {
  return `<a href="https://github.com/rust-lang/rust/commit/${sha}">${sha.substring(
    0,
    13
  )}</a>`;
}

function pullRequestUrlAsHtml(pr: number | null): string {
  if (pr === null) {
    return "";
  }
  return `<a href="https://github.com/rust-lang/rust/pull/${pr}">#${pr}</a>`;
}

function formatCommitAsHtml(commit: Commit, reason: MissingReason): string {
  const url = commitUrlAsHtml(commit.sha);
  const type = commit.type === "Try" ? "Try" : "Master";
  const pr = getArtifactPr(reason);

  return `${type} commit ${url} (${pullRequestUrlAsHtml(pr)})`;
}

interface CurrentRun {
  commit: Commit;
  missing_reason: MissingReason;
  start: Date;
  expected_end: Date;
  expected_total: number;
  progress: number;
}

interface TimelineEntry {
  pr: number | null;
  kind: string;
  sha: string;
  status: string;
  end_time: Date;
  end_estimated: boolean;
  duration: number | null;
  errors: BenchmarkError[];
  warning: string | null;
  current: boolean;
}

const timeLeft = computed(() => {
  const steps = data.value?.current?.progress ?? [];
  let left = 0;
  for (const step of steps) {
    if (!step.is_done) {
      left += Math.max(0, step.expected_duration - step.current_progress);
    }
  }
  return left;
});

const currentRun: Ref<CurrentRun | null> = computed(() => {
  const current = data.value?.current ?? null;
  if (current === null) return null;
  const sha = getArtifactSha(current.artifact);

  const elapsed = current.progress.reduce(
    (prev, current) => prev + current.current_progress,
    0
  );
  const start = subSeconds(new Date(), elapsed);
  const expected_total = current.progress.reduce(
    (prev, current) => prev + current.expected_duration,
    0
  );
  const progress = Math.min(1, elapsed / Math.max(1, expected_total));

  for (const [commit, reason] of data.value.missing) {
    if (commit.sha === sha && reason.hasOwnProperty("InProgress")) {
      return {
        commit,
        missing_reason: reason["InProgress"],
        start,
        expected_end: addSeconds(new Date(), timeLeft.value),
        expected_total,
        progress,
      };
    }
  }
  return null;
});
const lastFinishedRun: Ref<FinishedRun | null> = computed(() => {
  const finished = data.value?.finished_runs;
  if (finished.length === 0) return null;
  return finished[0];
});
const expectedDuration: Ref<number | null> = computed(() => {
  if (data.value === null) return null;
  const current = currentRun.value;
  if (current !== null) {
    return current.expected_total;
  } else {
    return lastFinishedRun.value?.duration;
  }
});

// Timeline of past, current and future runs.
// Ordered from future to past - at the beginning is the item with the least
// priority in the queue, and at the end is the oldest finished run.
const timeline: Ref<TimelineEntry[]> = computed(() => {
  if (data.value === null) return [];
  const timeline: TimelineEntry[] = [];

  const current = currentRun.value;
  const currentTimeLeft = timeLeft.value ?? 0;
  const expectedRunDuration = expectedDuration.value;

  // The next commit to be benchmarked will be at last position of `queued`
  const queued = data.value.missing
    .filter(([_, reason]) => !reason.hasOwnProperty("InProgress"))
    .reverse();
  let queued_before = queued.length - 1;
  const currentRunEnd =
    current?.expected_end ?? addSeconds(new Date(), currentTimeLeft);
  for (const [commit, reason] of queued) {
    const expected_end = addSeconds(
      currentRunEnd,
      queued_before * expectedRunDuration + expectedRunDuration
    );

    let kind = commit.type;
    if (reason.hasOwnProperty("Master") && reason["Master"].is_try_parent) {
      kind += " (try parent)";
    }

    timeline.push({
      pr: getArtifactPr(reason),
      kind,
      sha: commit.sha,
      status: `Queued (#${queued_before + 1})`,
      end_time: expected_end,
      end_estimated: true,
      errors: [],
      duration: null,
      warning: null,
      current: false,
    });
    queued_before -= 1;
  }

  if (current !== null) {
    timeline.push({
      pr: getArtifactPr(current.missing_reason),
      kind: current.commit.type,
      sha: current.commit.sha,
      status: "In progress",
      end_time: current.expected_end,
      end_estimated: true,
      errors: [],
      duration: null,
      warning: null,
      current: true,
    });
  }

  for (const run of data.value.finished_runs) {
    const kind = run.artifact.hasOwnProperty("Tag")
      ? run.artifact["Tag"]
      : run.artifact["Commit"].type;

    let warning = null;
    if (kind !== "Try" && run.errors.length > 0) {
      warning = "Master commit with errors";
    }
    timeline.push({
      pr: run.pr,
      kind,
      sha: getArtifactSha(run.artifact),
      status: "Finished",
      end_time: fromUnixTime(run.finished_at),
      end_estimated: false,
      errors: run.errors,
      duration: run.duration,
      warning,
      current: false,
    });
  }

  return timeline;
});

const {toggleExpanded: toggleExpandedErrors, isExpanded: hasExpandedErrors} =
  useExpandedStore();

const loading = ref(true);

const data: Ref<StatusResponse | null> = ref(null);
loadStatus(loading);
</script>

<template>
  <div class="wrapper" v-if="data !== null">
    <div class="current column-centered">
      <h3><b>Current collection</b></h3>
      <div class="column-centered" v-if="currentRun !== null">
        <div
          class="benchmark"
          v-html="
            formatCommitAsHtml(currentRun.commit, currentRun.missing_reason)
          "
        ></div>
        <table class="current-table">
          <thead>
            <tr>
              <th>Start</th>
              <th>Progress</th>
              <th>Expected end</th>
              <th>Time left</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>{{ format(currentRun.start, "HH:mm") }}</td>
              <td>
                <progress
                  max="100"
                  :value="currentRun.progress * 100"
                ></progress>
              </td>
              <td>
                {{ format(currentRun.expected_end, "HH:mm") }}
              </td>
              <td>{{ timeLeft <= 0 ? "?" : formatDuration(timeLeft) }}</td>
            </tr>
          </tbody>
        </table>
        <h4 style="margin-top: 20px">Detailed progress</h4>
        <table>
          <thead>
            <tr>
              <th>Step</th>
              <th></th>
              <th>Took</th>
              <th>Expected</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="step in data.current.progress">
              <td>{{ step.step }}</td>
              <td>
                <progress
                  :max="step.expected_duration.toString()"
                  :value="
                    (step.is_done
                      ? step.expected_duration
                      : step.current_progress
                    ).toString()
                  "
                ></progress>
              </td>
              <td class="aligned">
                {{
                  step.current_progress == 0
                    ? ""
                    : formatDuration(step.current_progress)
                }}
              </td>
              <td class="aligned">
                {{ formatDuration(step.expected_duration) }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <div class="column-centered" v-else>
        <div>No benchmark in progress.</div>
        <div>
          Last collection finished at
          {{ fromUnixTime(lastFinishedRun.finished_at).toLocaleString() }} ({{
            formatDuration(
              differenceInSeconds(
                new Date(),
                fromUnixTime(lastFinishedRun.finished_at)
              )
            )
          }}
          ago)
        </div>
      </div>
    </div>
    <div class="column-centered timeline">
      <h3><b>Timeline</b></h3>
      <div style="margin-bottom: 10px">
        Queue length: {{ data.missing.length }}
      </div>
      <table>
        <thead>
          <tr>
            <th>PR</th>
            <th>Kind</th>
            <th>SHA</th>
            <th>Status</th>
            <th>Run end</th>
            <th>Duration</th>
            <th>Errors</th>
          </tr>
        </thead>
        <tbody>
          <template v-for="item in timeline">
            <tr v-if="item.current">
              <td colspan="7"><hr /></td>
            </tr>
            <tr :class="{active: item.sha === currentRun?.commit?.sha}">
              <td
                class="right-align"
                v-html="pullRequestUrlAsHtml(item.pr)"
              ></td>
              <td class="centered">
                {{ item.kind
                }}<span v-if="item.warning !== null" :title="item.warning"
                  >❗</span
                >
              </td>
              <td class="right-align" v-html="commitUrlAsHtml(item.sha)"></td>
              <td class="centered">{{ item.status }}</td>
              <td>
                {{ item.end_time.toLocaleString()
                }}<template v-if="item.end_estimated"> (est.)</template>
              </td>
              <td v-if="item.duration !== null">
                {{ formatDuration(item.duration) }}
              </td>
              <td v-else class="centered">-</td>
              <td v-if="item.errors.length > 0">
                <button @click="toggleExpandedErrors(item.sha)">
                  {{ hasExpandedErrors(item.sha) ? "Hide" : "Show" }}
                  {{ item.errors.length }} error{{
                    item.errors.length !== 1 ? "s" : ""
                  }}
                </button>
              </td>
              <td v-else></td>
            </tr>
            <tr v-if="hasExpandedErrors(item.sha)">
              <td colspan="7" style="padding: 10px 0">
                <div v-for="benchmark in item.errors">
                  <div>
                    <details open>
                      <summary>{{ benchmark.name }}</summary>
                      <pre class="error">{{ benchmark.error }}</pre>
                    </details>
                  </div>
                </div>
              </td>
            </tr>
            <tr v-if="item.current">
              <td colspan="7"><hr /></td>
            </tr>
          </template>
        </tbody>
      </table>
    </div>
  </div>
  <div v-else>Loading status…</div>
  <div style="margin-top: 10px">Times are local.</div>
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
</style>
