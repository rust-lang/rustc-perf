<script setup lang="ts">
import {BenchmarkInfo} from "../../../api";
import {onMounted, ref} from "vue";
import Toggle from "../toggle.vue";

export interface SelectionParams {
  start: string;
  end: string;
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
const statRef = ref<HTMLSelectElement | null>(null);

onMounted(() => {
  startRef.value.value = props.start;
  endRef.value.value = props.end;
  statRef.value.value = props.stat;
});

function submitSettings() {
  const start = startRef.value.value;
  const end = endRef.value.value;
  const stat = statRef.value.value;

  const params = {start, end, stat};
  emit("change", params);
}

const opened = ref(false);
</script>

<template>
  <fieldset class="settings">
    <Toggle :opened="opened" @change="(value) => (opened = value)">
      <template #label>Do another comparison</template>
      <template #content>
        <div class="commits section">
          <div class="section-heading">Commits</div>
          <div
            style="display: flex; width: 100%; justify-content: space-around"
          >
            <div class="commit-input">
              <label for="start-bound">Before</label>
              <input
                width="100em"
                placeholder="YYYY-MM-DD or Commit SHA"
                ref="startRef"
              />
            </div>
            <div class="commit-input">
              <label for="end-bound">After</label>
              <input
                width="100em"
                placeholder="YYYY-MM-DD or Commit SHA"
                ref="endRef"
              />
            </div>
          </div>
        </div>
        <div class="metric section">
          <div class="section-heading">Metric</div>
          <div
            style="
              display: flex;
              flex-direction: column;
              justify-content: center;
            "
          >
            <select class="stats" ref="statRef">
              <option
                v-for="value in props.info.compile_metrics"
                :value="value"
              >
                {{ value }}
              </option>
            </select>
          </div>
        </div>
        <input type="submit" value="Submit" @click.prevent="submitSettings" />
      </template>
    </Toggle>
  </fieldset>
</template>

<style scoped lang="scss">
.settings input[type="submit"] {
  width: 100%;
  font-weight: bold;
  background: #add8e6;
}
.commits {
  border: none;
  display: flex;
  padding: 0;
}
.commit-input {
  width: 270px;
  display: flex;
  flex-direction: column;
}
.commit-input label {
  font-size: 12px;
  font-weight: bold;
  margin-bottom: 6px;
}
.metric {
  position: relative;
  height: 40px;
}
.stats {
  border-radius: 5px;
  width: 200px;
  font-size: 14px;
  margin-left: 52px;
}
</style>
