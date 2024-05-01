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
import { ScoresClient, type Score } from "@/api/scores";
import {
  SeasonCompetitionsClient,
  type SeasonCompetition,
} from "@/api/season_competitions";
import { SeasonsClient, type Season } from "@/api/seasons";
import { TeamsClient, type Team } from "@/api/teams";
import ScoreData from "@/components/score_overview/ScoreData.vue";
import { natcasecmp } from "@/helpers/sort";
import { computed, onMounted, ref, watch } from "vue";

const loading = ref(0);

const seasons = ref<Season[]>([]);
const seasonsById = computed(
  () => new Map(seasons.value.map((season) => [season.id, season]))
);
async function loadSeasons() {
  loading.value++;

  try {
    seasons.value = (await SeasonsClient.listSeasons()).filter(
      (season) => season.enabled
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading seasons");
  }

  loading.value--;
}

const competitions = ref<Competition[]>([]);
const competitionsById = computed(
  () =>
    new Map(
      competitions.value.map((competition) => [competition.id, competition])
    )
);
async function loadCompetitions() {
  loading.value++;

  try {
    competitions.value = (await CompetitionsClient.listCompetitions()).filter(
      (competition) => competition.enabled
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading competitions");
  }

  loading.value--;
}

const seasonCompetitions = ref<SeasonCompetition[]>([]);
const seasonCompetitionsById = computed(
  () =>
    new Map(
      seasonCompetitions.value.map((seasonCompetition) => [
        seasonCompetition.id,
        seasonCompetition,
      ])
    )
);
async function loadSeasonCompetitions() {
  loading.value++;

  try {
    seasonCompetitions.value = (
      await SeasonCompetitionsClient.listSeasonCompetitions()
    )
      .filter((seasonCompetition) => seasonCompetition.enabled)
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
      );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading season competitions");
  }

  loading.value--;
}

const events = ref<Event[]>([]);
const eventsById = computed(
  () => new Map(events.value.map((event) => [event.id, event]))
);
async function loadEvents() {
  loading.value++;

  try {
    events.value = (await EventsClient.listEvents())
      .filter((event) => event.enabled)
      .sort((a, b) =>
        natcasecmp(
          [
            competitionsById.value.get(a.competition_id)?.name ?? "",
            competitionsById.value.get(b.competition_id)?.name ?? "",
          ],
          [a.name, b.name]
        )
      );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading events");
  }

  loading.value--;
}

const competitionEvents = ref<CompetitionEvent[]>([]);
const competitionEventsById = computed(
  () =>
    new Map(
      competitionEvents.value.map((competitionEvent) => [
        competitionEvent.id,
        competitionEvent,
      ])
    )
);
async function loadCompetitionEvents() {
  loading.value++;

  try {
    competitionEvents.value = (
      await CompetitionEventsClient.listCompetitionEvents()
    )
      .filter((competitionEvent) => competitionEvent.enabled)
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
      );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading competition events");
  }

  loading.value--;
}

const groups = ref<Group[]>([]);
const groupsById = computed(
  () => new Map(groups.value.map((group) => [group.id, group]))
);
async function loadGroups() {
  loading.value++;

  try {
    groups.value = (await GroupsClient.listGroups())
      .filter((group) => group.enabled)
      .sort((a, b) => natcasecmp([a.name, b.name]));
  } catch (e) {
    console.error(e);
    alert("Error occurred loading groups");
  }

  loading.value--;
}

const groupParticipations = ref<GroupParticipation[]>([]);
const groupParticipationsById = computed(
  () =>
    new Map(
      groupParticipations.value.map((groupParticipation) => [
        groupParticipation.id,
        groupParticipation,
      ])
    )
);
async function loadGroupParticipations() {
  loading.value++;

  try {
    groupParticipations.value = (
      await GroupParticipationsClient.listGroupParticipations()
    )
      .filter((groupParticipation) => groupParticipation.enabled)
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
      );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading group participations");
  }

  loading.value--;
}

