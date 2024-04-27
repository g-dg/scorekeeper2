<script lang="ts">
const defaultCompetition: Competition = {
  id: null,
  name: "",
  description: "",
  enabled: true,
};
</script>

<script setup lang="ts">
import { CompetitionsClient, type Competition } from "@/api/competitions";
import clone from "@/helpers/clone";
import { computed, onMounted, ref, watch, type PropType } from "vue";

const props = defineProps({
  loading: {
    type: Number,
    default: 0,
  },
  competition: {
    type: Object as PropType<Competition>,
    default: () => clone(defaultCompetition),
  },
});

const competition = ref<Competition>(clone(props.competition));
watch(
  computed(() => props.competition),
  () => (competition.value = clone(props.competition))
);
onMounted(() => (competition.value = clone(props.competition)));

const emit = defineEmits<{
  (e: "update"): void;
}>();

const selfLoading = ref(0);
const loading = computed(() => props.loading + selfLoading.value);

async function create() {
  selfLoading.value++;
  try {
    await CompetitionsClient.createCompetition(competition.value);

    competition.value = clone(props.competition);
  } catch (e) {
    console.error(e);
    alert("Error occurred creating competition");
  }
  selfLoading.value--;
  emit("update");
}

async function update() {
  selfLoading.value++;
  try {
    await CompetitionsClient.updateCompetition(
      competition.value.id!,
      competition.value
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred updating competition");
  }
  selfLoading.value--;
  emit("update");
}

async function remove() {
  if (!confirm(`Really delete competition "${competition.value.name}"?`)) {
    return;
  }

  selfLoading.value++;
  try {
    await CompetitionsClient.deleteCompetition(competition.value.id!);
  } catch (e) {
    console.error(e);
    alert("Error occurred deleting competition");
  }
  selfLoading.value--;
  emit("update");
}
</script>

<template>
  <form v-if="loading == 0" @submit.prevent>
    <label> Name: </label>
    <input v-model="competition.name" type="text" />

    <label> Description: </label>
    <textarea v-model="competition.description"></textarea>

    <label> Enabled: </label>
    <input v-model="competition.enabled" type="checkbox" />

    <button v-if="competition.id == null" @click="create" type="submit">
      Create
    </button>
    <button v-if="competition.id != null" @click="update" type="submit">
      Update
    </button>
    <button v-if="competition.id != null" @click="remove">Delete</button>

    <template v-if="competition.id != null">
      ID: <code>{{ competition.id }}</code>
      <br />
    </template>
  </form>
</template>

<style lang="scss" scoped></style>
