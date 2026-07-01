<script setup lang="ts">
/**
 * This component displays a rustc-perf command for profiling a compile benchmark with Cachegrind.
 **/

import {CompileTestCase} from "../../common";
import {computed, ref} from "vue";
import {normalizeProfile} from "./utils";
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

const codeBlock = ref<HTMLElement | null>(null);
const buttonText = ref("Copy");

const copyCode = async () => {
  if (codeBlock.value) {
    const textToCopy = codeBlock.value.innerText;
    try {
      await navigator.clipboard.writeText(textToCopy);
      buttonText.value = "Copied!";
      setTimeout(() => {
        buttonText.value = "Copy";
      }, 2000);
    } catch (err) {
      console.error("Copy error:", err);
      buttonText.value = "Error";
    }
  }
};
</script>

<template>
  <button style="margin-left: 10px" @click="copyCode">
    {{ buttonText }}
  </button>
  <pre><code ref="codeBlock">{{ cargo_collector_command() }} \
    profile_local cachegrind \
    +{{ firstCommit }} \<template v-if="props.baselineCommit !== undefined">
    --rustc2 +{{ props.commit }} \</template>
    --exact-match {{ testCase.benchmark }} \
    --profiles {{ normalizeProfile(testCase.profile) }} \
    --scenarios {{ normalizeScenario(testCase.scenario) }} \
    --parallels {{ testCase.parallel }}</code></pre>
</template>

<style scoped lang="scss">
pre {
  background-color: #eeeeee;
}

code {
  user-select: all;
}
</style>
