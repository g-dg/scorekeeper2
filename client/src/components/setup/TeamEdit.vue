<script lang="ts">
const defaultTeam: Team = {
  id: null,
  group_participation_id: "",
  name: "",
  description: "",
  enabled: true,
};
</script>

<script setup lang="ts">
import type { GroupParticipation } from "@/api/group_participation";
import type { Group } from "@/api/groups";
import type { Season } from "@/api/seasons";
import { TeamsClient, type Team } from "@/api/teams";
import clone from "@/helpers/clone";
import { computed, onMounted, ref, watch, type PropType } from "vue";

const props = defineProps({
  loading: {
    type: Number,
    default: 0,
  },
  team: {
    type: Object as PropType<Team>,
    default: () => clone(defaultTeam),
  },
  groupParticipations: {
    type: Array as PropType<GroupParticipation[]>,
    required: true,
  },
  groupParticipationMap: {
    type: Map as PropType<Map<string, GroupParticipation>>,
    required: true,
  },
  seasonMap: {
    type: Map as PropType<Map<string, Season>>,
    required: true,
  },
  groupMap: {
    type: Map as PropType<Map<string, Group>>,
    required: true,
  },
});

const team = ref<Team>(clone(props.team));
watch(
  computed(() => props.team),
  () => (team.value = clone(props.team))
);
onMounted(() => (team.value = clone(props.team)));

const emit = defineEmits<{
  (e: "update"): void;
}>();

const selfLoading = ref(0);
const loading = computed(() => props.loading + selfLoading.value);

function validate() {
  if ((team.value.group_participation_id ?? "") == "") {
    alert("Group Participation is required");
    return false;
  }

  return true;
}

async function create() {
  if (!validate()) return;

  selfLoading.value++;
  try {
    await TeamsClient.createTeam(team.value);

    team.value = clone(props.team);
  } catch (e) {
    console.error(e);
    alert("Error occurred creating team");
  }
  selfLoading.value--;
  emit("update");
}

async function update() {
  if (!validate()) return;

  selfLoading.value++;
  try {
    await TeamsClient.updateTeam(team.value.id!, team.value);
  } catch (e) {
    console.error(e);
    alert("Error occurred updating team");
  }
  selfLoading.value--;
  emit("update");
}

async function remove() {
  if (!confirm(`Really delete team "${team.value.name}"?`)) {
    return;
  }

  selfLoading.value++;
  try {
    await TeamsClient.deleteTeam(team.value.id!);
  } catch (e) {
    console.error(e);
    alert("Error occurred deleting team");
  }
  selfLoading.value--;
  emit("update");
}
</script>

<template>
  <form v-if="loading == 0" @submit.prevent>
    <label> Group Participation: </label>
    <select v-model="team.group_participation_id">
      <option value=""></option>
      <option
        v-for="groupParticipation in groupParticipations"
        :key="groupParticipation.id ?? ''"
        :value="groupParticipation.id"
      >
        {{
          seasonMap.get(
            groupParticipationMap.get(groupParticipation.id ?? "")?.season_id ??
              ""
          )?.name
        }}
        -
        {{
          groupMap.get(
            groupParticipationMap.get(groupParticipation.id ?? "")?.group_id ??
              ""
          )?.name
        }}
      </option>
    </select>

    <label> Name: </label>
    <input v-model="team.name" type="text" />

    <label> Description: </label>
    <textarea v-model="team.description"></textarea>

    <label> Enabled: </label>
    <input v-model="team.enabled" type="checkbox" />

    <button v-if="team.id == null" @click="create" type="submit">Create</button>
    <button v-if="team.id != null" @click="update" type="submit">Update</button>
    <button v-if="team.id != null" @click="remove">Delete</button>

    <template v-if="team.id != null">
      ID: <code>{{ team.id }}</code>
      <br />
    </template>
  </form>
</template>

<style lang="scss" scoped></style>
