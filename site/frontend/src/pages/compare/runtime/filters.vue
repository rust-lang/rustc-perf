<script setup lang="ts">
import Toggle from "../toggle.vue";
import Tooltip from "../tooltip.vue";
import {ref, toRaw, watch} from "vue";
import {deepCopy} from "../../../utils/copy";
import {PREF_FILTERS_OPENED} from "../prefs";
import {createPersistedRef} from "../../../storage";
import {RuntimeBenchmarkFilter} from "./common";

const props = defineProps<{
  // When reset, set filter to this value
  defaultFilter: RuntimeBenchmarkFilter;
  // Initialize the filter with this value
  initialFilter: RuntimeBenchmarkFilter;
}>();
const emit = defineEmits<{
  (e: "change", filter: RuntimeBenchmarkFilter): void;
  (e: "export"): void;
}>();

function reset() {
  // We must not change the default filter
  filter.value = deepCopy(props.defaultFilter);
}

let filter = ref(deepCopy(props.initialFilter));
watch(
  filter,
  (newValue, _) => {
    emit("change", toRaw(newValue));
  },
  {deep: true}
);

const opened = createPersistedRef(PREF_FILTERS_OPENED);
</script>

<template>
  <fieldset class="collapsible-section">
    <Toggle :opened="opened" @change="(value) => (opened = value)">
      <template #label>Filters</template>
      <template #content>
        <div>
          <div class="section">
            <div class="section-heading">Filter</div>
            <input id="filter" type="text" v-model="filter.name" />
          </div>
          <div class="section">
            <div class="section-heading">
              <span>Show non-relevant results</span>
              <Tooltip>
                Whether to show test case results that are not relevant (i.e.,
                not significant or have a large enough magnitude). You can see
                <a
                  href="https://github.com/rust-lang/rustc-perf/blob/master/docs/comparison-analysis.md#how-is-relevance-of-a-test-run-summary-determined"
                >
                  here</a
                >
                how relevance is calculated.
              </Tooltip>
            </div>
            <input
              type="checkbox"
              v-model="filter.nonRelevant"
              style="margin-left: 20px"
            />
          </div>
          <div class="section">
            <div class="section-heading">
              <span>Display raw data</span>
              <Tooltip>Whether to display or not raw data columns.</Tooltip>
            </div>
            <input
              type="checkbox"
              v-model="filter.showRawData"
              style="margin-left: 20px"
            />
          </div>
          <button @click="reset" style="margin-right: 10px">
            Reset filters
          </button>
        </div>
      </template>
    </Toggle>
  </fieldset>
</template>

<style scoped lang="scss">
.section-heading {
  font-size: 16px;
}

#filter {
  margin-left: 52px;
}

.states-list {
  display: flex;
  flex-direction: column;
  align-items: start;
  list-style: none;
  margin: 0;
  padding: 0;
}

.section-list-wrapper {
  flex-direction: column;
}

@media (min-width: 760px) {
  .states-list {
    justify-content: start;
    flex-direction: row;
    align-items: center;
    width: 80%;
  }

  .section-list-wrapper {
    flex-direction: row;
  }
}

.states-list > li {
  margin-right: 15px;
}

.cache-label {
  font-weight: bold;
}
</style>
