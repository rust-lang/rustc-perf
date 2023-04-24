<script setup lang="ts">
import {Component, Suspense} from "vue";

const props = defineProps<{
  component: Component
}>();
</script>

<template>
  <Suspense>
    <component :is="props.component" />
  </Suspense>
</template>
