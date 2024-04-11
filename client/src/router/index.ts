import { createRouter, createWebHistory } from "vue-router";
import { useAuthStore } from "@/stores/auth";

const login_route = { name: "login" };

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      name: "home",
      path: "/",
      component: () => import("@/views/HomeView.vue"),
    },
    {
      name: "login",
      path: "/login",
      component: () => import("@/views/LoginView.vue"),
      meta: { requiresAuth: false },
    },
    {
      name: "logout",
      path: "/logout",
      component: () => import("@/views/LogoutView.vue"),
      meta: { requiresAuth: false },
    },
    {
      name: "account",
      path: "/account",
      component: () => import("@/views/Account.vue"),
    },
    {
      name: "user_admin",
      path: "/users",
      component: () => import("@/views/UserAdmin.vue"),
    },
  ],
});

router.beforeEach((to, from, next) => {
  if (to.meta?.requiresAuth) {
    const authStore = useAuthStore();

    if (!authStore.isAuthenticated) {
      next(login_route);
      return;
    }
  }

  next();
});

export default router;
