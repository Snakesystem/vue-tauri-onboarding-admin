import { RouteRecordRaw } from "vue-router";
const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'home',
        component: () => import('./admin/Home.vue'),
        meta: { requiresAuth: true, showHeader: true, title: "Home" }
    },
    { 
        path: "/login", 
        name: "Login", 
        component: () => import('./auth/Login.vue'), 
        meta: { guestOnly: true, showHeader: false, title: "Login" }
    },
    {
        path: '/:catchAll(.*)',
        name: 'NotFound',
        component: () => import('./auth/NotFound.vue'),
        meta: { showHeader: false, title: "Not Found" }
    }
]

export default routes