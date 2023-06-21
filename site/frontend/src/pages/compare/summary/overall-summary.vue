<script setup lang="ts">
import {SummaryGroup} from "../data";
import {computed} from "vue";
import SummaryTable from "./summary-table.vue";
import Tooltip from "../tooltip.vue";

const props = defineProps<{
  summary: SummaryGroup;
}>();
const summary = computed(() => props.summary);
</script>

<template>
  <div class="main-summary">
    <SummaryTable :summary="summary"></SummaryTable>
    <div style="position: absolute; right: 5px; top: 5px">
      <Tooltip style="margin-right: 1em">
        The table shows summaries of regressions, improvements and all changes
        calculated from data that is currently visible (data that passes the
        active filters).
      </Tooltip>
    </div>
  </div>
</template>

<style scoped lang="scss">
.main-summary {
  border: 1px dashed;
  padding: 4px;
  border-radius: 6px;

  display: flex;
  justify-content: center;
  position: relative;
}
</style>
