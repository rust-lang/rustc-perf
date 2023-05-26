<script setup lang="ts">
import {onMounted, ref} from "vue";

export interface SelectionParams {
  start: string;
  end: string;
}

const props = defineProps<SelectionParams>();

const emit = defineEmits<{
  (e: "change", params: SelectionParams): void;
}>();

const startRef = ref<HTMLInputElement | null>(null);
const endRef = ref<HTMLInputElement | null>(null);

onMounted(() => {
  startRef.value.value = props.start;
  endRef.value.value = props.end;
});

function submitSettings() {
  const start = startRef.value.value;
  const end = endRef.value.value;

  const params = {start, end};
  emit("change", params);
}
</script>

<template>
  <div id="settings">
    start: <input placeholder="yyyy-mm-dd or commit" ref="startRef" /> end:
    <input placeholder="yyyy-mm-dd or commit" ref="endRef" />&nbsp;<a
      href="#"
      @click.prevent="submitSettings"
      >Submit</a
    >
  </div>
</template>
