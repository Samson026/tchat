import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import { createPinia } from "pinia";
import { setupListeners } from "./listeners.ts";
import { router } from "./router/index.ts";

const pinia = createPinia();
const app = createApp(App);

app.use(router);
app.use(pinia);

// setup tauri listeners
setupListeners();

app.mount("#app");
