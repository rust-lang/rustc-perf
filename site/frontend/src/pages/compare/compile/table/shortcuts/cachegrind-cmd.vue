<script setup lang="ts">
/**
 * This component displays a rustc-perf command for profiling a compile benchmark with Cachegrind.
 **/

import {CompileTestCase} from "../../common";
import {computed} from "vue";
import {normalizeProfile} from "./utils";

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

function normalizeScenario(scenario: string): string {
  if (scenario === "full") {
    return "Full";
  } else if (scenario === "incr-full") {
    return "IncrFull";
  } else if (scenario === "incr-unchanged") {
    return "IncrUnchanged";
  } else if (scenario.startsWith("incr-patched")) {
    return "IncrPatched";
  }
  return "<invalid scenario>";
}
</script>

<template>
  <pre><code>./target/release/collector profile_local cachegrind \
    +{{ firstCommit }} \<template v-if="props.baselineCommit !== undefined">
    --rustc2 +{{ props.commit }} \</template>
    --include {{ testCase.benchmark }} \
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
