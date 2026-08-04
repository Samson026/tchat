import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import { createPinia } from "pinia";
import { router } from "./router/index.ts";
import { setupListeners } from "./listeners.ts";

setupListeners()

const pinia = createPinia();

createApp(App).use(router).use(pinia).mount("#app");
