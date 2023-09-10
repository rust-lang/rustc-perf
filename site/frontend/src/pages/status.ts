import Status from "./status/page.vue";
import {createApp} from "vue";
import WithSuspense from "../components/with-suspense.vue";

const app = createApp(WithSuspense, {
  component: Status,
});
app.mount("#app");
