<script setup lang="ts">
import {computed, inject} from "vue";
import {openTraceInPerfetto} from "../perfetto";
import {chromeProfileUrl} from "../self-profile";
import {CompileTestCase} from "../pages/compare/compile/common";
import {ArtifactDescription} from "../pages/compare/types";

const props = defineProps<{
  artifact: ArtifactDescription;
  testCase: CompileTestCase;
}>();

const link = computed(() => {
  return chromeProfileUrl(
    props.artifact.commit,
    `${props.testCase.benchmark}-${props.testCase.profile.toLowerCase()}`,
    props.testCase.scenario
  );
});

// This title will appear as the trace name in Perfetto
const traceTitle = computed(() => {
  return `${props.testCase.benchmark}-${props.testCase.profile}-${props.testCase.scenario} (${props.artifact.commit})`;
});

const isEmbeddedPerfettoEnabled = inject("isEmbeddedPerfettoEnabled");
</script>

<template>
  <a
    :href="`/perfetto/index.html?url=${encodeURIComponent(link)}`"
    v-if="isEmbeddedPerfettoEnabled"
    ><slot></slot
  ></a>
  <a
    @click="openTraceInPerfetto(link, traceTitle)"
    title="Open the self-profile query trace in Perfetto. You have to wait a bit for the profile to be downloaded after clicking on the link."
    v-else
  >
    <slot></slot>
  </a>
</template>

<style scoped lang="scss">
a {
  cursor: pointer;
}
</style>
