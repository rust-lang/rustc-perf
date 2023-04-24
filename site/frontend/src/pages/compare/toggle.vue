<script setup lang="ts">
import {ref} from "vue";

let opened = ref(false);
</script>

<template>
  <legend class="toggle section-heading" @click="opened = !opened">
    <slot name="label"></slot>
    <span>{{opened ? ' ▼' : ' ▶'}}</span>
  </legend>
  <div v-show="opened">
    <slot name="content"></slot>
  </div>
</template>

<style scoped>
.toggle {
    cursor: pointer;
}
</style>
