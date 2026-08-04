import { createMemoryHistory, createRouter } from "vue-router";
import App from "../App.vue";
import Login from "../pages/Login.vue";
import Home from "../pages/Home.vue";

const routes = [
	{ path: "/", component: Login },
	{ path: "/about", component: App },
	{ path: "/home", component: Home },
];

export const router = createRouter({
	history: createMemoryHistory(),
	routes,
});
