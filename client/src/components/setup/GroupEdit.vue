<script lang="ts">
const defaultGroup: Group = {
  id: null,
  name: "",
  description: "",
  enabled: true,
};
</script>

<script setup lang="ts">
import { GroupsClient, type Group } from "@/api/groups";
import clone from "@/helpers/clone";
import { computed, onMounted, ref, watch, type PropType } from "vue";

const props = defineProps({
  loading: {
    type: Number,
    default: 0,
  },
  group: {
    type: Object as PropType<Group>,
    default: () => clone(defaultGroup),
  },
});

const group = ref<Group>(clone(props.group));
watch(
  computed(() => props.group),
  () => (group.value = clone(props.group))
);
onMounted(() => (group.value = clone(props.group)));

const emit = defineEmits<{
  (e: "update"): void;
}>();

const selfLoading = ref(0);
const loading = computed(() => props.loading + selfLoading.value);

async function create() {
  selfLoading.value++;
  try {
    await GroupsClient.createGroup(group.value);

    group.value = clone(props.group);
  } catch (e) {
    console.error(e);
    alert("Error occurred creating group");
  }
  selfLoading.value--;
  emit("update");
}

async function update() {
  selfLoading.value++;
  try {
    await GroupsClient.updateGroup(group.value.id!, group.value);
  } catch (e) {
    console.error(e);
    alert("Error occurred updating group");
  }
  selfLoading.value--;
  emit("update");
}

async function remove() {
  if (!confirm(`Really delete group "${group.value.name}"?`)) {
    return;
  }

  selfLoading.value++;
  try {
    await GroupsClient.deleteGroup(group.value.id!);
  } catch (e) {
    console.error(e);
    alert("Error occurred deleting group");
  }
  selfLoading.value--;
  emit("update");
}
</script>

<template>
  <form v-if="loading == 0" @submit.prevent>
    <template v-if="group.id != null">
      ID: <code>{{ group.id }}</code>
      <br />
    </template>

    <label>Name: </label>
    <input v-model="group.name" type="text" />

    <br />

    <label>Description: </label>
    <textarea v-model="group.description"></textarea>

    <br />

    <label>Enabled: </label>
    <input v-model="group.enabled" type="checkbox" />

    <br />

    <button v-if="group.id == null" @click="create" type="submit">
      Create
    </button>
    <button v-if="group.id != null" @click="update" type="submit">
      Update
    </button>
    <button v-if="group.id != null" @click="remove">Delete</button>
  </form>
</template>

<style lang="scss" scoped></style>
