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

const maxTotalWidth = computed(() => {
  return Math.max(beforeTotalWidth.value, afterTotalWidth.value);
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

function getSectionByName(
  sections: CompilationSections,
  name: string
): CompilationSection | null {
  const values = sections.sections.filter((s) => s.name === name);
  if (values.length === 0) {
    return null;
  }
  return values[0];
}

function formatPercent(
  sections: CompilationSections,
  sectionName: string
): string {
  const section = getSectionByName(sections, sectionName);
  if (section === null) return "??";
  const value = section.value;
  const total = calculateTotalSectionsDuration(sections);
  const percent = (value / total) * 100;
  return `${percent.toFixed(2)}%`;
}

function getRowWidth(): number {
  return maxTotalWidth.value;
}

const chartRows: ComputedRef<Array<[string, CompilationSections]>> = computed(
  () => [
    ["Before", props.before],
    ["After", props.after],
  ]
);
const legendItems: ComputedRef<
  Array<{
    section: CompilationSection;
    color: string;
    beforePercent: string;
    beforeAbsolute: string;
    afterPercent: string;
    afterAbsolute: string;
  }>
> = computed(() => {
  const items = [];
  for (const section of props.before.sections) {
    items.push({
      section,
      beforePercent: formatPercent(props.before, section.name),
      beforeAbsolute:
        getSectionByName(props.before, section.name)?.value?.toLocaleString() ??
        "??",
      afterPercent: formatPercent(props.after, section.name),
      afterAbsolute:
        getSectionByName(props.after, section.name)?.value?.toLocaleString() ??
        "??",
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
  <div>
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
                width: calculate_width(section.value, getRowWidth()),
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
                  <div>
                    {{ formatPercent(props.before, section.name) }} ->
                    {{ formatPercent(props.after, section.name) }}
                  </div>
                  <div>
                    {{
                      getSectionByName(
                        props.before,
                        section.name
                      )?.value?.toLocaleString() ?? "??"
                    }}
                    ->
                    {{
                      getSectionByName(
                        props.after,
                        section.name
                      )?.value.toLocaleString() ?? "??"
                    }}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="legend">
        <table>
          <thead>
            <tr>
              <th></th>
              <th>Section</th>
              <th>Relative change</th>
              <th>Absolute change</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="item in legendItems"
              @mouseenter="activate(item.section.name)"
              @mouseleave="deactivate"
              :class="{active: activeSection === item.section.name}"
            >
              <td>
                <div class="color" :style="{backgroundColor: item.color}"></div>
              </td>
              <td class="name">{{ item.section.name }}</td>
              <td>{{ item.beforePercent }} -> {{ item.afterPercent }}</td>
              <td>{{ item.beforeAbsolute }} -> {{ item.afterAbsolute }}</td>
            </tr>
          </tbody>
        </table>
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
  width: 500px;

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

.section:first-child {
  border-radius: 5px 0 0 5px;
}

.section:last-child {
  border-radius: 0 5px 5px 0;
}
.legend {
  margin-left: 40px;

  table {
    td,
    th {
      padding: 5px;
    }
  }
  .color {
    width: 15px;
    height: 15px;
  }
  .active {
    font-weight: bold;
  }
  .name {
    margin-left: 5px;
  }
}

.active .color,
.active.section {
  box-shadow: inset 0 0 1px 2px #000;
}
</style>
