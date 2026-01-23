import {createApp} from "vue";
import Dashboard from "./dashboard/page.vue";
import WithSuspense from "../components/with-suspense.vue";

const app = createApp(WithSuspense, {
  component: Dashboard,
});
app.mount("#app");
