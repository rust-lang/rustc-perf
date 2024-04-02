<script setup lang="ts">
const props = defineProps<{
  tooltip: string;
  title: string;
  selected: boolean;
}>();
</script>

<template>
  <div class="tab" :title="props.tooltip" :class="{selected: props.selected}">
    <div class="title">{{ props.title }}</div>
    <div class="summary">
      <slot name="summary"></slot>
    </div>
  </div>
</template>

<style scoped lang="scss">
.tab {
  display: flex;
  flex-direction: column;
  position: relative;
  min-width: 200px;
  min-height: 60px;
  padding: 5px;
  text-align: center;
  border: 2px dotted #cccccc;
  border-radius: 5px;
  cursor: pointer;

  &:hover {
    @extend .selected;
  }

  @media (min-width: 600px) {
    &:not(:first-child) {
      margin-left: 30px;
    }

    &:not(:last-child) {
      :after {
        content: "";
        position: absolute;
        right: -17px;
        border-right: 1px solid black;
        top: 20%;
        bottom: 20%;
      }
    }
  }
}

.title {
  font-weight: bold;
  font-size: 1.1em;
  margin-bottom: 5px;
}

.selected {
  border-style: solid;
  border-color: black;
}

.summary {
  display: flex;
  flex-direction: column;
  justify-content: center;
  font-size: 0.9em;
  flex-grow: 1;
}
</style>
