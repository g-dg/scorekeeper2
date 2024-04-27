<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import { useAuthStore } from "./stores/auth";
import { computed } from "vue";

const authStore = useAuthStore();

const isAuthenticated = computed(() => authStore.isAuthenticated);
const user = computed(() => authStore.user);

const clientVersion = __APP_VERSION__;
</script>

<template>
  <div>
    <header>
      <nav v-if="isAuthenticated">
        <ul>
          <li>
            <RouterLink :to="{ name: 'home' }">Home</RouterLink>
          </li>
          <li v-if="user?.permission_setup_admin">
            <RouterLink :to="{ name: 'setup' }">Setup</RouterLink>
          </li>
          <li v-if="user?.permission_user_admin">
            <RouterLink :to="{ name: 'user_list' }">Users</RouterLink>
          </li>
          <li v-if="user?.permission_modify_self">
            <RouterLink :to="{ name: 'account' }">Account</RouterLink>
          </li>
          <li>
            <RouterLink :to="{ name: 'about' }">About</RouterLink>
          </li>
          <li>
            <RouterLink :to="{ name: 'logout' }">Logout</RouterLink>
          </li>
        </ul>
      </nav>
    </header>

    <main>
      <RouterView />
    </main>

    <footer>ScoreKeeper2 Copyright &copy; 2024 Garnet DeGelder</footer>
  </div>
</template>

<style lang="scss" scoped>
footer {
  margin-top: 2em;
}
</style>
