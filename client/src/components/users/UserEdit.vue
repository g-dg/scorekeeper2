<script setup lang="ts">
import { UserClient, UserPermission, type User } from '@/api/users';
import clone from '@/helpers/clone';
import { randomString } from '@/helpers/random';
import { natcasecmp } from '@/helpers/sort';
import router from '@/router';
import { useAuthStore } from '@/stores/auth';
import { computed, onMounted, ref, watch } from 'vue';

const props = defineProps({
  id: String,
});
const userId = computed(() => props.id);

const authStore = useAuthStore();
const currentUserId = computed(() => authStore.user!.id);

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

const user = ref<User>(clone(defaultUser));

const password = ref("");
const passwordConfirm = ref("");
const blankPassword = ref(false);

const loading = ref(0);

async function loadUser() {
  if (props.id == null) {
    // new user
    user.value = clone(defaultUser);

    password.value = "";
    passwordConfirm.value = "";
    blankPassword.value = false;
  } else {
    // existing user
    try {
      loading.value++;

      user.value = (await UserClient.getUser(props.id));

      password.value = "";
      passwordConfirm.value = "";
      blankPassword.value = false;
    } catch (e) {
      alert("Error occurred loading user");
      throw e;
    } finally {
      loading.value--;
    }
  }
}
onMounted(loadUser);
watch(userId, loadUser);

async function createUser() {
  if (password.value == passwordConfirm.value) {
    user.value.new_password = password.value;
  } else {
    alert("Passwords do not match");
    return;
  }

  try {
    loading.value++;

    await UserClient.createUser(user.value);
  } catch (e) {
    alert("Error occurred creating user");
    throw e;
  } finally {
    await router.push({ name: "user_list" });
    loading.value--;
  }
}

async function updateUser() {
  if (password.value != "" || blankPassword.value) {
    if (password.value == passwordConfirm.value) {
      user.value.new_password = password.value;
    } else {
      alert("Passwords do not match");
      return;
    }
  } else {
    user.value.new_password = null;
  }

  try {
    loading.value++;
    await UserClient.updateUser(userId.value!, user.value);
  } catch (e) {
    alert("Error occurred updating user");
    throw e;
  } finally {
    await loadUser();
    loading.value--;
  }
}

async function deleteUser() {
  if (confirm(`Really delete user "${user.value.username}"?`)) {
    try {
      loading.value++;
      await UserClient.deleteUser(userId.value!);
    } catch (e) {
      alert("Error occurred deleting user");
      throw e;
    } finally {
      await router.push({ name: "user_list" });
      loading.value--;
    }
  }
}

</script>

<template>
  <main>
    <h1> User Administration </h1>

    <RouterLink :to="{ name: 'user_list' }">&lt; All Users</RouterLink>

    <template v-if="userId != null">
      <br />
      <button v-if="userId != null" @click="loadUser" :disabled="loading > 0"> Revert / Reload </button>
      <br />
    </template>

    <form v-if="loading == 0" @submit.prevent>

      <div v-if="userId != null">
        User ID: <code>{{ user.id }}</code>
        <br />
      </div>

      <label for="username">Username: </label>
      <input v-model="user.username" type="text" id="username" />

      <br />

      <label for="password">Password: </label>
      <input v-model="password" type="password" autocomplete="off" id="password" />

      <template v-if="userId != null">
        <label for="blank_password"> Blank: </label>
        <input v-model="blankPassword" type="checkbox" id="blank_password" />
      </template>

      <br />

      <label for="password_confirm">Confirm password: </label>
      <input v-model="passwordConfirm" type="password" autocomplete="off" id="password_confirm" />

      <br />

      <label for="enabled">Can Login: </label>
      <input v-model="user.enabled" :disabled="userId == currentUserId" type="checkbox" id="enabled" />

      <br />

      <label for="permission_modify_self">Manage Self: </label>
      <input v-model="user.permission_modify_self" type="checkbox" id="permission_modify_self" />

      <br />

      <label for="permission_user_admin">Manage Users: </label>
      <input v-model="user.permission_user_admin" :disabled="userId == currentUserId" type="checkbox"
        id="permission_user_admin" />

      <br />

      <label for="permission_setup_admin">Manage Setup: </label>
      <input v-model="user.permission_setup_admin" type="checkbox" id="permission_setup_admin" />

      <br />

      <label for="permission_view_results">View Results: </label>
      <input v-model="user.permission_view_results" type="checkbox" id="permission_view_results" />

      <br />

      <label for="permission_view_scores">View Scores: </label>
      <input v-model="user.permission_view_scores" type="checkbox" id="permission_view_scores" />

      <br />

      <label for="permission_enter_scores">Score Entry: </label>
      <input v-model="user.permission_enter_scores" type="checkbox" id="permission_enter_scores" />

      <br />

      <label for="permission_view_registration">Registration List: </label>
      <input v-model="user.permission_view_registration" type="checkbox" id="permission_view_registration" />

      <br />

      <label for="permission_enter_registration">Registration Entry: </label>
      <input v-model="user.permission_enter_registration" type="checkbox" id="permission_enter_registration" />

      <br />

      <button v-if="userId == null" @click="createUser" type="submit">Create</button>
      <button v-if="userId != null" @click="updateUser" type="submit">Update</button>
      <button v-if="userId != null" @click="deleteUser">Delete</button>

    </form>
    <em v-else>Loading...</em>

  </main>
</template>

<style lang="scss" scoped></style>
