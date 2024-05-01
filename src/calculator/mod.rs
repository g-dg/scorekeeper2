pub mod decimal;
pub mod script_context;

use uuid::Uuid;

use crate::database::Database;

pub struct ResultsCalculator {
    db: Database,
}

impl ResultsCalculator {
    /// Calculates team/group scores for a competition event
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
    pub fn get_results_for_competition_event(&self, competition_event_id: Uuid) {
        todo!()
    }

    /// Calculate team/group scores for season competition
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
    pub fn get_results_for_season_competition(&self, season_competition_id: Uuid) {
        todo!()
    }

    /// Calculate group scores for season competition
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
    pub fn get_results_for_season(&self, season_id: Uuid) {
        todo!()
    }
}
