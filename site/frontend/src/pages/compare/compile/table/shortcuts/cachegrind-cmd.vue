<script setup lang="ts">
/**
 * This component displays a rustc-perf command for profiling a compile benchmark with Cachegrind.
 **/

import {CompileTestCase} from "../../common";
import {computed} from "vue";
import {normalizeProfile, normalizeScenario} from "./utils";
import {cargo_collector_command} from "../../../../../utils/cargo";

const props = defineProps<{
  commit: string;
  testCase: CompileTestCase;
  baselineCommit?: string;
}>();

const firstCommit = computed(() => {
  if (props.baselineCommit !== undefined) {
    return props.baselineCommit;
  } else {
    return props.commit;
  }
});
</script>

<template>
  <pre><code>{{ cargo_collector_command() }} \
    profile_local cachegrind \
    +{{ firstCommit }} \<template v-if="props.baselineCommit !== undefined">
    --rustc2 +{{ props.commit }} \</template>
    --exact-match {{ testCase.benchmark }} \
    --profiles {{ normalizeProfile(testCase.profile) }} \
    --scenarios {{ normalizeScenario(testCase.scenario) }}</code></pre>
</template>

<style scoped lang="scss">
pre {
  background-color: #eeeeee;
}

code {
  user-select: all;
}
</style>
