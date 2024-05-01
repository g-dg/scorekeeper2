<script lang="ts">
const defaultSeason: Season = {
  id: null,
  name: "",
  description: "",
  score_calculator: null,
  calculator_config: "{}",
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

function validate() {
  try {
    const config = JSON.parse(season.value.calculator_config);
    if (typeof config != "object" || Array.isArray(config)) throw new Error();
    season.value.calculator_config = JSON.stringify(config);
  } catch {
    alert("Calculator config must be a valid JSON object");
    return false;
  }

  return true;
}

async function create() {
  if (!validate()) return;

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
  if (!validate()) return;

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
    <label> Name: </label>
    <input v-model="season.name" type="text" />

    <label> Description: </label>
    <textarea v-model="season.description"></textarea>

    <label> Score Calculator: </label>
    <select v-model="season.score_calculator">
      <option :value="null"></option>
      <option
        v-for="scoreCalculator in scoreCalculators.filter(
          (x) => x.supports_seasons
        )"
        :value="scoreCalculator.id"
      >
        {{ scoreCalculator.name }}
      </option>
    </select>

    <label> Calculator Config: </label>
    <textarea v-model="season.calculator_config"></textarea>

    <label> Enabled: </label>
    <input v-model="season.enabled" type="checkbox" />

    <button v-if="season.id == null" @click="create" type="submit">
      Create
    </button>
    <button v-if="season.id != null" @click="update" type="submit">
      Update
    </button>
    <button v-if="season.id != null" @click="remove">Delete</button>

    <template v-if="season.id != null">
      ID: <code>{{ season.id }}</code>
      <br />
    </template>
  </form>
</template>

<style lang="scss" scoped></style>
