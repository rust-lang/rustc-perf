<script setup lang="ts">
import {CompilationSection, CompilationSections} from "./detail-resolver";
import {computed, ComputedRef, Ref, ref} from "vue";

const props = defineProps<{
  before: CompilationSections;
  after: CompilationSections;
}>();

function calculateTotalSectionsDuration(sections: CompilationSections): number {
  return sections.sections.reduce((accum, section) => accum + section.value, 0);
}

const beforeTotalWidth = computed(() => {
  return calculateTotalSectionsDuration(props.before);
});
const afterTotalWidth = computed(() => {
  return calculateTotalSectionsDuration(props.after);
});

const SECTIONS_PALETTE = [
  "#7768AE",
  "#FFCf96",
  "#ff8080",
  "#29adb2",
  "#3BB273",
];

function getSectionColor(index: number): string {
  return SECTIONS_PALETTE[index % SECTIONS_PALETTE.length];
}

function calculate_width(value: number, totalDuration: number): string {
  const fraction = value / totalDuration;
  return `${(fraction * 100).toFixed(2)}%`;
}

function formatPercent(
  sections: CompilationSections,
  sectionName: string
): string {
  const values = sections.sections.filter((s) => s.name === sectionName);
  if (values.length === 0) return "??";
  const value = values[0].value;
  const total = calculateTotalSectionsDuration(sections);
  const percent = (value / total) * 100;
  return `${percent.toFixed(2)}%`;
}

const chartRows: ComputedRef<Array<[string, CompilationSections]>> = computed(
  () => [
    ["Before", props.before],
    ["After", props.after],
  ]
);
const legendItems: ComputedRef<
  Array<{section: CompilationSection; label: string; color: string}>
> = computed(() => {
  const items = [];
  for (const section of props.before.sections) {
    items.push({
      section,
      label: `${section.name} (${formatPercent(
        props.before,
        section.name
      )} -> ${formatPercent(props.after, section.name)})`,
      color: getSectionColor(items.length),
    });
  }
  return items;
});

const activeSection: Ref<string | null> = ref(null);

function activate(section: string) {
  activeSection.value = section;
}
function deactivate() {
  activeSection.value = null;
}
</script>

<template>
  <div class="wrapper">
    <div class="chart-wrapper">
      <div class="chart" v-for="([label, sections], rowIndex) in chartRows">
        <span class="label">{{ label }}</span>
        <div class="section-wrapper">
          <div
            v-for="(section, index) in sections.sections"
            :class="{section: true, active: activeSection === section.name}"
            @mouseenter="activate(section.name)"
            @mouseleave="deactivate"
            :style="{
              width: calculate_width(
                section.value,
                rowIndex == 0 ? beforeTotalWidth : afterTotalWidth
              ),
              backgroundColor: getSectionColor(index),
            }"
          >
            <div
              class="description"
              v-if="rowIndex == 1 && activeSection === section.name"
            >
              <div>
                <b>{{ section.name }}</b>
              </div>
              <div>
                {{ formatPercent(props.before, section.name) }} ->
                {{ formatPercent(props.after, section.name) }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="legend">
      <div
        class="item"
        v-for="item in legendItems"
        @mouseenter="activate(item.section.name)"
        @mouseleave="deactivate"
      >
        <div
          :class="{color: true, active: activeSection === item.section.name}"
          :style="{backgroundColor: item.color}"
        ></div>
        <div class="name">{{ item.label }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.wrapper {
  display: flex;
  align-items: center;
}
.chart {
  display: flex;
  justify-content: flex-end;
  width: 600px;

  &:first-child {
    margin-bottom: 10px;
  }

  .label {
    width: 55px;
    margin-right: 5px;
    align-self: center;
  }

  .section-wrapper {
    width: calc(100% - 60px);
    display: flex;
    flex-direction: row;

    .section {
      height: 30px;
      position: relative;
    }
  }

  .description {
    position: absolute;
    top: 35px;
    width: max-content;
    z-index: 99;
    padding: 10px;
    background-color: white;
    border: 2px solid black;
  }
}

.active {
  box-shadow: inset 0 0 1px 2px #000;
}

.section:first-child {
  border-radius: 5px 0 0 5px;
}

.section:last-child {
  border-radius: 0 5px 5px 0;
}
.legend {
  margin-left: 40px;

  .item {
    display: flex;
    margin-bottom: 5px;

    .color {
      width: 15px;
      height: 15px;
    }
    .name {
      margin-left: 5px;
    }
  }
}
</style>