const teams = ref<Team[]>([]);
const teamsById = computed(
  () => new Map(teams.value.map((team) => [team.id, team]))
);
async function loadTeams() {
  loading.value++;

  try {
    teams.value = (await TeamsClient.listTeams())
      .filter((team) => team.enabled)
      .sort((a, b) =>
        natcasecmp(
          [
            seasonsById.value.get(
              groupParticipationsById.value.get(a.group_participation_id)
                ?.season_id ?? ""
            )?.name,
            seasonsById.value.get(
              groupParticipationsById.value.get(b.group_participation_id)
                ?.season_id ?? ""
            )?.name,
          ],
          [
            groupsById.value.get(
              groupParticipationsById.value.get(a.group_participation_id)
                ?.group_id ?? ""
            )?.name,
            groupsById.value.get(
              groupParticipationsById.value.get(b.group_participation_id)
                ?.group_id ?? ""
            )?.name,
          ],
          [a.name, b.name]
        )
      );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading teams");
  }

  loading.value--;
}

const allScores = ref<Score[]>([]);
const allScoresById = computed(
  () => new Map(allScores.value.map((score) => [score.id, score]))
);
async function loadScores() {
  loading.value++;

  try {
    allScores.value = (await ScoresClient.listScores()).filter(
      (score) => score.valid
    );
  } catch (e) {
    console.error(e);
    alert("Error occurred loading scores");
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
    loadScores(),
  ]);
}
onMounted(loadAll);

const selectedSeasonId = ref<string | null>(null);
watch(seasons, () => {
  if (!seasons.value.some((season) => season.id == selectedSeasonId.value))
    selectedSeasonId.value = null;
});

const filteredSeasonCompetitions = computed(() =>
  seasonCompetitions.value.filter(
    (seasonCompetition) => seasonCompetition.season_id == selectedSeasonId.value
  )
);
const selectedSeasonCompetitionId = ref<string | null>(null);
watch(filteredSeasonCompetitions, () => {
  if (
    !filteredSeasonCompetitions.value.some(
      (seasonCompetition) =>
        seasonCompetition.id == selectedSeasonCompetitionId.value
    )
  )
    selectedSeasonCompetitionId.value = null;
});

const filteredCompetitionEvents = computed(() =>
  competitionEvents.value.filter(
    (competitionEvent) =>
      competitionEvent.season_competition_id ==
      selectedSeasonCompetitionId.value
  )
);

const filteredGroupParticipations = computed(() =>
  groupParticipations.value.filter(
    (groupParticipation) =>
      groupParticipation.season_id == selectedSeasonId.value
  )
);

const filteredTeams = computed(() =>
  teams.value.filter((team) =>
    filteredGroupParticipations.value.some(
      (groupParticipation) =>
        groupParticipation.id == team.group_participation_id
    )
  )
);

const showScoreData = ref(false);

const scoresBySubjectId = computed(() =>
  allScores.value.reduce((map, score) => {
    const scores = map.get(score.subject_id);
    if (scores != undefined) {
      scores.push(score);
    } else {
      map.set(score.subject_id, [score]);
    }
    return map;
  }, new Map<string, Score[]>())
);

function getGroupScores(
  competitionEventId: string,
  groupParticipationId: string
): Score[] {
  return (
    scoresBySubjectId.value
      .get(groupParticipationId)
      ?.filter(
        (score) =>
          score.competition_event_id == competitionEventId &&
          score.score_type == "Group" &&
          score.valid
      ) ?? []
  );
}

function getTeamScores(competitionEventId: string, teamId: string): Score[] {
  return (
    scoresBySubjectId.value
      .get(teamId)!
      ?.filter(
        (score) =>
          score.competition_event_id == competitionEventId &&
          score.score_type == "Team" &&
          score.valid
      ) ?? []
  );
}
</script>

