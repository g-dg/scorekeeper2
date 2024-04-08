import { ref, computed, watch } from "vue";
import { defineStore } from "pinia";
import type { User } from "@/api/users";
import router from "@/router";

export const useAuthStore = defineStore("auth", () => {
  const API_TOKEN_LOCALSTORAGE_KEY = "ScoreKeeper_ApiToken";

  const token = ref<string | null>(null);

  const user = ref<User | null>(null);

  const isAuthenticated = computed(() => token.value != null);

  function loadFromLocalStorage() {
    if (token.value == null) {
      token.value = window.localStorage.getItem(API_TOKEN_LOCALSTORAGE_KEY);
    }
  }

  loadFromLocalStorage();

  const authBroadcastChannel = new BroadcastChannel("ScoreKeeper_Auth");

  watch(token, () => {
    if (token.value != window.localStorage.getItem(API_TOKEN_LOCALSTORAGE_KEY)) {
      if (token.value != null) {
        window.localStorage.setItem(API_TOKEN_LOCALSTORAGE_KEY, token.value);
        authBroadcastChannel.postMessage("login");
      } else {
        window.localStorage.removeItem(API_TOKEN_LOCALSTORAGE_KEY);
        authBroadcastChannel.postMessage("logout");
      }
    }
  });

  authBroadcastChannel.addEventListener("message", (event) => {
    switch (event.data) {
      case "login":
        token.value = window.localStorage.getItem(API_TOKEN_LOCALSTORAGE_KEY);
        break;
      case "logout":
        token.value = null;
        user.value = null;
        router.push({ name: "login" });
        break;
    }
  });

  return {
    token,
    user,
    isAuthenticated,
  };
});
