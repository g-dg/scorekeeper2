<script setup lang="ts">
import { UserClient, UserPermission, type User } from '@/api/users';
import { natcasecmp } from '@/helpers/sort';
import { useAuthStore } from '@/stores/auth';
import { computed, onMounted, ref } from 'vue';

const authStore = useAuthStore();
const currentUserId = computed(() => authStore.user!.id);

const users = ref<User[] | null>(null);

const loading = ref(0);

async function loadUsers() {
  try {
    loading.value++;
    users.value = (await UserClient.listUsers()).sort((a, b) => natcasecmp(a.username, b.username));
    newUser.value = JSON.parse(JSON.stringify(defaultUser));
  } catch (e) {
    alert("Error occurred loading users");
    throw e;
  } finally {
    loading.value--;
  }
}
onMounted(loadUsers);

const shownPassword = ref<string | null | undefined>();

const defaultUser: User = {
  id: null,
  username: "",
  new_password: "",
  enabled: true,
  permissions: UserPermission.MODIFY_SELF,
  permission_modify_self: true,
  permission_user_admin: false,
  permission_setup_admin: false,
  permission_view_results: false,
  permission_view_scores: false,
  permission_enter_scores: false,
  permission_view_registration: false,
  permission_enter_registration: false,
};

const newUser = ref<User>(JSON.parse(JSON.stringify(defaultUser)));

async function createUser() {
  try {
    loading.value++;
    await UserClient.createUser(newUser.value);
  } catch (e) {
    alert("Error occurred creating user");
    throw e;
  } finally {
    await loadUsers();
    loading.value--;
  }
}

async function updateUser(index: number) {
  const user = users.value![index];

  if (user.new_password === "")
    user.new_password = null;

  try {
    loading.value++;
    await UserClient.updateUser(user.id!, user);
  } catch (e) {
    alert("Error occurred updating user");
    throw e;
  } finally {
    await loadUsers();
    loading.value--;
  }
}

async function deleteUser(index: number) {
  const user = users.value![index];

  if (confirm(`Really delete user "${user.username}"?`)) {
    try {
      loading.value++;
      await UserClient.deleteUser(user.id!);
    } catch (e) {
      alert("Error occurred deleting user");
      throw e;
    } finally {
      await loadUsers();
      loading.value--;
    }
  }
}

</script>

<template>
  <main>
    <h1> User Administration </h1>
    <button @click="loadUsers"> Revert / Reload </button>
    <table v-if="!loading" style="width: 100%; text-align: center;">
      <thead>
        <tr>
          <th></th>
          <th>Username</th>
          <th>Change Password</th>
          <th>Login</th>
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
        <tr>
          <td>
            <button @click="createUser">Create</button>
          </td>
          <td>
            <input v-model="newUser.username" type="text" />
          </td>
          <td>
            <input v-model="newUser.new_password" :type="newUser.id === shownPassword ? 'text' : 'password'" />
            <button @click="shownPassword = (shownPassword === newUser.id ? undefined : newUser.id)">
              {{ shownPassword === newUser.id ? 'Hide' : 'Show' }}
            </button>
          </td>
          <td>
            <input v-model="newUser.enabled" type="checkbox" />
          </td>
          <td>
            <input v-model="newUser.permission_modify_self" type="checkbox" />
          </td>
          <td>
            <input v-model="newUser.permission_user_admin" type="checkbox" />
          </td>
          <td>
            <input v-model="newUser.permission_setup_admin" type="checkbox" />
          </td>
          <td>
            <input v-model="newUser.permission_view_results" type="checkbox" />
          </td>
          <td>
            <input v-model="newUser.permission_view_scores" type="checkbox" />
          </td>
          <td>
            <input v-model="newUser.permission_enter_scores" type="checkbox" />
          </td>
          <td>
            <input v-model="newUser.permission_view_registration" type="checkbox" />
          </td>
          <td>
            <input v-model="newUser.permission_enter_registration" type="checkbox" />
          </td>
        </tr>

        <tr v-for="(user, index) in users" :key="user.id ?? ''">
          <td>
            <button @click="updateUser(index)">Update</button>
            <button @click="deleteUser(index)">Delete</button>
          </td>
          <td>
            <input v-model="user.username" type="text" />
          </td>
          <td>
            <input v-model="user.new_password" :type="user.id === shownPassword ? 'text' : 'password'" />
            <button @click="shownPassword = (shownPassword === user.id ? undefined : user.id)">
              {{ shownPassword === user.id ? 'Hide' : 'Show' }}
            </button>
          </td>
          <td>
            <input v-model="user.enabled" :disabled="user.id == currentUserId" type="checkbox" />
          </td>
          <td>
            <input v-model="user.permission_modify_self" type="checkbox" />
          </td>
          <td>
            <input v-model="user.permission_user_admin" :disabled="user.id == currentUserId" type="checkbox" />
          </td>
          <td>
            <input v-model="user.permission_setup_admin" type="checkbox" />
          </td>
          <td>
            <input v-model="user.permission_view_results" type="checkbox" />
          </td>
          <td>
            <input v-model="user.permission_view_scores" type="checkbox" />
          </td>
          <td>
            <input v-model="user.permission_enter_scores" type="checkbox" />
          </td>
          <td>
            <input v-model="user.permission_view_registration" type="checkbox" />
          </td>
          <td>
            <input v-model="user.permission_enter_registration" type="checkbox" />
          </td>
        </tr>
      </tbody>
    </table>
    <div v-else style="text-align: center;"> Loading... </div>
  </main>
</template>

<style lang="scss" scoped></style>
