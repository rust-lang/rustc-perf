<script setup lang="ts">
import {CompareResponse} from "../types";
import OverallSummary from "../summary/overall-summary.vue";
import {computed, ref} from "vue";
import {computeSummary, filterNonRelevant} from "../data";
import {
  computeRuntimeComparisonsWithNonRelevant,
  defaultRuntimeFilter,
} from "./common";
import {deepCopy} from "../../../utils/copy";

const props = defineProps<{data: CompareResponse}>();

const filter = ref(deepCopy(defaultRuntimeFilter));
const allComparisons = computed(() =>
  computeRuntimeComparisonsWithNonRelevant(
    filter.value,
    props.data.runtime_comparisons
  )
);
const comparisons = computed(() =>
  filterNonRelevant(filter.value, allComparisons.value)
);
const filteredSummary = computed(() => computeSummary(comparisons.value));
</script>

<template>
  <OverallSummary :summary="filteredSummary" />
</template>
