<script setup lang="ts">
import {
  CompetitionEventsClient,
  type CompetitionEvent,
} from "@/api/competition_events";
import { CompetitionsClient, type Competition } from "@/api/competitions";

import { EventsClient, type Event } from "@/api/events";
import {
  GroupParticipationsClient,
  type GroupParticipation,
} from "@/api/group_participation";
import { GroupsClient, type Group } from "@/api/groups";
import {
  ScoreCalculatorsClient,
  type ScoreCalculator,
  type ScoreFieldType,
} from "@/api/score_calculators";
import { ScoresClient, type Score } from "@/api/scores";
import {
  SeasonCompetitionsClient,
  type SeasonCompetition,
} from "@/api/season_competitions";
import { SeasonsClient, type Season } from "@/api/seasons";
import { TeamsClient, type Team } from "@/api/teams";
import clone from "@/helpers/clone";
import { natcasecmp } from "@/helpers/sort";
import { computed, onMounted, ref, watch } from "vue";

const DEFAULT_SCORE_FIELDS: Record<string, ScoreFieldType> = {
  Points: "Number",
};

//#region Initial Load

const loading = ref(0);

const allScoreCalculators = ref<ScoreCalculator[]>([]);
async function loadScoreCalculators() {
  loading.value++;

  try {
    allScoreCalculators.value =
      await ScoreCalculatorsClient.listScoreCalculators();
  } catch (e) {
    console.error(e);
    alert("Error occurred loading score calculators");
  }

  loading.value--;
}

