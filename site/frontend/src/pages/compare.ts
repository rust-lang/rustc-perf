import Compare from "./compare/page.vue";
import {createApp} from "vue";
import WithSuspense from "../components/with-suspense.vue";

const app = createApp(WithSuspense, {
  component: Compare,
});
app.mount("#app");
