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
  SeasonCompetitionsClient,
  type SeasonCompetition,
} from "@/api/season_competitions";
import { SeasonsClient, type Season } from "@/api/seasons";
import { TeamsClient, type Team } from "@/api/teams";
import { computed, onMounted, ref } from "vue";

//#region Initial Load

const loading = ref(0);

const allSeasons = ref<Season[]>([]);
async function loadSeasons() {
  loading.value++;

  try {
    allSeasons.value = await SeasonsClient.listSeasons();
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
    allCompetitions.value = await CompetitionsClient.listCompetitions();
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
    allSeasonCompetitions.value =
      await SeasonCompetitionsClient.listSeasonCompetitions();
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
    allEvents.value = await EventsClient.listEvents();
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
    allCompetitionEvents.value =
      await CompetitionEventsClient.listCompetitionEvents();
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
    allGroups.value = await GroupsClient.listGroups();
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
    allGroupParticipations.value =
      await GroupParticipationsClient.listGroupParticipations();
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
    allTeams.value = await TeamsClient.listTeams();
  } catch (e) {
    console.error(e);
    alert("Error occurred loading teams");
  }

  loading.value--;
}

async function loadAll() {
  await Promise.all([
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
</script>

<template>
  <div></div>
</template>

<style scoped lang="scss"></style>
