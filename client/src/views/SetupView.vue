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
  type ScoreCalculator,
  ScoreCalculatorsClient,
} from "@/api/score_calculators";
import {
  SeasonCompetitionsClient,
  type SeasonCompetition,
} from "@/api/season_competitions";
import { SeasonsClient, type Season } from "@/api/seasons";
import { TeamsClient, type Team } from "@/api/teams";
import ScoreCalculatorEdit from "@/components/setup/ScoreCalculatorEdit.vue";
import { natcasecmp } from "@/helpers/sort";
import { computed, onMounted, ref } from "vue";

const loading = ref(0);

const scoreCalculators = ref<ScoreCalculator[]>([]);
const scoreCalculatorsFiltered = computed(() =>
  scoreCalculators.value.filter((_) => true)
);
const scoreCalculatorsSorted = computed(() =>
  scoreCalculatorsFiltered.value
    .slice()
    .sort((a, b) => natcasecmp([a.name, b.name]))
);
const scoreCalculatorsById = computed(
  () => new Map(scoreCalculators.value.map((x) => [x.id, x]))
);
async function loadScoreCalculators() {
  loading.value++;
  try {
    scoreCalculators.value =
      await ScoreCalculatorsClient.listScoreCalculators();
  } catch (e) {
    console.error(e);
    alert("Error occurred loading score calculators");
  }
  loading.value--;
}
const showScoreCalculators = ref(false);

const seasons = ref<Season[]>([]);
const seasonsFiltered = computed(() => seasons.value.filter((_) => true));
const seasonsSorted = computed(() =>
  seasonsFiltered.value.slice().sort((a, b) => natcasecmp([a.name, b.name]))
);
const seasonsById = computed(
  () => new Map(seasons.value.map((x) => [x.id, x]))
);
async function loadSeasons() {
  loading.value++;
  try {
    seasons.value = await SeasonsClient.listSeasons();
  } catch (e) {
    console.error(e);
    alert("Error occurred loading seasons");
  }
  loading.value--;
}
const showSeasons = ref(false);

const groups = ref<Group[]>([]);
const groupsFiltered = computed(() => groups.value.filter((_) => true));
const groupsSorted = computed(() =>
  groupsFiltered.value.slice().sort((a, b) => natcasecmp([a.name, b.name]))
);
const groupsById = computed(() => new Map(groups.value.map((x) => [x.id, x])));
async function loadGroups() {
  loading.value++;
  try {
    groups.value = await GroupsClient.listGroups();
  } catch (e) {
    console.error(e);
    alert("Error occurred loading groups");
  }
  loading.value--;
}
const showGroups = ref(false);

const groupParticipations = ref<GroupParticipation[]>([]);
const groupParticipationsFiltered = computed(() =>
  groupParticipations.value.filter((_) => true)
);
const groupParticipationsSorted = computed(() =>
  groupParticipationsFiltered.value
    .slice()
    .sort((a, b) =>
      natcasecmp(
        [
          seasonsById.value.get(a.season_id)?.name,
          seasonsById.value.get(b.season_id)?.name,
        ],
        [
          groupsById.value.get(a.group_id)?.name,
          groupsById.value.get(b.group_id)?.name,
        ]
      )
    )
);
const groupParticipationsById = computed(
  () => new Map(groupParticipations.value.map((x) => [x.id, x]))
);
async function loadGroupParticipations() {
  loading.value++;
  try {
    groupParticipations.value =
      await GroupParticipationsClient.listGroupParticipations();
  } catch (e) {
    console.error(e);
    alert("Error occurred loading group participations");
  }
  loading.value--;
}
const showGroupParticipations = ref(false);

const competitions = ref<Competition[]>([]);
const competitionsFiltered = computed(() =>
  competitions.value.filter((_) => true)
);
const competitionsSorted = computed(() =>
  competitionsFiltered.value
    .slice()
    .sort((a, b) => natcasecmp([a.name, b.name]))
);
const competitionsById = computed(
  () => new Map(competitions.value.map((x) => [x.id, x]))
);
async function loadCompetitions() {
  loading.value++;
  try {
    competitions.value = await CompetitionsClient.listCompetitions();
  } catch (e) {
    console.error(e);
    alert("Error occurred loading competitions");
  }
  loading.value--;
}
const showCompetitions = ref(false);

