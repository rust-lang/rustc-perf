<script setup lang="ts">
import {CompareResponse} from "../types";
import {diffClass, percentClass} from "../shared";

const props = defineProps<{data: CompareResponse}>();

const totalsA = props.data.a.bootstrap_total / 1e9;
const totalsB = props.data.b.bootstrap_total / 1e9;

const bootstraps = Object.entries(props.data.a.bootstrap)
  .map((e) => {
    const name = e[0];

    const format = (datum) =>
      datum
        ? datum.toLocaleString("en-US", {
            minimumFractionDigits: 3,
            maximumFractionDigits: 3,
          })
        : "";
    const a = e[1] / 1e9;
    const b = props.data.b.bootstrap[name] / 1e9;
    return {
      name,
      a: format(a),
      b: format(b),
      percent: (100 * (b - a)) / a,
    };
  })
  .sort((a, b) => {
    let bp = Math.abs(b.percent);
    if (Number.isNaN(bp)) {
      bp = 0;
    }
    let ap = Math.abs(a.percent);
    if (Number.isNaN(ap)) {
      ap = 0;
    }
    if (bp < ap) {
      return -1;
    } else if (bp > ap) {
      return 1;
    } else {
      return a.name.localeCompare(b.name);
    }
  });
</script>

<template>
  <div class="category-title">Bootstrap timings</div>
  <table
    class="bootstrap"
    style="margin: auto"
    v-if="Object.keys(props.data.a.bootstrap).length > 0"
  >
    <tr>
      <td colspan="4">
        Values in seconds. Variance is 1-3% on smaller crates!
      </td>
    </tr>
    <tr>
      <th>total</th>
      <th v-if="totalsA">A: {{ totalsA.toFixed(3) }}</th>
      <th v-if="totalsB">B: {{ totalsB.toFixed(3) }}</th>
      <th v-if="totalsA && totalsB">
        Total: {{ (totalsB - totalsA).toFixed(1) }}
        <div v-bind:class="diffClass(totalsB - totalsA)">
          ({{ (((totalsB - totalsA) / totalsA) * 100).toFixed(3) }}%)
        </div>
      </th>
    </tr>
    <template v-for="bootstrap in bootstraps">
      <tr>
        <th style="text-align: right; width: 19em">{{ bootstrap.name }}</th>
        <td v-if="bootstrap.a">{{ bootstrap.a }}</td>
        <td v-if="bootstrap.b">{{ bootstrap.b }}</td>
        <td>
          <span
            v-if="bootstrap.percent"
            v-bind:class="percentClass(bootstrap.percent)"
            >{{ bootstrap.percent.toFixed(1) }}%</span
          >
        </td>
      </tr>
    </template>
  </table>
</template>

<style scoped lang="scss">
.bootstrap {
  table-layout: fixed;

  td,
  th {
    text-align: center;
    padding: 0.3em;
  }
}
</style>
