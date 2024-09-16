import Compare from "./compare/page.vue";
import {createApp, ref} from "vue";
import WithSuspense from "../components/with-suspense.vue";
import {checkIsEmbeddedPerfettoEnabled} from "../perfetto";

const isEmbeddedPerfettoEnabled = ref(false);
checkIsEmbeddedPerfettoEnabled().then((res) => {
  isEmbeddedPerfettoEnabled.value = res;
});

const app = createApp(WithSuspense, {
  component: Compare,
});
app.provide("isEmbeddedPerfettoEnabled", isEmbeddedPerfettoEnabled);
app.mount("#app");
