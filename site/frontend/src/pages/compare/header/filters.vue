<script setup lang="ts">
import Toggle from "../toggle.vue";
import {DataFilter} from "../types";
import Tooltip from "../tooltip.vue";
import {ref, toRaw, watch} from "vue";
import {deepCopy} from "../../../utils/copy";

const props = defineProps<{
  // When reset, set filter to this value
  defaultFilter: DataFilter;
  // Initialize the filter with this value
  initialFilter: DataFilter;
}>();
const emit = defineEmits<{
  (e: "change", filter: DataFilter): void;
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

function updateOpened(opened: boolean) {
  filterOpened.value = opened;
}

const filterOpened = ref(false);
</script>

<template>
  <fieldset class="collapsible-section">
    <Toggle :opened="filterOpened" @change="updateOpened">
      <template #label>Filters</template>
      <template #content>
        <div>
          <div class="section">
            <div class="section-heading">Filter</div>
            <input id="filter" type="text" v-model="filter.name" />
          </div>
          <div class="section section-list-wrapper">
            <div class="section-heading">
              <div style="width: 160px">
                <span>Profiles</span>
                <Tooltip
                  >The different compilation profiles (check, debug, opt, doc).
                </Tooltip>
              </div>
            </div>
            <ul class="states-list">
              <li>
                <label>
                  <input
                    type="checkbox"
                    id="profile-check"
                    v-model="filter.profile.check"
                  />
                  <span class="cache-label">check</span>
                </label>
                <Tooltip>Check build that does not generate any code.</Tooltip>
              </li>
              <li>
                <label>
                  <input
                    type="checkbox"
                    id="profile-debug"
                    v-model="filter.profile.debug"
                  />
                  <span class="cache-label">debug</span>
                </label>
                <Tooltip>Debug build that produces unoptimized code.</Tooltip>
              </li>
              <li>
                <label>
                  <input
                    type="checkbox"
                    id="profile-opt"
                    v-model="filter.profile.opt"
                  />
                  <span class="cache-label">opt</span>
                </label>
                <Tooltip
                  >Release build that produces as optimized code as possible.
                </Tooltip>
              </li>
              <li>
                <label>
                  <input
                    type="checkbox"
                    id="profile-doc"
                    v-model="filter.profile.doc"
                  />
                  <span class="cache-label">doc</span>
                </label>
                <Tooltip
                  >Documentation build that produces HTML documentation site
                  produced by `rustdoc`.
                </Tooltip>
              </li>
            </ul>
          </div>
          <div class="section section-list-wrapper">
            <div class="section-heading">
              <div style="width: 160px">
                <span>Scenarios</span>
                <Tooltip
                  >The different scenarios based on their incremental
                  compilation cache state.
                </Tooltip>
              </div>
            </div>
            <ul class="states-list">
              <li>
                <label>
                  <input
                    type="checkbox"
                    id="build-full"
                    v-model="filter.scenario.full"
                  />
                  <span class="cache-label">full</span>
                </label>
                <Tooltip
                  >A non-incremental full build starting with empty cache.
                </Tooltip>
              </li>
              <li>
                <label>
                  <input
                    type="checkbox"
                    id="build-incremental-full"
                    v-model="filter.scenario.incrFull"
                  />
                  <span class="cache-label">incr-full</span>
                </label>
                <Tooltip
                  >An incremental build starting with empty cache.
                </Tooltip>
              </li>
              <li>
                <label>
                  <input
                    type="checkbox"
                    id="build-incremental-unchanged"
                    v-model="filter.scenario.incrUnchanged"
                  />
                  <span class="cache-label">incr-unchanged</span>
                </label>
                <Tooltip
                  >An incremental build starting with complete cache, and
                  unchanged source directory -- the "perfect" scenario for
                  incremental.
                </Tooltip>
              </li>
              <li>
                <label>
                  <input
                    type="checkbox"
                    id="build-incremental-patched"
                    v-model="filter.scenario.incrPatched"
                  />
                  <span class="cache-label">incr-patched</span>
                </label>
                <Tooltip
                  >An incremental build starting with complete cache, and an
                  altered source directory. The typical variant of this is
                  "println" which represents the addition of a `println!` macro
                  somewhere in the source code.
                </Tooltip>
              </li>
            </ul>
          </div>
          <div class="section section-list-wrapper">
            <div class="section-heading">
              <div style="width: 160px">
                <span>Categories</span>
                <Tooltip
                  >Select benchmarks based on their category (primary or
                  secondary).
                </Tooltip>
              </div>
            </div>
            <ul class="states-list">
              <li>
                <label>
                  <input type="checkbox" v-model="filter.category.primary" />
                  <span class="cache-label">primary</span>
                </label>
                <Tooltip>Real-world benchmarks.</Tooltip>
              </li>
              <li>
                <label>
                  <input type="checkbox" v-model="filter.category.secondary" />
                  <span class="cache-label">secondary</span>
                </label>
                <Tooltip>Artificial benchmarks and stress-tests.</Tooltip>
              </li>
            </ul>
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
          <button
            @click="emit('export')"
            title="Download the currently filtered data as a Markdown table"
          >
            Export to Markdown
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
