import Graphs from "./graphs/page.vue";
import {createApp} from "vue";
import WithSuspense from "../components/with-suspense.vue";

const app = createApp(WithSuspense, {
  component: Graphs,
});
app.mount("#app");
