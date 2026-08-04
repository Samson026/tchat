import { createMemoryHistory, createRouter } from "vue-router";
import App from "../App.vue";
import HomePage from "../pages/HomePage.vue";
import LoginPage from "../pages/LoginPage.vue";

const routes = [
	{ path: "/", component: LoginPage },
	{ path: "/about", component: App },
	{ path: "/home", component: HomePage },
];

export const router = createRouter({
	history: createMemoryHistory(),
	routes,
});
