<script setup lang="ts">
import {ref} from "vue";

const props = withDefaults(
  defineProps<{
    defaultOpened?: boolean;
  }>(),
  {
    defaultOpened: false,
  }
);

const opened = ref(props.defaultOpened);
</script>

<template>
  <legend class="toggle section-heading" @click="opened = !opened">
    <slot name="label"></slot>
    <span>{{ opened ? " ▼" : " ▶" }}</span>
  </legend>
  <div v-show="opened">
    <slot name="content"></slot>
  </div>
</template>

<style scoped lang="scss">
.toggle {
  cursor: pointer;
}
</style>
