<script lang="ts">
const defaultScoreCalculator: ScoreCalculator = {
  id: null,
  name: "",
  description: "",
  script: "",
  config_options: "{}",
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

async function create() {
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

    <label> Config Options: </label>
    <textarea v-model="scoreCalculator.config_options"></textarea>

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
