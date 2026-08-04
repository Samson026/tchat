import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import { createPinia } from "pinia";
import { router } from "./router/index.ts";
import { setupListeners } from "./listeners.ts";

const pinia = createPinia();
const app = createApp(App);

app.use(router);
app.use(pinia);

// setup tauri listeners
setupListeners()

app.mount("#app");
