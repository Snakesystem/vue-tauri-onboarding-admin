import { createApp } from "vue";
import App from "./App.vue";
import "./assets/css/style.css";
import "./assets/scss/style.scss";
import { createRouter, createWebHistory } from "vue-router";
import routes from "./routes";
import { invoke } from "@tauri-apps/api/core";
import { ActionResult } from "./app/model";
import {createBootstrap} from 'bootstrap-vue-next'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: routes
})

async function checkSession() {
    try {
      const response: ActionResult = await invoke("check_session");
      return response.result;
    } catch (error: ActionResult | any) {
      return false;
    }
}

router.beforeEach(async (to, _from, next) => {
    const isAuthenticated = await checkSession();

    if (to.meta.requiresAuth && !isAuthenticated) {
      next("/login");
      router.replace("/login");
    } else if (to.meta.guestOnly && isAuthenticated) {
      next("/");
      router.replace("/");
    } else {
      document.title = `${to.meta.title} - Customers Onboarding` || "Customers Onboarding";
      next();
    }
});

createApp(App).use(router).use(createBootstrap).mount("#app");
