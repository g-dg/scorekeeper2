use rusqlite::{named_params, OptionalExtension};
use uuid::Uuid;

use crate::{
    database::{
        scores::{Score, ScoreType},
        Database,
    },
    helpers::errors::GenericError,
};

pub struct ScoresService {
    db: Database,
}

impl ScoresService {
    pub fn new(database: &Database) -> Self {
        Self {
            db: database.clone(),
        }
    }

    pub fn get(&self, id: Uuid) -> Option<Score> {
        let db = self.db.get();
        let result: Option<Score> = db
            .prepare_cached(&format!(
                "SELECT * FROM ({}) WHERE \"id\" = :id;",
                Score::UNION_SELECT
            ))
            .unwrap()
            .query_row(named_params! {":id": id}, |row| Ok(Score::from_row(row)))
            .optional()
            .expect("Error occurred getting score by id from database");

        result
    }

    pub fn list(&self) -> Vec<Score> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!("SELECT * FROM ({});", Score::UNION_SELECT,))
            .unwrap()
            .query_map(named_params! {}, |row| Ok(Score::from_row(row)))
            .expect("Error occurred getting all scores from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn list_in_competition_event(&self, competition_event_id: Uuid) -> Vec<Score> {
        let db = self.db.get();

        let result = db
            .prepare_cached(&format!(
                "SELECT * FROM ({}) WHERE \"competition_event_id\" = :competition_event_id;",
                Score::UNION_SELECT
            ))
            .unwrap()
            .query_map(
                named_params! {
                    ":competition_event_id": competition_event_id,
                },
                |row| Ok(Score::from_row(row)),
            )
            .expect("Error occurred getting scores for competition event from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn get_for_competition_event_group(
        &self,
        competition_event_id: Uuid,
        group_participation_id: Uuid,
    ) -> Vec<Score> {
        let db = self.db.get();

        let result = db
            .prepare_cached(&format!(
                "SELECT * FROM ({}) WHERE \"competition_event_id\" = :competition_event_id AND \"score_type\" = 'group' AND \"subject_id\" = :group_participation_id;",
                Score::UNION_SELECT
            ))
            .unwrap()
            .query_map(
                named_params! {
                    ":competition_event_id": competition_event_id,
                    ":group_participation_id": group_participation_id,
                },
                |row| Ok(Score::from_row(row)),
            )
            .expect("Error occurred getting scores for competition event from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn get_for_competition_event_team(
        &self,
        competition_event_id: Uuid,
        team_id: Uuid,
    ) -> Vec<Score> {
        let db = self.db.get();

        let result = db
            .prepare_cached(&format!(
                "SELECT * FROM ({}) WHERE \"competition_event_id\" = :competition_event_id AND \"score_type\" = 'team' AND \"subject_id\" = :team_id;",
                Score::UNION_SELECT
            ))
            .unwrap()
            .query_map(
                named_params! {
                    ":competition_event_id": competition_event_id,
                    ":team_id": team_id,
                },
                |row| Ok(Score::from_row(row)),
            )
            .expect("Error occurred getting scores for competition event from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn create(&self, score: &Score) -> Result<Uuid, GenericError> {
        let id = Uuid::new_v4();

        let query = match score.score_type {
            ScoreType::Group => "INSERT INTO \"group_scores\" (\"id\", \"competition_event_id\", \"group_participation_id\", \"score_data\", \"timestamp\", \"valid\", \"disqualified\", \"notes\") VALUES (:id, :competition_event_id, :subject_id, :score_data, :timestamp, :valid, :disqualified, :notes);",
            ScoreType::Team => "INSERT INTO \"team_scores\" (\"id\", \"competition_event_id\", \"team_id\", \"score_data\", \"timestamp\", \"valid\", \"disqualified\", \"notes\") VALUES (:id, :competition_event_id, :subject_id, :score_data, :timestamp, :valid, :disqualified, :notes);",
        };

        let db = self.db.get();
        let success = db
            .prepare_cached(query)
            .unwrap()
            .execute(named_params! {
                ":id": id,
                ":competition_event_id": score.competition_event_id,
                ":subject_id": score.subject_id,
                ":score_data": score.score_data,
                ":timestamp": score.timestamp,
                ":valid": score.valid,
                ":disqualified": score.disqualified,
                ":notes": score.notes,
            })
            .is_ok();

        if success {
            Ok(id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn update(&self, score: &Score) -> Result<Uuid, GenericError> {
        let Some(_existing) = self.get(score.id.unwrap()) else {
            return Err(GenericError::NOT_FOUND);
        };

        let query = match score.score_type {
            ScoreType::Group => "UPDATE \"group_scores\" SET \"competition_event_id\" = :competition_event_id, \"group_participation_id\" = :subject_id, \"score_data\" = :score_data, \"timestamp\" = :timestamp, \"valid\" = :valid, \"disqualified\" = :disqualified, \"notes\" = :notes WHERE \"id\" = :id;",
            ScoreType::Team => "UPDATE \"team_scores\" SET \"competition_event_id\" = :competition_event_id, \"team_id\" = :subject_id, \"score_data\" = :score_data, \"timestamp\" = :timestamp, \"valid\" = :valid, \"disqualified\" = :disqualified, \"notes\" = :notes WHERE \"id\" = :id;",
        };

        let db = self.db.get();
        let success = db
            .prepare_cached(query)
            .unwrap()
            .execute(named_params! {
                ":id": score.id,
                ":competition_event_id": score.competition_event_id,
                ":subject_id": score.subject_id,
                ":score_data": score.score_data,
                ":timestamp": score.timestamp,
                ":valid": score.valid,
                ":disqualified": score.disqualified,
                ":notes": score.notes,
            })
            .is_ok();

        if success {
            Ok(score.id.unwrap())
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn delete(&self, id: Uuid) -> Result<(), GenericError> {
        let Some(_existing) = self.get(id) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();
        let group_success = db
            .prepare_cached("DELETE FROM \"group_scores\" WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": id,
            })
            .is_ok();
        let team_success = db
            .prepare_cached("DELETE FROM \"team_scores\" WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": id,
            })
            .is_ok();

        if group_success && team_success {
            Ok(())
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }
}
