<script setup lang="ts">
import {BenchmarkInfo} from "../api";

const props = defineProps<{
  info: BenchmarkInfo;
}>();
</script>
<template>
  <div id="as-of">
    Updated as of: {{ new Date(props.info.as_of).toLocaleString() }}
  </div>
</template>
