<script setup lang="ts">
import {Suspense} from "vue";

const props = defineProps<{
  component: Object;
}>();
const component = props.component as any;
</script>

<template>
  <Suspense>
    <component :is="component" />
  </Suspense>
</template>
