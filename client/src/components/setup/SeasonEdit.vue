<script lang="ts">
const defaultSeason: Season = {
  id: null,
  name: "",
  description: "",
  score_calculator: null,
  enabled: true,
};
</script>

<script setup lang="ts">
import type { ScoreCalculator } from "@/api/score_calculators";
import { SeasonsClient, type Season } from "@/api/seasons";
import clone from "@/helpers/clone";
import { computed, onMounted, ref, watch, type PropType } from "vue";

const props = defineProps({
  loading: {
    type: Number,
    default: 0,
  },
  season: {
    type: Object as PropType<Season>,
    default: () => clone(defaultSeason),
  },
  scoreCalculators: {
    type: Array as PropType<ScoreCalculator[]>,
    required: true,
  },
});

const season = ref<Season>(clone(props.season));
watch(
  computed(() => props.season),
  () => (season.value = clone(props.season))
);
onMounted(() => (season.value = clone(props.season)));

const emit = defineEmits<{
  (e: "update"): void;
}>();

const selfLoading = ref(0);
const loading = computed(() => props.loading + selfLoading.value);

async function create() {
  selfLoading.value++;
  try {
    await SeasonsClient.createSeason(season.value);

    season.value = clone(props.season);
  } catch (e) {
    console.error(e);
    alert("Error occurred creating season");
  }
  selfLoading.value--;
  emit("update");
}

async function update() {
  selfLoading.value++;
  try {
    await SeasonsClient.updateSeason(season.value.id!, season.value);
  } catch (e) {
    console.error(e);
    alert("Error occurred updating season");
  }
  selfLoading.value--;
  emit("update");
}

async function remove() {
  if (!confirm(`Really delete season "${season.value.name}"?`)) {
    return;
  }

  selfLoading.value++;
  try {
    await SeasonsClient.deleteSeason(season.value.id!);
  } catch (e) {
    console.error(e);
    alert("Error occurred deleting season");
  }
  selfLoading.value--;
  emit("update");
}
</script>

<template>
  <form v-if="loading == 0" @submit.prevent>
    <template v-if="season.id != null">
      ID: <code>{{ season.id }}</code>
      <br />
    </template>

    <label>Name: </label>
    <input v-model="season.name" type="text" />

    <br />

    <label>Description: </label>
    <textarea v-model="season.description"></textarea>

    <br />

    <label>Score Calculator: </label>
    <select v-model="season.score_calculator">
      <option :value="null"></option>
      <option
        v-for="scoreCalculator in scoreCalculators"
        :value="scoreCalculator.id"
      >
        {{ scoreCalculator.name }}
      </option>
    </select>

    <br />

    <label>Enabled: </label>
    <input v-model="season.enabled" type="checkbox" />

    <br />

    <button v-if="season.id == null" @click="create" type="submit">
      Create
    </button>
    <button v-if="season.id != null" @click="update" type="submit">
      Update
    </button>
    <button v-if="season.id != null" @click="remove">Delete</button>
  </form>
</template>

<style lang="scss" scoped></style>
