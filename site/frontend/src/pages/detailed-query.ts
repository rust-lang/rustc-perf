import {createApp} from "vue";
import WithSuspense from "../components/with-suspense.vue";
import DetailedQuery from "./detailed-query/page.vue";

const app = createApp(WithSuspense, {
  component: DetailedQuery,
});
app.mount("#app");
