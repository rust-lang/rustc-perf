<script setup lang="ts">
import {computed} from "vue";
import {percentClass, signIfPositive} from "../shared";

const props = withDefaults(
  defineProps<{
    value: number;
    padWidth?: number | null;
  }>(),
  {padWidth: null}
);

const formattedValue = computed((): string => {
  return `${signIfPositive(props.value)}${props.value.toFixed(2)}`;
});
const padSpaces = computed((): string => {
  const value = formattedValue.value;
  if (value.length < props.padWidth) {
    return "&nbsp;".repeat(props.padWidth - value.length);
  }
  return "";
});
</script>

<template>
  <span :class="percentClass(props.value)"
    ><span v-html="padSpaces" />{{ formattedValue }}%</span
  >
</template>
