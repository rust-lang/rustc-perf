<script setup lang="ts">
import {onMounted, ref} from "vue";
import {GraphKind} from "../../graph/data";
import {BenchmarkInfo} from "../../api";

export interface SelectionParams {
  start: string;
  end: string;
  kind: GraphKind;
  stat: string;
}

interface Props extends SelectionParams {
  info: BenchmarkInfo;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  (e: "change", params: SelectionParams): void;
}>();

const startRef = ref<HTMLInputElement | null>(null);
const endRef = ref<HTMLInputElement | null>(null);
const kindRef = ref<HTMLSelectElement | null>(null);
const statRef = ref<HTMLSelectElement | null>(null);

onMounted(() => {
  startRef.value.value = props.start;
  endRef.value.value = props.end;
  kindRef.value.value = props.kind;
  statRef.value.value = props.stat;
});

function submitSettings() {
  const start = startRef.value.value;
  const end = endRef.value.value;
  const kind = kindRef.value.value as GraphKind;
  const stat = statRef.value.value;

  const params = {start, end, kind, stat};
  emit("change", params);
}
</script>

<template>
  <div id="settings">
    start: <input placeholder="yyyy-mm-dd or commit" ref="startRef" /> end:
    <input placeholder="yyyy-mm-dd or commit" ref="endRef" /> Graph kind:
    <select ref="kindRef">
      <option value="raw">Raw</option>
      <option value="percentfromfirst">Percent Delta from First</option>
      <option value="percentrelative">Percent Delta from Previous</option>
    </select>
    <select ref="statRef">
      <option v-for="value in info.compile_metrics" :value="value">
        {{ value }}
      </option></select
    >&nbsp;<a href="#" @click.prevent="submitSettings">Submit</a>
  </div>
</template>
