import { RouteRecordRaw } from "vue-router";
import Home from "./admin/Home.vue";
import Login from "./auth/Login.vue";

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'home',
        component: () => Home,
        meta: { requiresAuth: true, showHeader: true, title: "Home" }
    },
    { 
        path: "/login", 
        name: "Login", 
        component: Login, 
        meta: { guestOnly: true, showHeader: false, title: "Login" }
    },
]

export default routes