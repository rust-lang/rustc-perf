<script setup lang="ts">
import {computed} from "vue";
import {openTraceInPerfetto} from "../perfetto";
import {croxTraceUrl} from "../self-profile";
import {CompileTestCase} from "../pages/compare/compile/common";
import {ArtifactDescription} from "../pages/compare/types";

const props = defineProps<{
  label: string;
  artifact: ArtifactDescription;
  testCase: CompileTestCase;
}>();

const link = computed(() => {
  return croxTraceUrl(
    props.artifact.commit,
    `${props.testCase.benchmark}-${props.testCase.profile.toLowerCase()}`,
    props.testCase.scenario
  );
});
const traceTitle = computed(() => {
  return `${props.testCase.benchmark}-${props.testCase.profile}-${props.testCase.scenario} (${props.artifact.commit})`;
});
</script>

<template>
  <a @click="openTraceInPerfetto(link, traceTitle)">{{ props.label }}</a>
</template>

<style scoped lang="scss">
a {
  cursor: pointer;
}
</style>
