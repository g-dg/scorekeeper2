<script setup lang="ts">
import { API_URI } from "@/api/api";
import { useAuthStore } from "@/stores/auth";
import { computed, onMounted, ref } from "vue";

const authStore = useAuthStore();
const isAuthenticated = computed(() => authStore.isAuthenticated);

const license = ref<string>();
async function loadLicense() {
  license.value = await (
    await fetch(`${API_URI}about/license`, { mode: "cors", redirect: "follow" })
  ).text();
}
onMounted(loadLicense);

const serverVersion = ref<string>();
async function loadVersion() {
  serverVersion.value = await (
    await fetch(`${API_URI}about/version`, { mode: "cors", redirect: "follow" })
  ).text();
}
onMounted(loadVersion);

const clientVersion = ref<string>(__APP_VERSION__);
</script>

<template>
  <main>
    <h1>Garnet DeGelder's ScoreKeeper 2</h1>

    <RouterLink v-if="!isAuthenticated" :to="{ name: 'login' }"
      >Login</RouterLink
    >

    <h2>Version:</h2>
    <dl>
      <dt>Client:</dt>
      <dd>
        <code> {{ clientVersion }} </code>
      </dd>

      <dt>Server:</dt>
      <dd>
        <em v-if="serverVersion == undefined">Loading...</em>
        <code v-else> {{ serverVersion }}</code>
      </dd>
    </dl>

    <h2>License:</h2>
    <code v-if="license != undefined">
      <pre>{{ license }}</pre>
    </code>
    <em v-else> MIT License </em>

    <h2>Source Code:</h2>
    <p>
      The source code for the client and server is hosted on
      <a
        href="https://github.com/g-dg/scorekeeper2"
        target="_blank"
        rel="noopener noreferrer"
        >Github</a
      >
    </p>
  </main>
</template>

<style lang="scss" scoped></style>
