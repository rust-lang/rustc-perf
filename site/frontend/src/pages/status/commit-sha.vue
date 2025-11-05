<script setup lang="ts">
import {computed} from "vue";
import {tagLooksLikeSha} from "./data";

const props = defineProps<{
  tag: string;
  truncate?: boolean;
}>();

const looksLikeSha = computed(() => tagLooksLikeSha(props.tag));
</script>

<template>
  <a
    v-if="looksLikeSha"
    :title="tag"
    :href="'https://github.com/rust-lang/rust/commit/' + tag"
  >
    {{ truncate ? tag.substring(0, 13) : tag }}
  </a>
  <template v-else>
    {{ tag }}
  </template>
</template>
