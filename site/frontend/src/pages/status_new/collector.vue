<script setup lang="ts">
import {CollectorConfigAndWork, CollectorConfig} from "./data";

const props = defineProps<{
  collector: CollectorConfigAndWork;
}>();

/* trick typescript into thinking props is used. We _have_ to define it else
 * `collector` in the below code is untyped and thus fails to compile */
<any>props;

function statusClass(c: CollectorConfig): string {
  return c.isActive ? "active" : "inactive";
}
</script>

<template>
  <div class="collector-card">
    <div class="header">
      <div class="collector-name">
        <span>
          <strong class="collector-sm-padding-right">{{
            collector.config.name
          }}</strong>
          <span
            class="collector-sm-padding-left-right collector-left-divider"
            >{{ collector.config.target }}</span
          >
          <span
            class="collector-sm-padding-left-right status"
            :class="statusClass(collector.config)"
          >
            {{ collector.config.isActive ? "Active" : "Inactive" }}
          </span>
        </span>
      </div>
    </div>

    <div class="meta">
      <div class="collector-meta">
        <span class="collector-meta-name">
          <strong>Benchmark Set:</strong>
        </span>
        <span> #{{ collector.config.benchmarkSet }}</span>
      </div>

      <div class="collector-meta">
        <span class="collector-meta-name">
          <strong>Last Heartbeat:</strong>
        </span>
        <span>{{ collector.config.lastHeartbeatAt }}</span>
      </div>

      <div class="collector-meta">
        <span class="collector-meta-name">
          <strong>Date Added:</strong>
        </span>
        <span>{{ collector.config.dateAdded }}</span>
      </div>
    </div>

    <div v-if="collector.request !== null">
      <div class="table-collector-wrapper">
        <table class="table-collector">
          <caption>
            current benchmark request
          </caption>
          <thead>
            <tr class="table-header-row">
              <th class="table-header-padding">Type</th>
              <th class="table-header-padding">Created At</th>
              <th class="table-header-padding">Sha / Tag</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td class="table-cell-padding">{{ collector.request.type }}</td>
              <td class="table-cell-padding">
                {{ collector.request.createdAt }}
              </td>
              <td class="table-cell-padding">{{ collector.request.tag }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="table-collector-wrapper">
        <table class="table-collector" style="border-collapse: collapse">
          <caption>
            current benchmark jobs
          </caption>
          <thead>
            <tr class="table-header-row">
              <th class="table-header-padding">State</th>
              <th class="table-header-padding">Started At</th>
              <th class="table-header-padding">Backend</th>
              <th class="table-header-padding">Profile</th>
              <th class="table-header-padding">Dequeue Counter</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="j in collector.jobs">
              <td class="table-cell-padding">{{ j.state }}</td>
              <td class="table-cell-padding">{{ j.startedAt }}</td>
              <td class="table-cell-padding">{{ j.backend }}</td>
              <td class="table-cell-padding">{{ j.profile }}</td>
              <td class="table-cell-padding">{{ j.dequeCounter }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div class="collector-no-work" v-if="collector.request === null">
      <h3>no active benchmarks 🦦</h3>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.collector-card {
  border-radius: 8px;
  flex-direction: column;
  justify-content: space-between;
  padding: 16px;
  display: flex;
  box-shadow: 0 1px 2px #0006;
  margin: 0px 8px 8px 0px;
}
.collector-name {
  font-size: 1.5em;
  padding: 8px;
}

.meta {
  padding: 8px;
}

.collector-meta {
  display: flex;
}

.collector-meta-name {
  display: block;
  min-width: 125px;
}

.collector-left-divider {
  border-left: 2px solid black;
}

.collector-right-divider {
  border-right: 2px solid black;
}

.collector-sm-padding-left-right {
  padding: 0px 8px;
}
.collector-sm-padding-left {
  padding-left: 8px;
}
.collector-sm-padding-right {
  padding-right: 8px;
}

.status {
}

.status.active {
  background: #117411;
  color: white;
  font-weight: bold;
}
.status.inactive {
  background: red;
  color: white;
  font-weight: bold;
}

.table-collector-wrapper {
  padding: 8px;
  background-color: #eee;
  margin: 8px 0px;
  border-radius: 8px;

  table {
    border-collapse: collapse;
    width: 100%;
  }
}
.table-collector {
  caption {
    caption-side: top;
    text-align: left;
    font-variant: small-caps;
    font-weight: bold;
    font-size: 1.5em;
  }

  .table-header-row {
    border-bottom: 1px solid black;
  }

  .table-header-padding {
    padding: 8px 8px 0px 0px;
    text-align: left;
  }

  .table-cell-padding {
    padding: 8px 8px 1px 0px;
    text-align: left;
  }
}

.collector-no-work {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 40px;
  background-color: #eee;
  margin: 8px;
  padding: 8px;
  border-radius: 8px;

  h3 {
    font-variant: small-caps;
    font-weight: 700;
    font-size: 1.5em;
  }
}
</style>
