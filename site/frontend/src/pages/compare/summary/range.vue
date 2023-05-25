<script setup lang="ts">
import {computed} from "vue";
import SummaryPercentValue from "./percent-value.vue";

const props = defineProps<{range: Array<number>}>();
const range = computed(() => props.range);
</script>

<template>
  <div v-if="range.length > 0">
    [<SummaryPercentValue :value="range[0]" :padWidth="6" />,
    <SummaryPercentValue :value="range[1]" :padWidth="6" />]
  </div>
  <div v-else style="text-align: center">-</div>
</template>
