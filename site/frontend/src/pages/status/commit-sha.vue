<script setup lang="ts">
import {computed} from "vue";

const props = defineProps<{
  tag: string;
}>();

const looksLikeSha = computed(() => props.tag.length === 40);
</script>

<template>
  <a
    v-if="looksLikeSha"
    :title="tag"
    :href="'https://github.com/rust-lang/rust/commit/' + tag"
  >
    {{ tag.substring(0, 13) }}
  </a>
  <template v-else>
    {{ tag }}
  </template>
</template>
