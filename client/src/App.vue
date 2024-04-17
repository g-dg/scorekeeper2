<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import { useAuthStore } from "./stores/auth";
import { computed } from "vue";

const authStore = useAuthStore();

const isAuthenticated = computed(() => authStore.isAuthenticated);
const user = computed(() => authStore.user);
</script>

<template>
  <header>
    <nav v-if="isAuthenticated">
      <ul>
        <li>
          <RouterLink :to="{ name: 'home' }">Home</RouterLink>
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

  <RouterView />
</template>

<style lang="scss" scoped></style>
