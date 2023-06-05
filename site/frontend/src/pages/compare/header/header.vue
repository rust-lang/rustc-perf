<script setup lang="ts">
import {ArtifactDescription, CompareResponse, CompareSelector} from "../types";
import {formatDate} from "../shared";
import {computed} from "vue";

const props = defineProps<{
  data: CompareResponse | null;
  selector: CompareSelector;
}>();

function prLink(pr: number | null): string {
  return `https://github.com/rust-lang/rust/pull/${pr}`;
}
function commitLink(commit: string): string {
  return `https://github.com/rust-lang/rust/commit/${commit}`;
}
function short(artifact: ArtifactDescription): string {
  return shortCommit(artifact.commit);
}
function shortCommit(commit): string {
  return commit.substring(0, 8);
}
function formatBound(
  artifact: ArtifactDescription | null,
  bound: string
): string {
  if (artifact === null) {
    return bound ? bound.substring(0, 10) : "???";
  }
  if (artifact.pr) {
    return `#${artifact.pr}`;
  }
  if (artifact.date) {
    return formatDate(artifact.date);
  }

  // 0,7 extracts 7 chars from the git commit id/tag, which is probably
  // enough to distinguish it. (It is only for display purposes.)
  return artifact.commit.substring(0, 7);
}

const prevLink = computed(
  () =>
    `/compare.html?start=${props.data.prev}&end=${props.data.a.commit}&stat=${props.selector.stat}`
);
const nextLink = computed(
  () =>
    `/compare.html?start=${props.data.b.commit}&end=${props.data.next}&stat=${props.selector.stat}`
);
const compareLink = computed(
  () =>
    `https://github.com/rust-lang/rust/compare/${props.data.a.commit}...${props.data.b.commit}`
);

const before = computed(() =>
  formatBound(props.data?.a ?? null, props.selector.start)
);
const after = computed(() =>
  formatBound(props.data?.b ?? null, props.selector.end)
);
</script>

<template>
  <h2 style="text-align: center">
    Comparing <span id="stat-header">{{ selector.stat }}</span> between
    <span id="before">{{ before }}</span> and
    <span id="after">{{ after }}</span>
  </h2>
  <div v-if="props.data !== null" style="margin: 12px 0">
    <div style="display: flex; justify-content: center">
      <div class="description-box">
        <div v-if="data.prev" class="description-arrow">
          <a v-bind:href="prevLink">&larr;</a>
        </div>
        <div style="padding: 10px">
          <span
            ><a v-if="data.a.pr" v-bind:href="prLink(data.a.pr)"
              >#{{ data.a.pr }}</a
            >&nbsp;</span
          >
          <span v-if="data.a.date">{{ formatDate(data.a.date) }}</span>
          (<a v-bind:href="commitLink(data.a.commit)">{{ short(data.a) }}</a
          >)
        </div>
      </div>
      <div
        v-if="!data.is_contiguous"
        class="not-continuous"
        title="WARNING! The commits are not continuous."
      >
        ...
      </div>
      <div class="description-box">
        <div style="padding: 10px">
          <span
            ><a v-if="data.b.pr" v-bind:href="prLink(data.b.pr)"
              >#{{ data.b.pr }}</a
            >&nbsp;</span
          >
          <span v-if="data.b.date">{{ formatDate(data.b.date) }}</span>
          (<a v-bind:href="commitLink(data.b.commit)">{{ short(data.b) }}</a
          >)
        </div>
        <div v-if="data.next" class="description-arrow">
          <a v-bind:href="nextLink">&rarr;</a>
        </div>
      </div>
    </div>
    <div style="display: flex; justify-content: center">
      <a v-bind:href="compareLink">🔎 compare commits</a>
    </div>
  </div>
</template>

<style scoped lang="scss">
.description-box {
  border: 1px solid;
  display: flex;
}
.description-arrow {
  display: flex;
  flex-direction: column;
  justify-content: center;
  background: #8dcc8d;
  padding: 5px;
}
.not-continuous {
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  background: #cc3300;
  border: 1px solid;
  cursor: help;
}
</style>
