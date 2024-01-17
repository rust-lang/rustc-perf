<script setup lang="ts">
import {ArtifactDescription} from "../types";
import {diffClass, formatPercentChange, formatSize} from "../shared";
import Tooltip from "../tooltip.vue";

const props = defineProps<{
  a: ArtifactDescription;
  b: ArtifactDescription;
}>();

interface ComponentSize {
  name: string;
  before: number;
  after: number;
}

const allComponents: ComponentSize[] = [
  ...new Set([
    ...Object.keys(props.a.component_sizes),
    ...Object.keys(props.b.component_sizes),
  ]),
].map((name) => {
  const before = props.a.component_sizes[name] ?? 0;
  const after = props.b.component_sizes[name] ?? 0;
  return {
    name,
    before,
    after,
  };
});

// Sort binaries first, libraries later. Then within each category, sort alphabetically.
const components = allComponents.sort((a, b) => {
  const aLib = isLibrary(a.name);
  const bLib = isLibrary(b.name);
  if (aLib && !bLib) {
    return 1;
  } else if (!aLib && bLib) {
    return -1;
  } else {
    return a.name.localeCompare(b.name);
  }
});

function isLibrary(component: string): boolean {
  return component.startsWith("lib");
}

function formatName(component: string): string {
  if (isLibrary(component)) {
    return `${component}.so`;
  }
  return component;
}

function formatValue(value: number): string {
  if (value === 0) {
    return "-";
  }
  return formatSize(value);
}

function formatChangeTitle(a: number, b: number): string {
  return (b - a).toLocaleString();
}

function formatTitle(value: number): string {
  if (value === 0) {
    return "Missing value";
  }
  return value.toLocaleString();
}

function formatChange(a: number, b: number): string {
  const diff = b - a;
  const formatted = formatSize(Math.abs(diff));
  if (diff < 0) {
    return `-${formatted}`;
  }
  return formatted;
}

function getClass(a: number, b: number): string {
  if (a === b) {
    return "";
  }
  return diffClass(b - a);
}

function generateTitle(component: string): string {
  if (component === "rustc") {
    return `Executable of the Rust compiler. A small wrapper that links to librustc_driver.so, which provides most of the compiler logic.`;
  } else if (component === "rustdoc") {
    return `Executable of rustdoc. Links to librustc_driver.so, which provides most of the compiler logic.`;
  } else if (component === "cargo") {
    return "Executable of cargo.";
  } else if (component === "librustc_driver") {
    return `Shared library containing the core implementation of the compiler. It is used by several other tools and binaries.`;
  } else if (component === "libLLVM") {
    return `Shared library of the LLVM codegen backend. It is used by librustc_driver.so.`;
  } else if (component === "libstd") {
    return `Shared library containing the Rust standard library. It is used by librustc_driver.so.`;
  } else if (component === "libtest") {
    return `Shared library containing the Rust test harness.`;
  } else {
    return ""; // Unknown component
  }
}
</script>

<template>
  <div class="category-title">Artifact component sizes</div>
  <div class="wrapper">
    <table>
      <thead>
        <tr>
          <th>Component</th>
          <th>Kind</th>
          <th class="aligned-header">Before</th>
          <th class="aligned-header">After</th>
          <th class="aligned-header">Change</th>
          <th class="aligned-header">% Change</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="component in components">
          <td class="component">
            {{ formatName(component.name) }}
            <Tooltip>{{ generateTitle(component.name) }}</Tooltip>
          </td>
          <td>
            {{ isLibrary(component.name) ? "Library" : "Binary" }}
          </td>
          <td>
            <div class="aligned" :title="formatTitle(component.before)">
              {{ formatValue(component.before) }}
            </div>
          </td>
          <td>
            <div class="aligned" :title="formatTitle(component.after)">
              {{ formatValue(component.after) }}
            </div>
          </td>
          <td :class="getClass(component.before, component.after)">
            <div
              class="aligned"
              :title="formatChangeTitle(component.before, component.after)"
            >
              {{ formatChange(component.before, component.after) }}
            </div>
          </td>
          <td :class="getClass(component.before, component.after)">
            <div class="aligned">
              {{ formatPercentChange(component.before, component.after) }}
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped lang="scss">
.wrapper {
  display: flex;
  justify-content: center;
}
table {
  table-layout: fixed;
  margin-top: 10px;

  td,
  th {
    text-align: center;
    padding: 0.3em;
  }

  .component {
    word-break: break-word;
  }

  .aligned {
    text-align: right;

    @media (min-width: 600px) {
      width: 120px;
    }
  }
  .aligned-header {
    text-align: right;
  }
}
</style>
