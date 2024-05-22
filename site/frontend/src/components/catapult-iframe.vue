<script setup lang="ts">
import {ArtifactDescription} from "../pages/compare/types";
import {CompileTestCase} from "../pages/compare/compile/common";
import {catapultUrl} from "../self-profile";
import {ref} from "vue";

const props = defineProps<{
  testCase: CompileTestCase;
  artifact: ArtifactDescription;
}>();

const loading = ref(true);

const onIframeLoad = () => {
  loading.value = false;
};
</script>

<template>
  <div v-if="loading">Loading...</div>
  <iframe
    v-show="!loading"
    width="100%"
    style="aspect-ratio: 16/9"
    :src="catapultUrl(props.artifact, props.testCase)"
    @load="onIframeLoad"
  />
</template>
