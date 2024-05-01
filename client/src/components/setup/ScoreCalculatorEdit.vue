<script lang="ts">
const defaultScoreCalculator: ScoreCalculator = {
  id: null,
  name: "",
  description: "",
  script: "",
  default_config: "{}",
  supports_seasons: false,
  supports_competitions: false,
  supports_events: false,
  score_fields: null,
};
</script>

<script setup lang="ts">
import {
  ScoreCalculatorsClient,
  type ScoreCalculator,
} from "@/api/score_calculators";
import clone from "@/helpers/clone";
import { computed, onMounted, ref, watch, type PropType } from "vue";

const props = defineProps({
  loading: {
    type: Number,
    default: 0,
  },
  scoreCalculator: {
    type: Object as PropType<ScoreCalculator>,
    default: () => clone(defaultScoreCalculator),
  },
});

const scoreCalculator = ref<ScoreCalculator>(clone(props.scoreCalculator));
watch(
  computed(() => props.scoreCalculator),
  () => (scoreCalculator.value = clone(props.scoreCalculator))
);
onMounted(() => (scoreCalculator.value = clone(props.scoreCalculator)));

const emit = defineEmits<{
  (e: "update"): void;
}>();

const selfLoading = ref(0);
const loading = computed(() => props.loading + selfLoading.value);

function validate() {
  try {
    const config = JSON.parse(scoreCalculator.value.default_config);
    if (typeof config != "object" || Array.isArray(config)) throw new Error();
    scoreCalculator.value.default_config = JSON.stringify(config);
  } catch {
    alert("Default config must be a valid JSON object");
    return false;
  }

  if ((scoreCalculator.value.score_fields ?? "") != "") {
    try {
      const config = JSON.parse(scoreCalculator.value.score_fields!);
      if (typeof config != "object" || Array.isArray(config)) throw new Error();
      scoreCalculator.value.score_fields = JSON.stringify(config);
    } catch {
      alert("Score fields must be a valid JSON object");
      return false;
    }
  } else {
    scoreCalculator.value.score_fields = null;
  }

  return true;
}

async function create() {
  if (!validate()) return;

  selfLoading.value++;
  try {
    await ScoreCalculatorsClient.createScoreCalculator(scoreCalculator.value);

    scoreCalculator.value = clone(props.scoreCalculator);
  } catch (e) {
    console.error(e);
    alert("Error occurred creating score calculator");
  }
  selfLoading.value--;
  emit("update");
}

async function update() {
  if (!validate()) return;

  selfLoading.value++;
  try {
    await ScoreCalculatorsClient.updateScoreCalculator(
      scoreCalculator.value.id!,
      scoreCalculator.value
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred updating score calculator");
  }
  selfLoading.value--;
  emit("update");
}

async function remove() {
  if (
    !confirm(`Really delete score calculator "${scoreCalculator.value.name}"?`)
  ) {
    return;
  }

  selfLoading.value++;
  try {
    await ScoreCalculatorsClient.deleteScoreCalculator(
      scoreCalculator.value.id!
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred deleting score calculator");
  }
  selfLoading.value--;
  emit("update");
}
</script>

<template>
  <form v-if="loading == 0" @submit.prevent>
    <label> Name: </label>
    <input v-model="scoreCalculator.name" type="text" />

    <label> Description: </label>
    <textarea v-model="scoreCalculator.description"></textarea>

    <label> Script: </label>
    <textarea v-model="scoreCalculator.script"></textarea>

    <label> Default Config: </label>
    <textarea v-model="scoreCalculator.default_config"></textarea>

    <label> Supports Seasons: </label>
    <input v-model="scoreCalculator.supports_seasons" type="checkbox" />

    <label> Supports Competitions: </label>
    <input v-model="scoreCalculator.supports_competitions" type="checkbox" />

    <label> Supports Events: </label>
    <input v-model="scoreCalculator.supports_events" type="checkbox" />

    <label> Score Fields: </label>
    <textarea v-model="scoreCalculator.score_fields"></textarea>

    <button v-if="scoreCalculator.id == null" @click="create" type="submit">
      Create
    </button>
    <button v-if="scoreCalculator.id != null" @click="update" type="submit">
      Update
    </button>
    <button v-if="scoreCalculator.id != null" @click="remove">Delete</button>

    <template v-if="scoreCalculator.id != null">
      ID: <code>{{ scoreCalculator.id }}</code>
      <br />
    </template>
  </form>
</template>

<style lang="scss" scoped></style>
