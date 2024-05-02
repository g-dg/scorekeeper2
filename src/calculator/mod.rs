pub mod decimal;

use mlua::{Lua, Value as LuaValue};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::{score_calculators::ScoreCalculator, Database},
    services::{
        competition_events::CompetitionEventsService,
        group_participation::GroupParticipationsService, groups::GroupsService,
        score_calculators::ScoreCalculatorsService, season_competitions::SeasonCompetitionsService,
        seasons::SeasonsService, teams::TeamsService,
    },
};

use self::decimal::DecimalValue;

#[derive(Clone, Serialize, Deserialize)]
pub enum ScoreResult {
    Group {
        group_participation_id: Uuid,
        group_name: String,
        rank: Option<Decimal>,
    },
    Team {
        team_id: Uuid,
        group_name: String,
        team_name: String,
        rank: Option<Decimal>,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EventResult {
    pub event_name: String,
    pub results: Vec<ScoreResult>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CompetitionResult {
    pub competition_name: String,
    pub results: Vec<ScoreResult>,
    pub events: Vec<EventResult>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SeasonResult {
    pub season_name: String,
    pub results: Vec<ScoreResult>,
    pub competitions: Vec<CompetitionResult>,
}

/// Score results calculator
///
/// Score calculation scripts:
///
/// Team/group results for competition event
/// Lua function: `calculate_event_scores`
/// Parameter 1: array of teams/groups with each item in the format:
///   {
///     "id": <team/group id>,
///     "type": <either "Team" or "Group">
///     "scores": [ // sorted newest to oldest, invalid ones are excluded, nil values for items with disqualified flag set
///       {
///         // score data object
///       }
///     ]
///   }
/// Parameter 2: object of score config options
/// Return: array of teams/groups, ordering doesn't matter, with each item in the format:
///   {
///     "id": <team/group id>,
///     "event_score": <number, will be ranked highest to lowest, ties are allowed, nil items will be put at the bottom and flagged>,
///     "competition_data": <anything, will be passed to the competition score calculator>
///   }
///
///
/// Team/group results for season competition:
/// Lua function: `calculate_competition_scores`
/// Parameter 1: array of teams/groups with each item in the format:
///   {
///     "id": <team/group id>,
///     "data": [ // array of data passed from each calculator for the events in this competition
///       <data from competition event calculators>
///     ]
///   }
/// Parameter 2: object of score config options
/// Return: array of teams/groups, ordering doesn't matter, with each item in the format:
///   {
///     "id": <team/group id>,
///     "competition_score": <number, will be ranked highest to lowest, ties are allowed, nil items will be put at the bottom and flagged>,
///     "season_data": <anything, will be passed to the season score calculator>
///   }
///
///
/// Group results for season:
/// Lua function: `calculate_season_scores`
/// Parameter 1: array of groups with each item in the format:
///   {
///     "id": <group id>,
///     "data": [ // array of data passed from each calculator for the competitions in this season (note that this includes all entries for each team that belongs to this group)
///       <data from season competition calculators>
///     ],
///   }
/// Parameter 2: object of score config options
/// Return: array of groups, ordering doesn't matter, with each item in the format:
///   {
///     "id": <group_id>,
///     "season_score": <number, will be ranked highest to lowest, ties are allowed, nil items will be put at the bottom and flagged>,
///   }
///
pub struct ResultsCalculator {
    pub lua: Lua,
    score_calculator_service: ScoreCalculatorsService,
    season_service: SeasonsService,
    season_competition_service: SeasonCompetitionsService,
    competition_event_service: CompetitionEventsService,
    group_service: GroupsService,
    group_participation_service: GroupParticipationsService,
    team_service: TeamsService,
}

impl ResultsCalculator {
    pub fn new(database: &Database) -> Self {
        let lua = Lua::new();

        DecimalValue::add_constructor(&lua, "decimal");

        Self {
            lua,
            score_calculator_service: ScoreCalculatorsService::new(database),
            season_service: SeasonsService::new(database),
            season_competition_service: SeasonCompetitionsService::new(database),
            competition_event_service: CompetitionEventsService::new(database),
            group_service: GroupsService::new(database),
            group_participation_service: GroupParticipationsService::new(database),
            team_service: TeamsService::new(database),
        }
    }

    pub fn calculate_team_event_score(
        &self,
        competition_event_id: Uuid,
        team_id: Uuid,
    ) -> (Option<Decimal>, LuaValue) {
        let competition_event = self
            .competition_event_service
            .get(competition_event_id)
            .expect("Error occurred getting competition event");
        let score_calculator = competition_event
            .score_calculator
            .and_then(|score_calculator| self.score_calculator_service.get(score_calculator))
            .unwrap_or_else(ScoreCalculator::get_default);
        let team = self
            .team_service
            .get(team_id)
            .expect("Error occurred getting team");
        let group = self
            .group_service
            .get(
                self.group_participation_service
                    .get(team.group_participation_id)
                    .expect("Error occurred getting group participation")
                    .group_id,
            )
            .expect("Error occurred getting group");

        todo!()
    }

    pub fn calculate_group_event_score(
        &self,
        competition_event_id: Uuid,
        group_participation_id: Uuid,
    ) -> (Option<Decimal>, LuaValue) {
        let competition_event = self
            .competition_event_service
            .get(competition_event_id)
            .expect("Error occurred getting competition event");
        let score_calculator = competition_event
            .score_calculator
            .and_then(|score_calculator| self.score_calculator_service.get(score_calculator))
            .unwrap_or_else(ScoreCalculator::get_default);
        let group = self
            .group_service
            .get(
                self.group_participation_service
                    .get(group_participation_id)
                    .expect("Error occurred getting group participation")
                    .group_id,
            )
            .expect("Error occurred getting group");

        todo!()
    }

    pub fn calculate_team_competition_score(
        &self,
        season_competition_id: Uuid,
        team_id: Uuid,
        event_values: &[LuaValue],
    ) -> (Option<Decimal>, LuaValue) {
        let season_competition = self
            .season_competition_service
            .get(season_competition_id)
            .expect("Error occurred getting season competition");
        let score_calculator = season_competition
            .score_calculator
            .and_then(|score_calculator| self.score_calculator_service.get(score_calculator))
            .unwrap_or_else(ScoreCalculator::get_default);
        let team = self
            .team_service
            .get(team_id)
            .expect("Error occurred getting team");
        let group = self
            .group_service
            .get(
                self.group_participation_service
                    .get(team.group_participation_id)
                    .expect("Error occurred getting group participation")
                    .group_id,
            )
            .expect("Error occurred getting group");

        todo!()
    }

    pub fn calculate_group_competition_score(
        &self,
        season_competition_id: Uuid,
        group_participation_id: Uuid,
        event_values: &[LuaValue],
    ) -> (Option<Decimal>, LuaValue) {
        let season_competition = self
            .season_competition_service
            .get(season_competition_id)
            .expect("Error occurred getting season competition");
        let score_calculator = season_competition
            .score_calculator
            .and_then(|score_calculator| self.score_calculator_service.get(score_calculator))
            .unwrap_or_else(ScoreCalculator::get_default);
        let group = self
            .group_service
            .get(
                self.group_participation_service
                    .get(group_participation_id)
                    .expect("Error occurred getting group participation")
                    .group_id,
            )
            .expect("Error occurred getting group");

        todo!()
    }

    pub fn calculate_group_season_score(
        &self,
        season_id: Uuid,
        group_participation_id: Uuid,
        competition_values: &[LuaValue],
    ) -> Option<Decimal> {
        let season = self
            .season_service
            .get(season_id)
            .expect("Error occurred getting season");
        let score_calculator = season
            .score_calculator
            .and_then(|score_calculator| self.score_calculator_service.get(score_calculator))
            .unwrap_or_else(ScoreCalculator::get_default);
        let group = self
            .group_service
            .get(
                self.group_participation_service
                    .get(group_participation_id)
                    .expect("Error occurred getting group participation")
                    .group_id,
            )
            .expect("Error occurred getting group");

        todo!()
    }
}
