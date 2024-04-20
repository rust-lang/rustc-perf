<script setup lang="ts">
/**
 * This component displays a rustc-perf command for analyzing binary size diff between the baseline
 * and the new commit.
 **/

import {CompileTestCase} from "../../common";
import {ArtifactDescription} from "../../../types";
import Tooltip from "../../../tooltip.vue";
import {normalizeProfile} from "./utils";

const props = defineProps<{
  artifact: ArtifactDescription;
  baseArtifact: ArtifactDescription;
  testCase: CompileTestCase;
}>();

function normalizeBackend(backend: string): string {
  if (backend === "llvm") {
    return "Llvm";
  } else if (backend == "cranelift") {
    return "Cranelift";
  }
  return "<invalid backend>";
}
</script>

<template>
  <div class="title">
    Command for analyzing binary size locally
    <Tooltip>
      Execute this command in a checkout of
      <a href="https://github.com/rust-lang/rustc-perf">rustc-perf</a>, after a
      `cargo build --release`, to compare binary section sizes. Add `--symbols`
      to include a diff of symbol sizes.
    </Tooltip>
  </div>

  <pre><code>./target/release/collector binary_stats compile \
    +{{ props.baseArtifact.commit }} \
    --rustc2 +{{ props.artifact.commit }} \
    --include {{ testCase.benchmark }} \
    --profile {{ normalizeProfile(testCase.profile) }} \
    --backend {{ normalizeBackend(testCase.backend) }}</code></pre>
</template>

<style scoped lang="scss">
.title {
  font-weight: bold;
}

pre {
  background-color: #eeeeee;
}

code {
  user-select: all;
}
</style>
