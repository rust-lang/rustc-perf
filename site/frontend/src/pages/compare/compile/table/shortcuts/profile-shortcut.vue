<script setup lang="ts">
/**
 * This component displays a select box that allows users to choose between three variants
 * of the `CachegrindCmd` component with a local `rustc-perf` profiling command.
 * Users can choose either to profile the baseline commit, the new commit, or to profile both
 * and display a diff.
 **/

import {computed, ref, Ref} from "vue";
import {CompileTestCase} from "../../common";
import {ArtifactDescription} from "../../../types";
import Tooltip from "../../../tooltip.vue";
import CachegrindCmd from "./cachegrind-cmd.vue";

const props = defineProps<{
  artifact: ArtifactDescription;
  baseArtifact: ArtifactDescription;
  testCase: CompileTestCase;
}>();

enum ProfileCommand {
  Before = "before",
  After = "after",
  Diff = "diff",
}

function changeProfileCommand(event: Event) {
  const target = event.target as HTMLSelectElement;
  profileCommand.value = target.value as ProfileCommand;
}

const profileCommand: Ref<ProfileCommand> = ref(ProfileCommand.Diff);
const profileCommit = computed(() => {
  if (profileCommand.value === ProfileCommand.Before) {
    return props.baseArtifact.commit;
  }
  return props.artifact.commit;
});
const profileBaselineCommit = computed(() => {
  if (profileCommand.value === ProfileCommand.Diff) {
    return props.baseArtifact.commit;
  }
  return undefined;
});
</script>

<template>
  <div class="title">
    Command for profiling locally
    <Tooltip>
      Execute this command in a checkout of
      <a href="https://github.com/rust-lang/rustc-perf">rustc-perf</a>, after a
      `cargo build --release`, to generate a Cachegrind profile.
    </Tooltip>
  </div>

  <select @change="changeProfileCommand">
    <option
      :value="ProfileCommand.Diff"
      :selected="profileCommand === ProfileCommand.Diff"
    >
      Diff
    </option>
    <option
      :value="ProfileCommand.Before"
      :selected="profileCommand === ProfileCommand.Before"
    >
      Baseline commit
    </option>
    <option
      :value="ProfileCommand.After"
      :selected="profileCommand === ProfileCommand.After"
    >
      Benchmarked commit
    </option>
  </select>

  <CachegrindCmd
    :commit="profileCommit"
    :baseline-commit="profileBaselineCommit"
    :test-case="props.testCase"
  />
</template>

<style scoped lang="scss">
.title {
  font-weight: bold;
}
</style>
