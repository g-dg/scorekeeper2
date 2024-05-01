<script lang="ts">
const defaultSeasonCompetition: SeasonCompetition = {
  id: null,
  season_id: "",
  competition_id: "",
  description: "",
  score_calculator: null,
  calculator_config: "{}",
  enabled: true,
};
</script>

<script setup lang="ts">
import type { Competition } from "@/api/competitions";
import type { ScoreCalculator } from "@/api/score_calculators";
import {
  SeasonCompetitionsClient,
  type SeasonCompetition,
} from "@/api/season_competitions";
import type { Season } from "@/api/seasons";
import clone from "@/helpers/clone";
import { computed, onMounted, ref, watch, type PropType } from "vue";

const props = defineProps({
  loading: {
    type: Number,
    default: 0,
  },
  seasonCompetition: {
    type: Object as PropType<SeasonCompetition>,
    default: () => clone(defaultSeasonCompetition),
  },
  seasons: {
    type: Array as PropType<Season[]>,
    required: true,
  },
  competitions: {
    type: Array as PropType<Competition[]>,
    required: true,
  },
  scoreCalculators: {
    type: Array as PropType<ScoreCalculator[]>,
    required: true,
  },
});

const seasonCompetition = ref<SeasonCompetition>(
  clone(props.seasonCompetition)
);
watch(
  computed(() => props.seasonCompetition),
  () => (seasonCompetition.value = clone(props.seasonCompetition))
);
onMounted(() => (seasonCompetition.value = clone(props.seasonCompetition)));

const emit = defineEmits<{
  (e: "update"): void;
}>();

const selfLoading = ref(0);
const loading = computed(() => props.loading + selfLoading.value);

function validate() {
  if ((seasonCompetition.value.season_id ?? "") == "") {
    alert("Season is required");
    return false;
  }

  if ((seasonCompetition.value.competition_id ?? "") == "") {
    alert("Competition is required");
    return false;
  }

  try {
    const config = JSON.parse(seasonCompetition.value.calculator_config);
    if (typeof config != "object" || Array.isArray(config)) throw new Error();
    seasonCompetition.value.calculator_config = JSON.stringify(config);
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
    await SeasonCompetitionsClient.createSeasonCompetition(
      seasonCompetition.value
    );

    seasonCompetition.value = clone(props.seasonCompetition);
  } catch (e) {
    console.error(e);
    alert("Error occurred creating season competition");
  }
  selfLoading.value--;
  emit("update");
}

async function update() {
  if (!validate()) return;

  selfLoading.value++;
  try {
    await SeasonCompetitionsClient.updateSeasonCompetition(
      seasonCompetition.value.id!,
      seasonCompetition.value
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred updating season competition");
  }
  selfLoading.value--;
  emit("update");
}

async function remove() {
  if (!confirm(`Really delete season competition?`)) {
    return;
  }

  selfLoading.value++;
  try {
    await SeasonCompetitionsClient.deleteSeasonCompetition(
      seasonCompetition.value.id!
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred deleting season competition");
  }
  selfLoading.value--;
  emit("update");
}
</script>

<template>
  <form v-if="loading == 0" @submit.prevent>
    <label> Season: </label>
    <select v-model="seasonCompetition.season_id">
      <option value=""></option>
      <option
        v-for="season in seasons"
        :key="season.id ?? ''"
        :value="season.id"
      >
        {{ season.name }}
      </option>
    </select>

    <label> Competition: </label>
    <select v-model="seasonCompetition.competition_id">
      <option value=""></option>
      <option
        v-for="competition in competitions"
        :key="competition.id ?? ''"
        :value="competition.id"
      >
        {{ competition.name }}
      </option>
    </select>

    <label> Description: </label>
    <textarea v-model="seasonCompetition.description"></textarea>

    <label> Score Calculator: </label>
    <select v-model="seasonCompetition.score_calculator">
      <option :value="null"></option>
      <option
        v-for="scoreCalculator in scoreCalculators.filter(
          (x) => x.supports_competitions
        )"
        :value="scoreCalculator.id"
      >
        {{ scoreCalculator.name }}
      </option>
    </select>

    <label> Calculator Config: </label>
    <textarea v-model="seasonCompetition.calculator_config"></textarea>

    <label> Enabled: </label>
    <input v-model="seasonCompetition.enabled" type="checkbox" />

    <button v-if="seasonCompetition.id == null" @click="create" type="submit">
      Create
    </button>
    <button v-if="seasonCompetition.id != null" @click="update" type="submit">
      Update
    </button>
    <button v-if="seasonCompetition.id != null" @click="remove">Delete</button>

    <template v-if="seasonCompetition.id != null">
      ID: <code>{{ seasonCompetition.id }}</code>
      <br />
    </template>
  </form>
</template>

<style lang="scss" scoped></style>
