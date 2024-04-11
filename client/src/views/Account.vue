<script setup lang="ts">
import { UserClient, UserPermission, type User } from '@/api/users';
import { useAuthStore } from '@/stores/auth';
import { computed, onMounted, ref } from 'vue';

const authStore = useAuthStore();
const currentUserId = computed(() => authStore.user!.id!);


const new_password = ref("");
const new_password_confirm = ref("");

const message = ref("");

const changingPassword = ref(false);
async function changePassword() {
  try {
    message.value = "";
    changingPassword.value = true;

    if (new_password.value !== new_password_confirm.value) {
      alert("Passwords do not match.");
      return;
    }

    await UserClient.changePassword(currentUserId.value, new_password.value);

    new_password.value = "";
    new_password_confirm.value = "";
    message.value = "Password has been changed";
  } catch (e) {
    alert("Error occurred changing password");
    throw e;
  } finally {
    changingPassword.value = false;
  }
}

</script>

<template>
  <main>
    <h1> My Account </h1>

    

    <form @submit.prevent="changePassword">
      Change Password:
      <input v-model="new_password" type="password" required placeholder="New password" />
      <input v-model="new_password_confirm" type="password" required placeholder="Confirm new password" />
      <button type="submit">
        Change Password
      </button>
    </form>

    <em>{{ message }}</em>

    <br /><br />
    User ID: <code>{{ currentUserId }}</code>
  </main>
</template>

<style lang="scss" scoped></style>
