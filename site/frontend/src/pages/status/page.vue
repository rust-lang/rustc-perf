<script setup lang="ts">
import {getJson} from "../../utils/requests";
import {STATUS_DATA_URL} from "../../urls";
import {withLoading} from "../../utils/loading";
import {computed, ref, Ref} from "vue";
import {Artifact, Commit, MissingReason, StatusResponse} from "./data";

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
    s = `${hours}h${mins < 10 ? "0" + mins : mins}m${
      secs < 10 ? "0" + secs : secs
    }s`;
  } else {
    s = `${mins < 10 ? " " + mins : mins}m${secs < 10 ? "0" + secs : secs}s`;
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

function pullRequestUrlAsHtml(pr: number): string {
  return `<a href="https://github.com/rust-lang/rust/pull/${pr}">#${pr}</a>`;
}

function formatCommitAsHtml(commit: Commit, reason: MissingReason): string {
  const url = commitUrlAsHtml(commit.sha);
  const type = commit.type === "Try" ? "try" : "master";
  const pr = getArtifactPr(reason);

  return `${pullRequestUrlAsHtml(pr)} (${type}): ${url}`;
}

function formatMissingReason(reason: MissingReason): string {
  if (reason.hasOwnProperty("InProgress")) {
    return `${formatMissingReason(reason["InProgress"])} - in progress`;
  } else if (reason.hasOwnProperty("Master")) {
    return `${pullRequestUrlAsHtml(reason["Master"].pr)}${
      reason["Master"].is_try_parent ? " - Try commit parent" : ""
    }`;
  } else if (reason.hasOwnProperty("Try")) {
    return `Try for ${pullRequestUrlAsHtml(reason["Try"].pr)}`;
  } else {
    // Should never happen, but a reasonable fallback
    return JSON.stringify(reason);
  }
}

const timeLeft = computed(() => {
  const steps = data.value?.current?.progress ?? [];
  let left = 0;
  for (const step of steps) {
    if (!step.is_done) {
      left += step.expected_duration - step.current_progress;
    }
  }
  return left;
});
const recentEndDate = computed(() => {
  return new Date(data.value.most_recent_end * 1000);
});
const currentCommitAndReason: Ref<[Commit, MissingReason] | null> = computed(
  () => {
    const current = data.value?.current ?? null;
    if (current === null) return null;
    const sha = getArtifactSha(current.artifact);

    for (const [commit, reason] of data.value.missing) {
      if (commit.sha === sha && reason.hasOwnProperty("InProgress")) {
        return [commit, reason["InProgress"]];
      }
    }
    return null;
  }
);

const loading = ref(true);

const data: Ref<StatusResponse | null> = ref(null);
loadStatus(loading);
</script>

<template>
  <div v-if="data !== null">
    <div>
      <div v-if="data.current !== null">
        <p>
          Currently benchmarking:
          <span
            v-html="
              formatCommitAsHtml(
                currentCommitAndReason[0],
                currentCommitAndReason[1]
              )
            "
          ></span
          >.<br />
          Time left: {{ formatDuration(timeLeft) }}
        </p>
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
      <div v-else>
        <p>
          No current collection in progress.
          <template v-if="data.most_recent_end !== null"
            >Last one finished at {{ recentEndDate.toLocaleString() }} local
            time,
            {{
              formatDuration(
                Math.trunc((Date.now() - recentEndDate.getTime()) / 1000)
              )
            }}
            ago.
          </template>
        </p>
      </div>
      <p>
        Queue ({{ data.missing.length }} total):<br />
        Times are local.
      </p>
      <table class="missing-commits">
        <thead>
          <tr>
            <th>Commit Date</th>
            <th>SHA</th>
            <th>Reason</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="[commit, reason] in data.missing">
            <td>{{ new Date(commit.date).toLocaleString() }}</td>
            <td v-html="commitUrlAsHtml(commit.sha)"></td>
            <td v-html="formatMissingReason(reason)"></td>
          </tr>
        </tbody>
      </table>
    </div>
    <span>Benchmarking errors for last commit:</span>
    <div v-if="data.last_commit !== null">
      <p>SHA: {{ data.last_commit.sha }}, date: {{ data.last_commit.date }}</p>
    </div>
    <div v-for="benchmark in data.benchmarks">
      <div>
        <details open>
          <summary>{{ benchmark.name }} - error</summary>
          <pre>{{ benchmark.error }}</pre>
        </details>
      </div>
    </div>
  </div>
  <div v-else>Loading status…</div>
</template>

<style scoped lang="scss">
.aligned {
  text-align: right;
}
.missing-commits {
  th {
    text-align: center;
  }
  td {
    text-align: left;
    padding: 0 0.5em;
  }
}
</style>