const allSeasons = ref<Season[]>([]);
async function loadSeasons() {
  loading.value++;

  try {
    allSeasons.value = (await SeasonsClient.listSeasons()).filter(
      (season) => season.enabled
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading seasons");
  }

  loading.value--;
}

const allCompetitions = ref<Competition[]>([]);
async function loadCompetitions() {
  loading.value++;

  try {
    allCompetitions.value = (
      await CompetitionsClient.listCompetitions()
    ).filter((competition) => competition.enabled);
  } catch (e) {
    console.error(e);
    alert("Error occurred loading competitions");
  }

  loading.value--;
}

const allSeasonCompetitions = ref<SeasonCompetition[]>([]);
async function loadSeasonCompetitions() {
  loading.value++;

  try {
    allSeasonCompetitions.value = (
      await SeasonCompetitionsClient.listSeasonCompetitions()
    ).filter((seasonCompetition) => seasonCompetition.enabled);
  } catch (e) {
    console.error(e);
    alert("Error occurred loading season competitions");
  }

  loading.value--;
}

const allEvents = ref<Event[]>([]);
async function loadEvents() {
  loading.value++;

  try {
    allEvents.value = (await EventsClient.listEvents()).filter(
      (event) => event.enabled
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading events");
  }

  loading.value--;
}

const allCompetitionEvents = ref<CompetitionEvent[]>([]);
async function loadCompetitionEvents() {
  loading.value++;

  try {
    allCompetitionEvents.value = (
      await CompetitionEventsClient.listCompetitionEvents()
    ).filter((competitionEvent) => competitionEvent.enabled);
  } catch (e) {
    console.error(e);
    alert("Error occurred loading competition events");
  }

  loading.value--;
}

const allGroups = ref<Group[]>([]);
async function loadGroups() {
  loading.value++;

  try {
    allGroups.value = (await GroupsClient.listGroups()).filter(
      (group) => group.enabled
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading groups");
  }

  loading.value--;
}

const allGroupParticipations = ref<GroupParticipation[]>([]);
async function loadGroupParticipations() {
  loading.value++;

  try {
    allGroupParticipations.value = (
      await GroupParticipationsClient.listGroupParticipations()
    ).filter((groupParticipation) => groupParticipation.enabled);
  } catch (e) {
    console.error(e);
    alert("Error occurred loading group participations");
  }

  loading.value--;
}

const allTeams = ref<Team[]>([]);
async function loadTeams() {
  loading.value++;

  try {
    allTeams.value = (await TeamsClient.listTeams()).filter(
      (team) => team.enabled
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading teams");
  }

  loading.value--;
}

async function loadAll() {
  await Promise.all([
    loadScoreCalculators(),
    loadSeasons(),
    loadCompetitions(),
    loadSeasonCompetitions(),
    loadEvents(),
    loadCompetitionEvents(),
    loadGroups(),
    loadGroupParticipations(),
    loadTeams(),
  ]);
}
onMounted(loadAll);

//#endregion

//#region Id Mappings

const scoreCalculatorsById = computed(
  () => new Map(allScoreCalculators.value.map((x) => [x.id!, x]))
);

const seasonsById = computed(
  () => new Map(allSeasons.value.map((x) => [x.id!, x]))
);

const competitionsById = computed(
  () => new Map(allCompetitions.value.map((x) => [x.id!, x]))
);

const seasonCompetitionsById = computed(
  () => new Map(allSeasonCompetitions.value.map((x) => [x.id!, x]))
);

const eventsById = computed(
  () => new Map(allEvents.value.map((x) => [x.id!, x]))
);

const competitionEventsById = computed(
  () => new Map(allCompetitionEvents.value.map((x) => [x.id!, x]))
);

const groupsById = computed(
  () => new Map(allGroups.value.map((x) => [x.id!, x]))
);

const groupParticipationsById = computed(
  () => new Map(allGroupParticipations.value.map((x) => [x.id!, x]))
);

const teamsById = computed(
  () => new Map(allTeams.value.map((x) => [x.id!, x]))
);

//#endregion

//#region Filtering

const seasons = computed(() =>
  allSeasons.value
    .filter((season) => true)
    .sort((a, b) => natcasecmp([a.name, b.name]))
);
const selectedSeasonId = ref<string | null>(null);
watch(seasons, () => {
  if (!seasons.value.some((season) => selectedSeasonId.value == season.id)) {
    selectedSeasonId.value = null;
  }
});

const seasonCompetitions = computed(() =>
  allSeasonCompetitions.value
    .filter(
      (seasonCompetition) =>
        seasonCompetition.season_id == selectedSeasonId.value
    )
    .sort((a, b) =>
      natcasecmp([
        competitionsById.value.get(a.competition_id)!.name,
        competitionsById.value.get(b.competition_id)!.name,
      ])
    )
);
const selectedSeasonCompetitionId = ref<string | null>(null);
watch(seasonCompetitions, () => {
  if (
    !seasonCompetitions.value.some(
      (seasonCompetition) =>
        selectedSeasonCompetitionId.value == seasonCompetition.id
    )
  ) {
    selectedSeasonCompetitionId.value = null;
  }
});

const competitionEvents = computed(() =>
  allCompetitionEvents.value
    .filter(
      (competitionEvent) =>
        competitionEvent.season_competition_id ==
        selectedSeasonCompetitionId.value
    )
    .sort((a, b) =>
      natcasecmp([
        eventsById.value.get(a.event_id)!.name,
        eventsById.value.get(b.event_id)!.name,
      ])
    )
);
const selectedCompetitionEventId = ref<string | null>(null);
watch(competitionEvents, () => {
  if (
    !competitionEvents.value.some(
      (competitionEvent) =>
        selectedCompetitionEventId.value == competitionEvent.id
    )
  ) {
    selectedCompetitionEventId.value = null;
  }
});

const groupParticipations = computed(() =>
  allGroupParticipations.value
    .filter(
      (groupParticipation) =>
        groupParticipation.season_id == selectedSeasonId.value
    )
    .sort((a, b) =>
      natcasecmp([
        groupsById.value.get(a.group_id)!.name,
        groupsById.value.get(b.group_id)!.name,
      ])
    )
);
const selectedGroupParticipationId = ref<string | null>(null);
watch(groupParticipations, () => {
  if (
    !groupParticipations.value.some(
      (groupParticipation) =>
        selectedGroupParticipationId.value == groupParticipation.id
    )
  ) {
    selectedGroupParticipationId.value = null;
  }
});

const teams = computed(() =>
  allTeams.value
    .filter(
      (team) =>
        team.group_participation_id == selectedGroupParticipationId.value
    )
    .sort((a, b) => natcasecmp([a.name, b.name]))
);
const selectedTeamId = ref<string | null>(null);
watch(teams, () => {
  if (!teams.value.some((team) => selectedTeamId.value == team.id)) {
    selectedTeamId.value = null;
  }
});

//#endregion

const scoreType = computed(
  () =>
    competitionEventsById.value.get(selectedCompetitionEventId.value ?? "")
      ?.score_type
);

const existingScores = ref<Score[]>([]);
async function loadScores() {
  let scoreTypeValid = true;
  switch (scoreType.value) {
    case "Group":
      if (
        selectedCompetitionEventId.value == null ||
        selectedGroupParticipationId.value == null
      )
        scoreTypeValid = false;
      break;
    case "Team":
      if (
        selectedCompetitionEventId.value == null ||
        selectedTeamId.value == null
      )
        scoreTypeValid = false;
      break;
    default:
      scoreTypeValid = false;
  }

  if (!scoreTypeValid) {
    newScore.value = null;
    return;
  }

  loading.value++;

  try {
    existingScores.value = [];
    switch (scoreType.value) {
      case "Group":
        existingScores.value = (
          await ScoresClient.getScoresForGroup(
            selectedGroupParticipationId.value!,
            selectedCompetitionEventId.value!
          )
        ).filter((score) => score.valid && score.score_type == "Group");
        break;
      case "Team":
        existingScores.value = (
          await ScoresClient.getScoresForTeam(
            selectedTeamId.value!,
            selectedCompetitionEventId.value!
          )
        ).filter((score) => score.valid && score.score_type == "Team");
        break;
    }

    existingScores.value = existingScores.value.sort((a, b) =>
      a.timestamp == b.timestamp ? 0 : a.timestamp < b.timestamp ? 1 : -1
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading existing scores");
  }

  newScore.value = clone(blankScore.value);

  loading.value--;
}

watch(
  () => ({
    competitionEventId: selectedCompetitionEventId.value,
    groupParticipationId: selectedGroupParticipationId.value,
    teamId: selectedTeamId.value,
  }),
  loadScores
);

const currentCompetitionEventScoreCalculator = computed(
  () =>
    scoreCalculatorsById.value.get(
      competitionEventsById.value.get(selectedCompetitionEventId.value ?? "")
        ?.score_calculator ?? ""
    ) ?? null
);

const newScore = ref<Record<string, string> | null>(null);

const scoreFields = computed(() => {
  try {
    const scoreFields = JSON.parse(
      currentCompetitionEventScoreCalculator.value?.score_fields ??
        JSON.stringify(DEFAULT_SCORE_FIELDS)
    ) as Record<string, ScoreFieldType>;
    return scoreFields;
  } catch (e) {
    console.error(e);
    alert("Error occurred parsing score field config");
  }
  return clone(DEFAULT_SCORE_FIELDS);
});
const blankScore = computed(() => {
  let blankScore: Record<string, any> = {};
  for (const [key, value] of Object.entries(scoreFields.value)) {
    switch (value) {
      case "Number":
        blankScore[key] = null;
        break;
      case "Time":
        blankScore[key] = "";
        break;
      case "Boolean":
        blankScore[key] = false;
        break;
    }
  }

  return blankScore;
});

async function saveNewScore() {
  loading.value++;

  try {
    const subjectId =
      scoreType.value == "Group"
        ? selectedGroupParticipationId.value
        : scoreType.value == "Team"
        ? selectedTeamId.value
        : null;
    const scoreData = JSON.stringify(newScore.value);
    const score: Score = {
      id: null,
      competition_event_id: selectedCompetitionEventId.value!,
      score_type: scoreType.value!,
      subject_id: subjectId!,
      score_data: scoreData,
      timestamp: new Date().toISOString(),
      valid: true,
      disqualified: false,
      notes: null,
    };

    await ScoresClient.createScore(score);
  } catch (e) {
    console.error(e);
    alert("Error occurred saving score");
  }

  await loadScores();

  loading.value--;
}

async function deleteExistingScore(scoreId: string) {
  if (!confirm("Really delete score?")) return;

  loading.value++;

  try {
    const score = existingScores.value.find((score) => score.id == scoreId)!;
    score.valid = false;

    await ScoresClient.updateScore(scoreId, score);
  } catch (e) {
    console.error(e);
    alert("Error occurred deleting score");
  }

  await loadScores();

  loading.value--;
}

function parseScoreData(scoreData: string) {
  try {
    return JSON.parse(scoreData);
  } catch {
    return {};
  }
}
</script>

<template>
  <form @submit.prevent>
    <label for="season"> Season: </label>
    <select
      v-model="selectedSeasonId"
      :disabled="seasons.length == 0 || loading > 0"
      id="season"
    >
      <option :value="null"></option>
      <option v-for="season in seasons" :key="season.id!" :value="season.id">
        {{ season.name }}
      </option>
    </select>

    <br />

    <label for="competition"> Competition: </label>
    <select
      v-model="selectedSeasonCompetitionId"
      :disabled="seasonCompetitions.length == 0 || loading > 0"
      id="competition"
    >
      <option :value="null"></option>
      <option
        v-for="seasonCompetition in seasonCompetitions"
        :key="seasonCompetition.id!"
        :value="seasonCompetition.id"
      >
        {{ competitionsById.get(seasonCompetition.competition_id)!.name }}
      </option>
    </select>

    <br />

    <label for="event"> Event: </label>
    <select
      v-model="selectedCompetitionEventId"
      :disabled="competitionEvents.length == 0 || loading > 0"
      id="event"
    >
      <option :value="null"></option>
      <option
        v-for="competitionEvent in competitionEvents"
        :key="competitionEvent.id!"
        :value="competitionEvent.id"
      >
        {{ eventsById.get(competitionEvent.event_id)!.name }}
      </option>
    </select>

    <br />

    <label for="group"> Group: </label>
    <select
      v-model="selectedGroupParticipationId"
      :disabled="groupParticipations.length == 0 || loading > 0"
      id="group"
    >
      <option :value="null"></option>
      <option
        v-for="groupParticipation in groupParticipations"
        :key="groupParticipation.id!"
        :value="groupParticipation.id"
      >
        {{ groupsById.get(groupParticipation.group_id)!.name }}
      </option>
    </select>

    <br />

    <label for="team"> Team: </label>
    <select
      v-model="selectedTeamId"
      :disabled="teams.length == 0 || scoreType != 'Team' || loading > 0"
      id="team"
    >
      <option :value="null"></option>
      <option v-for="team in teams" :key="team.id!" :value="team.id">
        {{ team.name }}
      </option>
    </select>

    <br />

    <template v-if="newScore != null" v-for="(_, key) in newScore" :key="key">
      {{ key }}:
      <input v-model="newScore[key]" type="text" />
      <br />
    </template>

    <button
      @click="saveNewScore"
      type="submit"
      :disabled="newScore == null || loading > 0"
    >
      Save
    </button>

    <br /><br />

    <template v-for="score in existingScores" :key="score.id">
      <template
        v-for="(value, key) in parseScoreData(score.score_data)"
        :key="key"
      >
        {{ key }}: {{ value }}
        <br />
      </template>
      <button @click="deleteExistingScore(score.id!)" :disabled="loading > 0">
        Delete
      </button>
      <br /><br />
    </template>

    <em v-if="loading"> Loading... </em>
  </form>
</template>

<style scoped lang="scss"></style>
