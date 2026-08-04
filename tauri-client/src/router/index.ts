import { createMemoryHistory, createRouter } from "vue-router";
import App from "../App.vue";
import Home from "../pages/Home.vue";
import Login from "../pages/Login.vue";

const routes = [
	{ path: "/", component: Login },
	{ path: "/about", component: App },
	{ path: "/home", component: Home },
];

export const router = createRouter({
	history: createMemoryHistory(),
	routes,
});
