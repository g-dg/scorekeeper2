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
import CompetitionEdit from "@/components/setup/CompetitionEdit.vue";
import CompetitionEventEdit from "@/components/setup/CompetitionEventEdit.vue";
import EventEdit from "@/components/setup/EventEdit.vue";
import GroupEdit from "@/components/setup/GroupEdit.vue";
import GroupParticipationEdit from "@/components/setup/GroupParticipationEdit.vue";
import ScoreCalculatorEdit from "@/components/setup/ScoreCalculatorEdit.vue";
import SeasonCompetitionEdit from "@/components/setup/SeasonCompetitionEdit.vue";
import SeasonEdit from "@/components/setup/SeasonEdit.vue";
import TeamEdit from "@/components/setup/TeamEdit.vue";
import { natcasecmp } from "@/helpers/sort";
import { computed, onMounted, ref, watch } from "vue";

const loading = ref(0);

//#region Score Calculators

const scoreCalculators = ref<ScoreCalculator[]>([]);
const scoreCalculatorsSorted = computed(() =>
  scoreCalculators.value.slice().sort((a, b) => natcasecmp([a.name, b.name]))
);
const scoreCalculatorsFiltered = computed(() =>
  scoreCalculatorsSorted.value.filter((scoreCalculator) => true)
);
const scoreCalculatorsFilteredSelf = computed(() =>
  scoreCalculatorsFiltered.value.filter(
    (scoreCalculator) =>
      filteredScoreCalculator.value == null ||
      scoreCalculator.id == filteredScoreCalculator.value
  )
);
const scoreCalculatorsById = computed(
  () => new Map(scoreCalculators.value.map((x) => [x.id!, x]))
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
const filteredScoreCalculator = ref<string | null>(null);
watch(scoreCalculatorsFiltered, () => {
  filteredScoreCalculator.value = !scoreCalculatorsFiltered.value.some(
    (x) => x.id == filteredScoreCalculator.value
  )
    ? null
    : filteredScoreCalculator.value;
});

//#endregion

//#region Seasons

const seasons = ref<Season[]>([]);
const seasonsSorted = computed(() =>
  seasons.value.slice().sort((a, b) => natcasecmp([a.name, b.name]))
);
const seasonsFiltered = computed(() =>
  seasonsSorted.value.filter(
    (season) =>
      filteredScoreCalculator.value == null ||
      season.score_calculator == filteredScoreCalculator.value
  )
);
const seasonsFilteredSelf = computed(() =>
  seasonsFiltered.value.filter(
    (season) =>
      filteredSeason.value == null || season.id == filteredSeason.value
  )
);
const seasonsById = computed(
  () => new Map(seasons.value.map((x) => [x.id!, x]))
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
const filteredSeason = ref<string | null>(null);
watch(seasonsFiltered, () => {
  filteredSeason.value = !seasonsFiltered.value.some(
    (x) => x.id == filteredSeason.value
  )
    ? null
    : filteredSeason.value;
});

//#endregion

//#region Competitions

const competitions = ref<Competition[]>([]);
const competitionsSorted = computed(() =>
  competitions.value.slice().sort((a, b) => natcasecmp([a.name, b.name]))
);
const competitionsFiltered = computed(() =>
  competitionsSorted.value.filter((competition) => true)
);
const competitionsFilteredSelf = computed(() =>
  competitionsFiltered.value.filter(
    (competition) =>
      filteredCompetition.value == null ||
      competition.id == filteredCompetition.value
  )
);
const competitionsById = computed(
  () => new Map(competitions.value.map((x) => [x.id!, x]))
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
const filteredCompetition = ref<string | null>(null);
watch(competitionsFiltered, () => {
  filteredCompetition.value = !competitionsFiltered.value.some(
    (x) => x.id == filteredCompetition.value
  )
    ? null
    : filteredCompetition.value;
});

//#endregion

//#region Season Competitions

const seasonCompetitions = ref<SeasonCompetition[]>([]);
const seasonCompetitionsSorted = computed(() =>
  seasonCompetitions.value
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
const seasonCompetitionsFiltered = computed(() =>
  seasonCompetitionsSorted.value.filter(
    (seasonCompetition) =>
      (filteredSeason.value == null ||
        seasonCompetition.season_id == filteredSeason.value) &&
      (filteredCompetition.value == null ||
        seasonCompetition.competition_id == filteredCompetition.value) &&
      (filteredScoreCalculator.value == null ||
        seasonCompetition.score_calculator == filteredScoreCalculator.value)
  )
);
const seasonCompetitionsFilteredSelf = computed(() =>
  seasonCompetitionsFiltered.value.filter(
    (seasonCompetition) =>
      filteredSeasonCompetition.value == null ||
      seasonCompetition.id == filteredSeasonCompetition.value
  )
);
const seasonCompetitionsById = computed(
  () => new Map(seasonCompetitions.value.map((x) => [x.id!, x]))
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
const filteredSeasonCompetition = ref<string | null>(null);
watch(seasonCompetitionsFiltered, () => {
  filteredSeasonCompetition.value = !seasonCompetitionsFiltered.value.some(
    (x) => x.id == filteredSeasonCompetition.value
  )
    ? null
    : filteredSeasonCompetition.value;
});

//#endregion

//#region Events

const events = ref<Event[]>([]);
const eventsSorted = computed(() =>
  events.value.slice().sort((a, b) => natcasecmp([a.name, b.name]))
);
const eventsFiltered = computed(() =>
  eventsSorted.value.filter(
    (event) =>
      filteredCompetition.value == null ||
      event.competition_id == filteredCompetition.value
  )
);
const eventsFilteredSelf = computed(() =>
  eventsFiltered.value.filter(
    (event) => filteredEvent.value == null || event.id == filteredEvent.value
  )
);
const eventsById = computed(() => new Map(events.value.map((x) => [x.id!, x])));
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
const filteredEvent = ref<string | null>(null);
watch(eventsFiltered, () => {
  filteredEvent.value = !eventsFiltered.value.some(
    (x) => x.id == filteredEvent.value
  )
    ? null
    : filteredEvent.value;
});

//#endregion

//#region Competition Events

const competitionEvents = ref<CompetitionEvent[]>([]);
const competitionEventsSorted = computed(() =>
  competitionEvents.value
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
const competitionEventsFiltered = computed(() =>
  competitionEventsSorted.value.filter(
    (competitionEvent) =>
      (filteredSeason.value == null ||
        seasonCompetitionsById.value.get(competitionEvent.season_competition_id)
          ?.season_id == filteredSeason.value) &&
      (filteredCompetition.value == null ||
        seasonCompetitionsById.value.get(competitionEvent.season_competition_id)
          ?.competition_id == filteredCompetition.value) &&
      (filteredSeasonCompetition.value == null ||
        competitionEvent.season_competition_id ==
          filteredSeasonCompetition.value) &&
      (filteredEvent.value == null ||
        competitionEvent.event_id == filteredEvent.value) &&
      (filteredScoreCalculator.value == null ||
        competitionEvent.score_calculator == filteredScoreCalculator.value)
  )
);
const competitionEventsFilteredSelf = computed(() =>
  competitionEventsFiltered.value.filter(
    (competitionEvent) =>
      filteredCompetitionEvent.value == null ||
      competitionEvent.id == filteredCompetitionEvent.value
  )
);
const competitionEventsById = computed(
  () => new Map(competitionEvents.value.map((x) => [x.id!, x]))
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
const filteredCompetitionEvent = ref<string | null>(null);
watch(competitionEventsFiltered, () => {
  filteredCompetitionEvent.value = !competitionEventsFiltered.value.some(
    (x) => x.id == filteredCompetitionEvent.value
  )
    ? null
    : filteredCompetitionEvent.value;
});

//#endregion

//#region Groups

const groups = ref<Group[]>([]);
const groupsSorted = computed(() =>
  groups.value.slice().sort((a, b) => natcasecmp([a.name, b.name]))
);
const groupsFiltered = computed(() =>
  groupsSorted.value.filter((group) => true)
);
const groupsFilteredSelf = computed(() =>
  groupsFiltered.value.filter(
    (group) => filteredGroup.value == null || group.id == filteredGroup.value
  )
);
const groupsById = computed(() => new Map(groups.value.map((x) => [x.id!, x])));
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
const filteredGroup = ref<string | null>(null);
watch(groupsFiltered, () => {
  filteredGroup.value = !groupsFiltered.value.some(
    (x) => x.id == filteredGroup.value
  )
    ? null
    : filteredGroup.value;
});

//#endregion

//#region Group Participation

const groupParticipations = ref<GroupParticipation[]>([]);
const groupParticipationsSorted = computed(() =>
  groupParticipations.value
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
const groupParticipationsFiltered = computed(() =>
  groupParticipationsSorted.value.filter(
    (groupParticipation) =>
      (filteredSeason.value == null ||
        groupParticipation.season_id == filteredSeason.value) &&
      (filteredGroup.value == null ||
        groupParticipation.group_id == filteredGroup.value)
  )
);
const groupParticipationsFilteredSelf = computed(() =>
  groupParticipationsFiltered.value.filter(
    (groupParticipation) =>
      filteredGroupParticipation.value == null ||
      groupParticipation.id == filteredGroupParticipation.value
  )
);
const groupParticipationsById = computed(
  () => new Map(groupParticipations.value.map((x) => [x.id!, x]))
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
const filteredGroupParticipation = ref<string | null>(null);
watch(groupParticipationsFiltered, () => {
  filteredGroupParticipation.value = !groupParticipationsFiltered.value.some(
    (x) => x.id == filteredGroupParticipation.value
  )
    ? null
    : filteredGroupParticipation.value;
});

//#endregion

//#region Teams

const teams = ref<Team[]>([]);
const teamsSorted = computed(() =>
  teams.value.slice().sort((a, b) => natcasecmp([a.name, b.name]))
);
const teamsFiltered = computed(() =>
  teamsSorted.value.filter(
    (team) =>
      (filteredSeason.value == null ||
        groupParticipationsById.value.get(team.group_participation_id)
          ?.season_id == filteredSeason.value) &&
      (filteredGroup.value == null ||
        groupParticipationsById.value.get(team.group_participation_id)
          ?.group_id == filteredGroup.value) &&
      (filteredGroupParticipation.value == null ||
        team.group_participation_id == filteredGroupParticipation.value)
  )
);
const teamsFilteredSelf = computed(() =>
  teamsFiltered.value.filter(
    (team) => filteredTeam.value == null || team.id == filteredTeam.value
  )
);
const teamsById = computed(() => new Map(teams.value.map((x) => [x.id!, x])));
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
const filteredTeam = ref<string | null>(null);
watch(teamsFiltered, () => {
  filteredTeam.value = !teamsFiltered.value.some(
    (x) => x.id == filteredTeam.value
  )
    ? null
    : filteredTeam.value;
});

//#endregion

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

function clearFilters() {
  filteredScoreCalculator.value = null;
  filteredSeason.value = null;
  filteredGroup.value = null;
  filteredGroupParticipation.value = null;
  filteredCompetition.value = null;
  filteredSeasonCompetition.value = null;
  filteredTeam.value = null;
  filteredEvent.value = null;
  filteredCompetitionEvent.value = null;
}

function setVisibility(visibility: boolean) {
  showScoreCalculators.value = visibility;
  showSeasons.value = visibility;
  showGroups.value = visibility;
  showGroupParticipations.value = visibility;
  showCompetitions.value = visibility;
  showSeasonCompetitions.value = visibility;
  showTeams.value = visibility;
  showEvents.value = visibility;
  showCompetitionEvents.value = visibility;
}

const showCreateForms = ref(true);
</script>

<template>
  <div>
    <h1>Setup</h1>

    <button @click="load">Reload / Revert</button>

    <button @click="setVisibility(true)">Show All</button>
    <button @click="setVisibility(false)">Hide All</button>
    <label> Show Create Forms: </label>
    <input v-model="showCreateForms" type="checkbox" />

    <br />

    <button @click="clearFilters">Clear Filters</button>

    <label> Score Calculator: </label>
    <select v-model="filteredScoreCalculator">
      <option :value="null"></option>
      <option
        v-for="scoreCalculator in scoreCalculatorsFiltered"
        :key="scoreCalculator.id ?? ''"
        :value="scoreCalculator.id"
      >
        {{ scoreCalculator.name }}
      </option>
    </select>

    <label> Season: </label>
    <select v-model="filteredSeason">
      <option :value="null"></option>
      <option
        v-for="season in seasonsFiltered"
        :key="season.id ?? ''"
        :value="season.id"
      >
        {{ season.name }}
      </option>
    </select>

    <label> Competition: </label>
    <select v-model="filteredCompetition">
      <option :value="null"></option>
      <option
        v-for="competition in competitionsFiltered"
        :key="competition.id ?? ''"
        :value="competition.id"
      >
        {{ competition.name }}
      </option>
    </select>

    <label> Season Competition: </label>
    <select v-model="filteredSeasonCompetition">
      <option :value="null"></option>
      <option
        v-for="seasonCompetition in seasonCompetitionsFiltered"
        :key="seasonCompetition.id ?? ''"
        :value="seasonCompetition.id"
      >
        {{ seasonsById.get(seasonCompetition.season_id)?.name }} -
        {{ competitionsById.get(seasonCompetition.competition_id)?.name }}
      </option>
    </select>

    <label> Event: </label>
    <select v-model="filteredEvent">
      <option :value="null"></option>
      <option
        v-for="event in eventsFiltered"
        :key="event.id ?? ''"
        :value="event.id"
      >
        {{ event.name }}
      </option>
    </select>

    <label> Competition Event: </label>
    <select v-model="filteredCompetitionEvent">
      <option :value="null"></option>
      <option
        v-for="competitionEvent in competitionEventsFiltered"
        :key="competitionEvent.id ?? ''"
        :value="competitionEvent.id"
      >
        {{
          seasonsById.get(
            seasonCompetitionsById.get(competitionEvent.season_competition_id)
              ?.season_id ?? ""
          )?.name
        }}
        -
        {{
          competitionsById.get(
            seasonCompetitionsById.get(competitionEvent.season_competition_id)
              ?.competition_id ?? ""
          )?.name
        }}
        -
        {{ eventsById.get(competitionEvent.event_id)?.name }}
      </option>
    </select>

    <label> Group: </label>
    <select v-model="filteredGroup">
      <option :value="null"></option>
      <option
        v-for="group in groupsFiltered"
        :key="group.id ?? ''"
        :value="group.id"
      >
        {{ group.name }}
      </option>
    </select>

    <label> Group Participation: </label>
    <select v-model="filteredGroupParticipation">
      <option :value="null"></option>
      <option
        v-for="groupParticipation in groupParticipationsFiltered"
        :key="groupParticipation.id ?? ''"
        :value="groupParticipation.id"
      >
        {{ seasonsById.get(groupParticipation.season_id)?.name }} -
        {{ groupsById.get(groupParticipation.group_id)?.name }}
      </option>
    </select>

    <label> Team: </label>
    <select v-model="filteredTeam">
      <option :value="null"></option>
      <option
        v-for="team in teamsFiltered"
        :key="team.id ?? ''"
        :value="team.id"
      >
        {{
          seasonsById.get(
            groupParticipationsById.get(team.group_participation_id)
              ?.season_id ?? ""
          )?.name
        }}
        -
        {{
          groupsById.get(
            groupParticipationsById.get(team.group_participation_id)
              ?.group_id ?? ""
          )?.name
        }}
        -
        {{ team.name }}
      </option>
    </select>

    <h1>Score Calculators</h1>
    <button @click="showScoreCalculators = !showScoreCalculators">
      Show/Hide
    </button>
    <br /><br />
    <template v-if="showScoreCalculators">
      <template v-if="showCreateForms">
        <ScoreCalculatorEdit @update="loadScoreCalculators" />
        <hr />
      </template>
      <template
        v-for="scoreCalculator in scoreCalculatorsFilteredSelf"
        :key="scoreCalculator.id ?? ''"
      >
        <h2>{{ scoreCalculator.name }}</h2>
        <ScoreCalculatorEdit
          :score-calculator="scoreCalculator"
          @update="loadScoreCalculators"
        />
        <hr />
      </template>
    </template>

    <h1>Seasons</h1>
    <button @click="showSeasons = !showSeasons">Show/Hide</button>
    <br /><br />
    <template v-if="showSeasons">
      <template v-if="showCreateForms">
        <SeasonEdit
          :score-calculators="scoreCalculatorsFilteredSelf"
          @update="loadSeasons"
        />
        <hr />
      </template>
      <template v-for="season in seasonsFilteredSelf" :key="season.id ?? ''">
        <h2>{{ season.name }}</h2>
        <SeasonEdit
          :season="season"
          :score-calculators="scoreCalculatorsFilteredSelf"
          @update="loadSeasons"
        />
        <hr />
      </template>
    </template>

    <h1>Competitions</h1>
    <button @click="showCompetitions = !showCompetitions">Show/Hide</button>
    <br /><br />
    <template v-if="showCompetitions">
      <template v-if="showCreateForms">
        <CompetitionEdit @update="loadCompetitions" />
        <hr />
      </template>
      <template
        v-for="competition in competitionsFilteredSelf"
        :key="competition.id ?? ''"
      >
        <h2>
          {{ competition.name }}
        </h2>
        <CompetitionEdit
          :competition="competition"
          @update="loadCompetitions"
        />
        <hr />
      </template>
    </template>

    <h1>Season Competitions</h1>
    <button @click="showSeasonCompetitions = !showSeasonCompetitions">
      Show/Hide
    </button>
    <br /><br />
    <template v-if="showSeasonCompetitions">
      <template v-if="showCreateForms">
        <SeasonCompetitionEdit
          :seasons="seasonsFilteredSelf"
          :competitions="competitionsFilteredSelf"
          :score-calculators="scoreCalculatorsFilteredSelf"
          @update="loadSeasonCompetitions"
        />
        <hr />
      </template>
      <template
        v-for="seasonCompetition in seasonCompetitionsFilteredSelf"
        :key="seasonCompetition.id ?? ''"
      >
        <h2>
          {{ seasonsById.get(seasonCompetition.season_id)?.name }} -
          {{ competitionsById.get(seasonCompetition.competition_id)?.name }}
        </h2>
        <SeasonCompetitionEdit
          :season-competition="seasonCompetition"
          :seasons="seasonsFilteredSelf"
          :competitions="competitionsFilteredSelf"
          :score-calculators="scoreCalculatorsFilteredSelf"
          @update="loadSeasonCompetitions"
        />
        <hr />
      </template>
    </template>

    <h1>Events</h1>
    <button @click="showEvents = !showEvents">Show/Hide</button>
    <br /><br />
    <template v-if="showEvents">
      <template v-if="showCreateForms">
        <EventEdit
          :competitions="competitionsFilteredSelf"
          @update="loadEvents"
        />
        <hr />
      </template>
      <template v-for="event in eventsFilteredSelf" :key="event.id ?? ''">
        <h2>{{ event.name }}</h2>
        <EventEdit
          :event="event"
          :competitions="competitionsFilteredSelf"
          @update="loadEvents"
        />
        <hr />
      </template>
    </template>

    <h1>Competition Events</h1>
    <button @click="showCompetitionEvents = !showCompetitionEvents">
      Show/Hide
    </button>
    <br /><br />
    <template v-if="showCompetitionEvents">
      <template v-if="showCreateForms">
        <CompetitionEventEdit
          :season-competitions="seasonCompetitionsFilteredSelf"
          :events="eventsFilteredSelf"
          :score-calculators="scoreCalculatorsFilteredSelf"
          :season-competition-map="seasonCompetitionsById"
          :season-map="seasonsById"
          :competition-map="competitionsById"
          @update="loadCompetitionEvents"
        />
        <hr />
      </template>
      <template
        v-for="competitionEvent in competitionEventsFilteredSelf"
        :key="competitionEvent.id ?? ''"
      >
        <h2>
          {{
            seasonsById.get(
              seasonCompetitionsById.get(competitionEvent.season_competition_id)
                ?.season_id ?? ""
            )?.name
          }}
          -
          {{
            competitionsById.get(
              seasonCompetitionsById.get(competitionEvent.season_competition_id)
                ?.competition_id ?? ""
            )?.name
          }}
          -
          {{ eventsById.get(competitionEvent.event_id)?.name }}
        </h2>
        <CompetitionEventEdit
          :competition-event="competitionEvent"
          :season-competitions="seasonCompetitionsFilteredSelf"
          :events="eventsFilteredSelf"
          :score-calculators="scoreCalculatorsFilteredSelf"
          :season-competition-map="seasonCompetitionsById"
          :season-map="seasonsById"
          :competition-map="competitionsById"
          @update="loadCompetitionEvents"
        />
        <hr />
      </template>
    </template>

    <h1>Groups</h1>
    <button @click="showGroups = !showGroups">Show/Hide</button>
    <br /><br />
    <template v-if="showGroups">
      <template v-if="showCreateForms">
        <GroupEdit @update="loadGroups" />
        <hr />
      </template>
      <template v-for="group in groupsFilteredSelf" :key="group.id ?? ''">
        <h2>{{ group.name }}</h2>
        <GroupEdit :group="group" @update="loadGroups" />
        <hr />
      </template>
    </template>

    <h1>Group Participations</h1>
    <button @click="showGroupParticipations = !showGroupParticipations">
      Show/Hide
    </button>
    <br /><br />
    <template v-if="showGroupParticipations">
      <template v-if="showCreateForms">
        <GroupParticipationEdit
          :seasons="seasonsFilteredSelf"
          :groups="groupsFilteredSelf"
          @update="loadGroupParticipations"
        />
        <hr />
      </template>
      <template
        v-for="groupParticipation in groupParticipationsFilteredSelf"
        :key="groupParticipation.id ?? ''"
      >
        <h2>
          {{ seasonsById.get(groupParticipation.season_id)?.name }} -
          {{ groupsById.get(groupParticipation.group_id)?.name }}
        </h2>
        <GroupParticipationEdit
          :group-participation="groupParticipation"
          :seasons="seasonsFilteredSelf"
          :groups="groupsFilteredSelf"
          @update="loadGroupParticipations"
        />
        <hr />
      </template>
    </template>

    <h1>Teams</h1>
    <button @click="showTeams = !showTeams">Show/Hide</button>
    <br /><br />
    <template v-if="showTeams">
      <template v-if="showCreateForms">
        <TeamEdit
          :group-participations="groupParticipationsFilteredSelf"
          :group-participation-map="groupParticipationsById"
          :season-map="seasonsById"
          :group-map="groupsById"
          @update="loadTeams"
        />
        <hr />
      </template>
      <template v-for="team in teamsFilteredSelf" :key="team.id ?? ''">
        <h2>
          {{
            seasonsById.get(
              groupParticipationsById.get(team.group_participation_id)
                ?.season_id ?? ""
            )?.name
          }}
          -
          {{
            groupsById.get(
              groupParticipationsById.get(team.group_participation_id)
                ?.group_id ?? ""
            )?.name
          }}
          -
          {{ team.name }}
        </h2>
        <TeamEdit
          :team="team"
          :group-participations="groupParticipationsFilteredSelf"
          :group-participation-map="groupParticipationsById"
          :season-map="seasonsById"
          :group-map="groupsById"
          @update="loadTeams"
        />
        <hr />
      </template>
    </template>
  </div>
</template>

<style lang="scss" scoped></style>