<template>
  <main>
    <button @click="loadScores">Reload Scores</button>
    <button @click="loadAll">Reload All</button>

    <label for="season"> Season: </label>
    <select v-model="selectedSeasonId" id="season">
      <option v-for="season in seasons" :key="season.id!" :value="season.id">
        {{ season.name }}
      </option>
    </select>

    <label for="competition"> Competition: </label>
    <select
      v-model="selectedSeasonCompetitionId"
      id="competition"
      :disabled="selectedSeasonId == null"
    >
      <option
        v-for="seasonCompetition in filteredSeasonCompetitions"
        :key="seasonCompetition.id!"
        :value="seasonCompetition.id"
      >
        {{ competitionsById.get(seasonCompetition.competition_id)?.name }}
      </option>
    </select>

    <label for="show_score_data"> Show Score Data: </label>
    <input v-model="showScoreData" type="checkbox" id="show_score_data" />

    <br /><br />

    <table class="score-overview">
      <thead></thead>
      <tbody>
        <tr>
          <td></td>
          <td
            v-for="competitionEvent in filteredCompetitionEvents"
            :key="competitionEvent.id!"
          >
            {{ eventsById.get(competitionEvent.event_id)?.name }}
          </td>
        </tr>

        <template
          v-for="groupParticipation in filteredGroupParticipations"
          :key="groupParticipation.id"
        >
          <tr
            v-if="
              filteredCompetitionEvents.some(
                (competitionEvent) => competitionEvent.score_type == 'Group'
              )
            "
          >
            <td>{{ groupsById.get(groupParticipation.group_id)?.name }}</td>
            <td
              v-for="competitionEvent in filteredCompetitionEvents"
              :key="competitionEvent.id!"
              :class="{
                'score-count': true,
                'no-score': competitionEvent.score_type == 'Group' && getGroupScores(competitionEvent.id!, groupParticipation.id!).length == 0,
                'has-score': competitionEvent.score_type == 'Group' && getGroupScores(competitionEvent.id!, groupParticipation.id!).length > 0
              }"
            >
              <template v-if="competitionEvent.score_type == 'Group'">
                <template v-if="showScoreData">
                  <ScoreData
                    :scores="getGroupScores(competitionEvent.id!, groupParticipation.id!)"
                  />
                </template>
                <template v-else>
                  {{
                    getGroupScores(competitionEvent.id!, groupParticipation.id!)
                      .length
                  }}
                </template>
              </template>
            </td>
          </tr>

          <tr
            v-if="
              filteredCompetitionEvents.some(
                (competitionEvent) => competitionEvent.score_type == 'Team'
              )
            "
            v-for="team in teams.filter(
              (team) => team.group_participation_id == groupParticipation.id
            )"
            :key="team.id!"
          >
            <td>
              {{ groupsById.get(groupParticipation.group_id)?.name }} -
              {{ team.name }}
            </td>
            <td
              v-for="competitionEvent in filteredCompetitionEvents"
              :key="competitionEvent.id!"
              :class="{
                'score-count': true,
                'no-score': competitionEvent.score_type == 'Team' && getTeamScores(competitionEvent.id!, team.id!).length == 0,
                'has-score': competitionEvent.score_type == 'Team' && getTeamScores(competitionEvent.id!, team.id!).length > 0
              }"
            >
              <template v-if="competitionEvent.score_type == 'Team'">
                <template v-if="showScoreData">
                  <ScoreData
                    :scores="getTeamScores(competitionEvent.id!, team.id!)"
                  />
                </template>
                <template v-else>
                  {{ getTeamScores(competitionEvent.id!, team.id!).length }}
                </template>
              </template>
            </td>
          </tr>
        </template>
      </tbody>
    </table>
  </main>
</template>

<style lang="scss" scoped>
table.score-overview {
  border-collapse: collapse;
  th,
  td {
    border: 1px #000 solid;
    padding: 0.25em;
  }
  .score-count {
    text-align: center;
    &.no-score {
      background-color: #ffc7c7;
    }
    &.has-score {
      background-color: #9eff9e;
    }
  }
}
</style>
