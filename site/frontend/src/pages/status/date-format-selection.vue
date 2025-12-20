<script setup lang="ts">
import {DATE_FMT_12HR} from "../../utils/formatting";

const props = defineProps<{
  dateFmt: string;
  toggleDate(): void;
}>();
</script>

<template>
  <span>
    <input
      @click="props.toggleDate"
      :checked="props.dateFmt === DATE_FMT_12HR"
      type="checkbox"
      id="user-date-fmt-toggle"
    />
    <label for="user-date-fmt-toggle">Show 12hr date format</label><br />
  </span>
</template>
