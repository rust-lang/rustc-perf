<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    opened?: boolean;
  }>(),
  {
    opened: false,
  }
);

const emit = defineEmits<{
  (e: "change", opened: boolean): void;
}>();
</script>

<template>
  <legend class="toggle section-heading" @click="emit('change', !props.opened)">
    <slot name="label"></slot>
    <span>{{ props.opened ? " ▼" : " ▶" }}</span>
  </legend>
  <div v-show="props.opened">
    <slot name="content"></slot>
  </div>
</template>

<style scoped lang="scss">
.toggle {
  cursor: pointer;
}
</style>
