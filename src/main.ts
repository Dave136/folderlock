import { createApp } from "vue";
import "virtual:uno.css";
import "@unocss/reset/tailwind.css";
import App from "./app.vue";
import router from "./router";

createApp(App).use(router).mount("#app");
