<script lang="ts">
const defaultGroupParticipation: GroupParticipation = {
  id: null,
  group_id: "",
  season_id: "",
  description: "",
  enabled: true,
};
</script>

<script setup lang="ts">
import {
  GroupParticipationsClient,
  type GroupParticipation,
} from "@/api/group_participation";
import type { Group } from "@/api/groups";
import type { Season } from "@/api/seasons";
import clone from "@/helpers/clone";
import { computed, onMounted, ref, watch, type PropType } from "vue";

const props = defineProps({
  loading: {
    type: Number,
    default: 0,
  },
  groupParticipation: {
    type: Object as PropType<GroupParticipation>,
    default: () => clone(defaultGroupParticipation),
  },
  seasons: {
    type: Array as PropType<Season[]>,
    required: true,
  },
  groups: {
    type: Array as PropType<Group[]>,
    required: true,
  },
});

const groupParticipation = ref<GroupParticipation>(
  clone(props.groupParticipation)
);
watch(
  computed(() => props.groupParticipation),
  () => (groupParticipation.value = clone(props.groupParticipation))
);
onMounted(() => (groupParticipation.value = clone(props.groupParticipation)));

const emit = defineEmits<{
  (e: "update"): void;
}>();

const selfLoading = ref(0);
const loading = computed(() => props.loading + selfLoading.value);

async function create() {
  if (groupParticipation.value.season_id == null) {
    alert("Season is required");
    return;
  }

  if (groupParticipation.value.group_id == null) {
    alert("Group is required");
    return;
  }

  if (
    groupParticipation.value.season_id == "" ||
    groupParticipation.value.group_id == ""
  ) {
    alert("Season and Group are required");
    return;
  }

  selfLoading.value++;
  try {
    await GroupParticipationsClient.createGroupParticipation(
      groupParticipation.value
    );

    groupParticipation.value = clone(props.groupParticipation);
  } catch (e) {
    console.error(e);
    alert("Error occurred creating group participation");
  }
  selfLoading.value--;
  emit("update");
}

async function update() {
  if (groupParticipation.value.season_id == null) {
    alert("Season is required");
    return;
  }

  if (groupParticipation.value.group_id == null) {
    alert("Group is required");
    return;
  }

  if (
    groupParticipation.value.season_id == "" ||
    groupParticipation.value.group_id == ""
  ) {
    alert("Season and Group are required");
    return;
  }

  selfLoading.value++;
  try {
    await GroupParticipationsClient.updateGroupParticipation(
      groupParticipation.value.id!,
      groupParticipation.value
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred updating group participation");
  }
  selfLoading.value--;
  emit("update");
}

async function remove() {
  if (!confirm(`Really delete group participation?`)) {
    return;
  }

  selfLoading.value++;
  try {
    await GroupParticipationsClient.deleteGroupParticipation(
      groupParticipation.value.id!
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred deleting group participation");
  }
  selfLoading.value--;
  emit("update");
}
</script>

<template>
  <form v-if="loading == 0" @submit.prevent>
    <label> Group: </label>
    <select v-model="groupParticipation.group_id">
      <option value=""></option>
      <option v-for="group in groups" :key="group.id ?? ''" :value="group.id">
        {{ group.name }}
      </option>
    </select>

    <label> Season: </label>
    <select v-model="groupParticipation.season_id">
      <option value=""></option>
      <option
        v-for="season in seasons"
        :key="season.id ?? ''"
        :value="season.id"
      >
        {{ season.name }}
      </option>
    </select>

    <label> Description: </label>
    <textarea v-model="groupParticipation.description"></textarea>

    <label> Enabled: </label>
    <input v-model="groupParticipation.enabled" type="checkbox" />

    <button v-if="groupParticipation.id == null" @click="create" type="submit">
      Create
    </button>
    <button v-if="groupParticipation.id != null" @click="update" type="submit">
      Update
    </button>
    <button v-if="groupParticipation.id != null" @click="remove">Delete</button>

    <template v-if="groupParticipation.id != null">
      ID: <code>{{ groupParticipation.id }}</code>
      <br />
    </template>
  </form>
</template>

<style lang="scss" scoped></style>
