<script setup lang="ts">
import {getJson} from "../../utils/requests";
import {STATUS_DATA_NEW_URL} from "../../urls";
import {withLoading} from "../../utils/loading";
import {ref, Ref} from "vue";
import {StatusResponse, CollectorConfig} from "./data";

async function loadStatusNew(loading: Ref<boolean>) {
  dataNew.value = await withLoading(loading, () =>
    getJson<StatusResponse>(STATUS_DATA_NEW_URL)
  );
}

const loading = ref(true);
const dataNew: Ref<StatusResponse | null> = ref(null);

function statusClass(c: CollectorConfig): string {
  return c.is_active ? "active" : "inactive";
}

loadStatusNew(loading);
</script>

<template>
  <div v-if="dataNew !== null">
    <span>
      <h2>JSON from the database</h2>
      <code style="white-space: break-spaces">
        {{ JSON.stringify(dataNew, null, 2) }}
      </code>
    </span>
    <div id="app" class="container">
      <h1>Collectors</h1>

      <div class="grid">
        <div
          v-for="c in dataNew.collector_configs"
          :key="c.name + c.target"
          class="card"
        >
          <div>
            <div class="header">
              <div class="name">{{ c.name }}:</div>
              <div class="status" :class="statusClass(c)">
                {{ c.is_active ? "Active" : "Inactive" }}
              </div>
            </div>
            <div class="meta">
              <div><strong>Target:</strong> {{ c.target }}</div>
              <div><strong>Benchmark Set:</strong> #{{ c.benchmark_set }}</div>
              <div>
                <strong>Last Heartbeat:</strong>
                {{ c.last_heartbeat_at }}
              </div>
              <div>
                <strong>Date Added:</strong>
                {{ c.date_added }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.timeline {
  max-width: 100%;
  width: fit-content;

  table {
    border-collapse: collapse;
    font-size: 1.1em;

    th,
    td {
      padding: 0.2em;
    }

    th {
      text-align: center;
    }
    td {
      text-align: left;
      padding: 0 0.5em;

      &.centered {
        text-align: center;
      }
      &.right-align {
        text-align: right;
      }
    }
    tr.active {
      font-weight: bold;
    }
  }

  @media screen and (min-width: 1440px) {
    width: 100%;
  }
}

.header {
  display: flex;
  align-items: center;
}

.status {
  padding: 2px 8px;
  border-radius: 20px;
  font-size: 0.75rem;
  width: 50px;
  font-weight: bold;
}

.status.active {
  color: green;
}
.status.inactive {
  color: red;
}

.wrapper {
  display: grid;
  column-gap: 100px;
  grid-template-columns: 1fr;

  @media screen and (min-width: 1440px) {
    grid-template-columns: 4fr 6fr;
  }
}
.current {
  max-width: 100%;
  width: fit-content;

  .benchmark {
    margin-bottom: 10px;
    font-size: 1.2em;
  }
}
.column-centered {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.current-table {
  border-collapse: collapse;
  font-size: 1.1em;

  td,
  th {
    padding: 0 10px;
  }
  tbody > tr {
    td {
      padding-top: 5px;
      text-align: center;
    }
  }
}
.aligned {
  text-align: right;
}
.error {
  padding: 10px;
  background-color: #f7f7f7;
  max-width: 100%;
  white-space: pre-wrap;
  word-break: break-word;
}
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}
.card {
  border-radius: 12px;
  padding: 16px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  transition: transform 0.2s ease;
}
</style>
