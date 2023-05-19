<script setup lang="ts">
import {computed} from "vue";
import {signIfPositive} from "../shared";

const props = withDefaults(defineProps<{
  value: number,
  padWidth?: number | null
}>(), { padWidth: null });

const formattedValue = computed((): string => {
  return `${signIfPositive(props.value)}${props.value.toFixed(2)}`;
});
const padSpaces = computed((): string => {
  const value = formattedValue.value;
  if (value.length < this.padWidth) {
    return "&nbsp;".repeat(this.padWidth - value.length);
  }
  return "";
});
</script>

<template>
  <span><span v-html="padSpaces" />{{ formattedValue }}%</span>
</template>