const seasonCompetitions = ref<SeasonCompetition[]>([]);
const seasonCompetitionsFiltered = computed(() =>
  seasonCompetitions.value.filter((_) => true)
);
const seasonCompetitionsSorted = computed(() =>
  seasonCompetitionsFiltered.value
    .slice()
    .sort((a, b) =>
      natcasecmp(
        [
          seasonsById.value.get(a.season_id)?.name,
          seasonsById.value.get(b.season_id)?.name,
        ],
        [
          competitionsById.value.get(a.competition_id)?.name,
          competitionsById.value.get(b.competition_id)?.name,
        ]
      )
    )
);
const seasonCompetitionsById = computed(
  () => new Map(seasonCompetitions.value.map((x) => [x.id, x]))
);
async function loadSeasonCompetitions() {
  loading.value++;
  try {
    seasonCompetitions.value =
      await SeasonCompetitionsClient.listSeasonCompetitions();
  } catch (e) {
    console.error(e);
    alert("Error occurred loading season competitions");
  }
  loading.value--;
}
const showSeasonCompetitions = ref(false);

const teams = ref<Team[]>([]);
const teamsFiltered = computed(() => teams.value.filter((_) => true));
const teamsSorted = computed(() =>
  teamsFiltered.value.slice().sort((a, b) => natcasecmp([a.name, b.name]))
);
const teamsById = computed(() => new Map(teams.value.map((x) => [x.id, x])));
async function loadTeams() {
  loading.value++;
  try {
    teams.value = await TeamsClient.listTeams();
  } catch (e) {
    console.error(e);
    alert("Error occurred loading teams");
  }
  loading.value--;
}
const showTeams = ref(false);

const events = ref<Event[]>([]);
const eventsFiltered = computed(() => events.value.filter((_) => true));
const eventsSorted = computed(() =>
  eventsFiltered.value.slice().sort((a, b) => natcasecmp([a.name, b.name]))
);
const eventsById = computed(() => new Map(events.value.map((x) => [x.id, x])));
async function loadEvents() {
  loading.value++;
  try {
    events.value = await EventsClient.listEvents();
  } catch (e) {
    console.error(e);
    alert("Error occurred loading events");
  }
  loading.value--;
}
const showEvents = ref(false);

const competitionEvents = ref<CompetitionEvent[]>([]);
const competitionEventsFiltered = computed(() =>
  competitionEvents.value.filter((_) => true)
);
const competitionEventsSorted = computed(() =>
  competitionEventsFiltered.value
    .slice()
    .sort((a, b) =>
      natcasecmp(
        [
          seasonsById.value.get(
            seasonCompetitionsById.value.get(a.season_competition_id)
              ?.season_id ?? ""
          )?.name,
          seasonsById.value.get(
            seasonCompetitionsById.value.get(b.season_competition_id)
              ?.season_id ?? ""
          )?.name,
        ],
        [
          competitionsById.value.get(
            seasonCompetitionsById.value.get(a.season_competition_id)
              ?.competition_id ?? ""
          )?.name,
          competitionsById.value.get(
            seasonCompetitionsById.value.get(b.season_competition_id)
              ?.competition_id ?? ""
          )?.name,
        ],
        [
          eventsById.value.get(a.event_id)?.name,
          eventsById.value.get(b.event_id)?.name,
        ]
      )
    )
);
const competitionEventsById = computed(
  () => new Map(competitionEvents.value.map((x) => [x.id, x]))
);
async function loadCompetitionEvents() {
  loading.value++;
  try {
    competitionEvents.value =
      await CompetitionEventsClient.listCompetitionEvents();
  } catch (e) {
    console.error(e);
    alert("Error occurred loading competition events");
  }
  loading.value--;
}
const showCompetitionEvents = ref(false);

async function load() {
  await Promise.all([
    loadScoreCalculators(),
    loadSeasons(),
    loadGroups(),
    loadGroupParticipations(),
    loadCompetitions(),
    loadSeasonCompetitions(),
    loadTeams(),
    loadEvents(),
    loadCompetitionEvents(),
  ]);
}
onMounted(load);
</script>

<template>
  <div>
    <h1>Setup</h1>
  </div>
</template>

<style lang="scss" scoped></style>
