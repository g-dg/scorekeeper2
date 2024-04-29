<script lang="ts">
const defaultCompetitionEvent: CompetitionEvent = {
  id: null,
  season_competition_id: "",
  event_id: "",
  description: "",
  score_calculator: null,
  calculator_config: "{}",
  enabled: true,
  score_type: "Team",
};
</script>

<script setup lang="ts">
import {
  CompetitionEventsClient,
  type CompetitionEvent,
} from "@/api/competition_events";
import type { Competition } from "@/api/competitions";
import type { Event } from "@/api/events";
import type { ScoreCalculator } from "@/api/score_calculators";
import type { SeasonCompetition } from "@/api/season_competitions";
import type { Season } from "@/api/seasons";
import clone from "@/helpers/clone";
import { computed, onMounted, ref, watch, type PropType } from "vue";

const props = defineProps({
  loading: {
    type: Number,
    default: 0,
  },
  competitionEvent: {
    type: Object as PropType<CompetitionEvent>,
    default: () => clone(defaultCompetitionEvent),
  },
  seasonCompetitions: {
    type: Array as PropType<SeasonCompetition[]>,
    required: true,
  },
  events: {
    type: Array as PropType<Event[]>,
    required: true,
  },
  scoreCalculators: {
    type: Array as PropType<ScoreCalculator[]>,
    required: true,
  },
  seasonCompetitionMap: {
    type: Map as PropType<Map<string, SeasonCompetition>>,
    required: true,
  },
  seasonMap: {
    type: Map as PropType<Map<string, Season>>,
    required: true,
  },
  competitionMap: {
    type: Map as PropType<Map<string, Competition>>,
    required: true,
  },
});

const competitionEvent = ref<CompetitionEvent>(clone(props.competitionEvent));
watch(
  computed(() => props.competitionEvent),
  () => (competitionEvent.value = clone(props.competitionEvent))
);
onMounted(() => (competitionEvent.value = clone(props.competitionEvent)));

const emit = defineEmits<{
  (e: "update"): void;
}>();

const selfLoading = ref(0);
const loading = computed(() => props.loading + selfLoading.value);

async function create() {
  selfLoading.value++;
  try {
    await CompetitionEventsClient.createCompetitionEvent(
      competitionEvent.value
    );

    competitionEvent.value = clone(props.competitionEvent);
  } catch (e) {
    console.error(e);
    alert("Error occurred creating competition event");
  }
  selfLoading.value--;
  emit("update");
}

async function update() {
  selfLoading.value++;
  try {
    await CompetitionEventsClient.updateCompetitionEvent(
      competitionEvent.value.id!,
      competitionEvent.value
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred updating competition event");
  }
  selfLoading.value--;
  emit("update");
}

async function remove() {
  if (!confirm(`Really delete competition event?`)) {
    return;
  }

  selfLoading.value++;
  try {
    await CompetitionEventsClient.deleteCompetitionEvent(
      competitionEvent.value.id!
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred deleting competition event");
  }
  selfLoading.value--;
  emit("update");
}
</script>

<template>
  <form v-if="loading == 0" @submit.prevent>
    <label> Season Competition: </label>
    <select v-model="competitionEvent.season_competition_id">
      <option value=""></option>
      <option
        v-for="seasonCompetition in seasonCompetitions"
        :key="seasonCompetition.id ?? ''"
        :value="seasonCompetition.id"
      >
        {{
          seasonMap.get(
            seasonCompetitionMap.get(seasonCompetition.id ?? "")?.season_id ??
              ""
          )?.name
        }}
        -
        {{
          competitionMap.get(
            seasonCompetitionMap.get(seasonCompetition.id ?? "")
              ?.competition_id ?? ""
          )?.name
        }}
      </option>
    </select>

    <label> Event: </label>
    <select v-model="competitionEvent.event_id">
      <option value=""></option>
      <option v-for="event in events" :key="event.id ?? ''" :value="event.id">
        {{ event.name }}
      </option>
    </select>

    <label> Description: </label>
    <textarea v-model="competitionEvent.description"></textarea>

    <label> Score Calculator: </label>
    <select v-model="competitionEvent.score_calculator">
      <option value=""></option>
      <option
        v-for="scoreCalculator in scoreCalculators.filter(
          (x) => x.supports_events
        )"
        :key="scoreCalculator.id ?? ''"
        :value="scoreCalculator.id"
      >
        {{ scoreCalculator.name }}
      </option>
    </select>

    <label> Calculator Config: </label>
    <textarea v-model="competitionEvent.calculator_config"></textarea>

    <label> Enabled: </label>
    <input v-model="competitionEvent.enabled" type="checkbox" />

    <label> Score Type: </label>
    <select v-model="competitionEvent.score_type">
      <option value="Group">Group</option>
      <option value="Team">Team</option>
    </select>

    <button v-if="competitionEvent.id == null" @click="create" type="submit">
      Create
    </button>
    <button v-if="competitionEvent.id != null" @click="update" type="submit">
      Update
    </button>
    <button v-if="competitionEvent.id != null" @click="remove">Delete</button>

    <template v-if="competitionEvent.id != null">
      ID: <code>{{ competitionEvent.id }}</code>
      <br />
    </template>
  </form>
</template>

<style lang="scss" scoped></style>