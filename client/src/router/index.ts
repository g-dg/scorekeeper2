import { createRouter, createWebHistory } from "vue-router";
import HomeView from "@/views/HomeView.vue";
import LoginView from "@/views/LoginView.vue";
import { useAuthStore } from "@/stores/auth";
import LogoutView from "@/views/LogoutView.vue";

const login_route = { name: "login" };

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      name: "home",
      path: "/",
      component: HomeView,
    },
    {
      name: "login",
      path: "/login",
      component: LoginView,
      meta: { requiresAuth: false },
    },
    {
      name: "logout",
      path: "/logout",
      component: LogoutView,
      meta: { requiresAuth: false },
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
