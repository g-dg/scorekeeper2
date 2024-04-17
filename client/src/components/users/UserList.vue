<script setup lang="ts">
import { UserClient, type User } from "@/api/users";
import { natcasecmp } from "@/helpers/sort";
import { useAuthStore } from "@/stores/auth";
import { computed, onMounted, ref } from "vue";

const authStore = useAuthStore();
const currentUserId = computed(() => authStore.user!.id);

const users = ref<User[] | null>(null);

const loading = ref(0);

async function loadUsers() {
  try {
    loading.value++;
    users.value = (await UserClient.listUsers()).sort((a, b) =>
      natcasecmp(a.username, b.username)
    );
  } catch (e) {
    alert("Error occurred loading users");
    throw e;
  } finally {
    loading.value--;
  }
}
onMounted(loadUsers);
</script>

<template>
  <main>
    <h1>User Administration</h1>

    <button @click="loadUsers">Reload</button>
    <br />

    <RouterLink :to="{ name: 'user_create' }">Create</RouterLink>
    <table v-if="!loading">
      <thead>
        <tr>
          <th>Username</th>
          <th>Enabled</th>
          <th>Account</th>
          <th>Users</th>
          <th>Setup</th>
          <th>Results</th>
          <th>Score List</th>
          <th>Score Entry</th>
          <th>Registration List</th>
          <th>Registration Entry</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="user in users" :key="user.id ?? ''">
          <td>
            <RouterLink :to="{ name: 'user_edit', params: { id: user.id } }">
              {{ user.username }}
            </RouterLink>
          </td>
          <td>
            {{ user.enabled ? "Yes" : "No" }}
          </td>
          <td>
            {{ user.permission_modify_self ? "Yes" : "No" }}
          </td>
          <td>
            {{ user.permission_user_admin ? "Yes" : "No" }}
          </td>
          <td>
            {{ user.permission_setup_admin ? "Yes" : "No" }}
          </td>
          <td>
            {{ user.permission_view_results ? "Yes" : "No" }}
          </td>
          <td>
            {{ user.permission_view_scores ? "Yes" : "No" }}
          </td>
          <td>
            {{ user.permission_enter_scores ? "Yes" : "No" }}
          </td>
          <td>
            {{ user.permission_view_registration ? "Yes" : "No" }}
          </td>
          <td>
            {{ user.permission_enter_registration ? "Yes" : "No" }}
          </td>
        </tr>
      </tbody>
    </table>
    <div v-else style="text-align: center">Loading...</div>
  </main>
</template>

<style lang="scss" scoped></style>
