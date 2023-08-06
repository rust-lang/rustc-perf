<script setup lang="ts">
import {
  CargoProfileMetadata,
  CompileBenchmarkMap,
  CompileBenchmarkMetadata,
  CompileTestCase,
} from "../common";
import {computed} from "vue";
import Tooltip from "../../tooltip.vue";

const props = defineProps<{
  testCase: CompileTestCase;
  benchmarkMap: CompileBenchmarkMap;
}>();
const metadata = computed(
  (): CompileBenchmarkMetadata =>
    props.benchmarkMap[props.testCase.benchmark] ?? null
);
const cargoProfile = computed((): CargoProfileMetadata => {
  if (
    props.testCase.profile === "opt" &&
    metadata?.value.release_profile !== null
  ) {
    return metadata.value.release_profile;
  } else if (
    props.testCase.profile === "debug" &&
    metadata?.value.dev_profile !== null
  ) {
    return metadata?.value.dev_profile;
  }
});
</script>
<template>
  <table>
    <tbody>
      <tr>
        <td>Benchmark</td>
        <td>{{ testCase.benchmark }}</td>
      </tr>
      <tr>
        <td>Profile</td>
        <td>{{ testCase.profile }}</td>
      </tr>
      <tr>
        <td>Scenario</td>
        <td>{{ testCase.scenario }}</td>
      </tr>
      <tr>
        <td>Category</td>
        <td>{{ testCase.category }}</td>
      </tr>
      <tr v-if="(metadata?.binary ?? null) !== null">
        <td>Artifact</td>
        <td>{{ metadata.binary ? "binary" : "library" }}</td>
      </tr>
      <tr v-if="(metadata?.iterations ?? null) !== null">
        <td>
          Iterations<Tooltip>
            How many times is the benchmark executed?
          </Tooltip>
        </td>
        <td>{{ metadata.iterations }}</td>
      </tr>
      <tr v-if="(cargoProfile?.lto ?? null) !== null">
        <td>LTO</td>
        <td>{{ cargoProfile.lto }}</td>
      </tr>
      <tr v-if="(cargoProfile?.debug ?? null) !== null">
        <td>Debuginfo</td>
        <td>{{ cargoProfile.debug }}</td>
      </tr>
      <tr v-if="(cargoProfile?.codegen_units ?? null) !== null">
        <td>Codegen units</td>
        <td>{{ cargoProfile.codegen_units }}</td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped lang="scss">
table {
  td {
    text-align: left;

    &:first-child {
      font-weight: bold;
      padding-right: 10px;
    }
  }
}
</style>
