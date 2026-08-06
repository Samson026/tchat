import { createMemoryHistory, createRouter } from "vue-router";
import App from "../App.vue";
import ChatPage from "../pages/ChatPage.vue";
import HomePage from "../pages/HomePage.vue";
import LoginPage from "../pages/LoginPage.vue";
import SearchPage from "../pages/SearchPage.vue";

const routes = [
	{ path: "/", component: LoginPage },
	{ path: "/about", component: App },
	{
		path: "/home",
		component: HomePage,
		children: [
			{
				path: "chat/:id",
				component: ChatPage,
			},
			{
				path: "search",
				component: SearchPage,
			},
		],
	},
];

export const router = createRouter({
	history: createMemoryHistory(),
	routes,
});
