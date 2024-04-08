<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import { useAuthStore } from "./stores/auth";
import { computed } from "vue";

const authStore = useAuthStore();

const isAuthenticated = computed(() => authStore.isAuthenticated);
</script>

<template>
  <header>
    <nav v-if="isAuthenticated">
      <ul>
        <li>
          <RouterLink :to='{ name: "home" }'>Home</RouterLink>
        </li>
        <li>
          <RouterLink :to='{ name: "logout" }'>Logout</RouterLink>
        </li>
      </ul>
    </nav>
  </header>

  <RouterView />
</template>

<style lang="scss" scoped></style>
