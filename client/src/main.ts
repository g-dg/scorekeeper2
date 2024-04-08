import "./assets/main.css";

import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";

import { useAuthStore } from "./stores/auth";
import { AuthClient } from "./api/auth";

(async () => {
  const app = createApp(App);

  app.use(createPinia());
  app.use(router);

  const authStore = useAuthStore();

  // if user is not authenticated, redirect to login
  if (!authStore.isAuthenticated) {
    await router.push({ name: "login" });
  } else {
    // load current user from api
    try {
      const user = await AuthClient.getCurrentUser();

      authStore.user = user;
    } catch {
      // logout if error occurred getting current user (indicates invalid session)
      await router.push({ name: "logout" });
    }
  }

  app.mount("#app");
})();
