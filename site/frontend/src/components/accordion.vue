<script lang="ts">
import {defineComponent} from "vue";
import {useExpandedStore} from "./expansion";

export default defineComponent({
  props: {
    id: {
      type: String,
      required: true,
    },
  },
  setup() {
    const {toggleExpanded, isExpanded} = useExpandedStore();
    return {
      toggleExpanded,
      isExpanded,
    };
  },
});
</script>

<template>
  <tr
    @click="toggleExpanded(id)"
    :class="{toggle: true, toggled: isExpanded(id)}"
  >
    <td class="toggle-arrow">
      {{ isExpanded(id) ? "▼" : "▶" }}
    </td>
    <slot name="default" />
  </tr>
  <tr v-if="isExpanded(id)">
    <slot name="collapsed" />
  </tr>
</template>

<style lang="scss" scoped>
.toggle {
  cursor: pointer;

  .toggle-arrow {
    padding-right: 5px;
  }

  &:hover,
  &.toggled {
    background-color: #d6d3d35c;
  }
}
</style>
