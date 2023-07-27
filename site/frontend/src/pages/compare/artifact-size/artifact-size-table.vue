<script setup lang="ts">
import {ArtifactDescription} from "../types";
import {
  diffClass,
  formatPercentChange,
  formatSize,
  isValidValue,
} from "../shared";
import Tooltip from "../tooltip.vue";

const props = defineProps<{
  a: ArtifactDescription;
  b: ArtifactDescription;
}>();

// Sort binaries first, libraries later. Then within each category, sort alphabetically.
const components = Object.keys(props.a.component_sizes).sort((a, b) => {
  const aLib = a.startsWith("lib");
  const bLib = b.startsWith("lib");
  if (aLib && !bLib) {
    return 1;
  } else if (!aLib && bLib) {
    return -1;
  } else {
    return a.localeCompare(b);
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

function formatValue(value: number | undefined): string {
  if (!isValidValue(value)) {
    return "-";
  }
  return formatSize(value);
}

function formatChange(a: number | undefined, b: number | undefined): string {
  if (!isValidValue(a) || !isValidValue(b)) {
    return "-";
  }
  const diff = b - a;
  const formatted = formatSize(Math.abs(diff));
  if (diff < 0) {
    return `-${formatted}`;
  }
  return formatted;
}

function getClass(a: number | undefined, b: number | undefined): string {
  if (!isValidValue(a) || !isValidValue(b) || a == b) {
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
            {{ formatName(component) }}
            <Tooltip>{{ generateTitle(component) }}</Tooltip>
          </td>
          <td>
            {{ isLibrary(component) ? "Library" : "Binary" }}
          </td>
          <td>
            <div class="aligned">
              {{ formatValue(a.component_sizes[component]) }}
            </div>
          </td>
          <td>
            <div class="aligned">
              {{ formatValue(b.component_sizes[component]) }}
            </div>
          </td>
          <td
            :class="
              getClass(
                a.component_sizes[component],
                b.component_sizes[component]
              )
            "
          >
            <div class="aligned">
              {{
                formatChange(
                  a.component_sizes[component],
                  b.component_sizes[component]
                )
              }}
            </div>
          </td>
          <td
            :class="
              getClass(
                a.component_sizes[component],
                b.component_sizes[component]
              )
            "
          >
            <div class="aligned">
              {{
                formatPercentChange(
                  a.component_sizes[component],
                  b.component_sizes[component]
                )
              }}
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
