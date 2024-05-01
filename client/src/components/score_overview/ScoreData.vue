<script setup lang="ts">
import type { Score } from "@/api/scores";
import { computed } from "vue";

const props = defineProps<{
  scores: Score[];
}>();
const scores = computed(() =>
  props.scores.map((score) => {
    try {
      return {
        disqualified: score.disqualified,
        data: JSON.parse(score.score_data),
      };
    } catch {
      return null;
    }
  })
);
</script>

<template>
  <div v-for="score in scores" class="score-data">
    <div v-if="score == null"><em>Error</em></div>
    <div v-else-if="score!.disqualified"><strong>Disqualified</strong></div>
    <div v-else v-for="(value, key) in score.data">{{ key }}: {{ value }}</div>
  </div>
</template>

<style lang="scss" scoped>
.score-data {
  margin: .5em;
}
</style>
