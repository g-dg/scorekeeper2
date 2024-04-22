use rusqlite::{named_params, OptionalExtension};
use uuid::Uuid;

use crate::{
    database::{season_competitions::SeasonCompetition, Database},
    helpers::errors::GenericError,
};

pub struct SeasonCompetitionsService {
    db: Database,
}

impl SeasonCompetitionsService {
    pub fn new(database: Database) -> Self {
        Self { db: database }
    }

    pub fn get(&self, id: Uuid) -> Option<SeasonCompetition> {
        let db = self.db.get();
        let result: Option<SeasonCompetition> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"season_competitions\" WHERE \"id\" = :id;",
                SeasonCompetition::COLUMNS_SQL
            ))
            .unwrap()
            .query_row(named_params! {":id": id}, |row| {
                Ok(SeasonCompetition::from_row(row))
            })
            .optional()
            .expect("Error occurred getting season competition by id from database");

        result
    }

    pub fn list(&self) -> Vec<SeasonCompetition> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"season_competitions\";",
                SeasonCompetition::COLUMNS_SQL
            ))
            .unwrap()
            .query_map(named_params! {}, |row| Ok(SeasonCompetition::from_row(row)))
            .expect("Error occurred getting all season competitions from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    /// Returns enabled season competitions in specified season
    pub fn list_in_season(&self, season_id: Uuid) -> Vec<SeasonCompetition> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"season_competitions\" WHERE \"season_id\" = :season_id AND \"enabled\" != 0;",
                SeasonCompetition::COLUMNS_SQL
            ))
            .unwrap()
            .query_map(named_params! {
                ":season_id": season_id,
            }, |row| Ok(SeasonCompetition::from_row(row)))
            .expect("Error occurred getting season competitions in season from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn create(&self, season_competition: &SeasonCompetition) -> Result<Uuid, GenericError> {
        let id = Uuid::new_v4();

        let db = self.db.get();
        let success = db
            .prepare_cached(&format!(
                "INSERT INTO \"season_competitions\" ({}) VALUES (:id, :season_id, :competition_id, :description, :score_calculator, :enabled);",
                SeasonCompetition::COLUMNS_SQL
            ))
            .unwrap()
            .execute(named_params! {
                ":id": id,
                ":season_id": season_competition.season_id,
                ":competition_id": season_competition.competition_id,
                ":description": season_competition.description,
                ":score_calculator": season_competition.score_calculator,
                ":enabled": season_competition.enabled,
            })
            .is_ok();

        if success {
            Ok(id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn update(&self, season_competition: &SeasonCompetition) -> Result<Uuid, GenericError> {
        let Some(_existing) = self.get(season_competition.id.unwrap()) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();
        let success = db.prepare_cached("UPDATE \"season_competitions\" SET \"season_id\" = :season_id, \"competition_id\" = :competition_id, \"description\" = :description, \"score_calculator\" = :score_calculator, \"enabled\" = :enabled WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": season_competition.id,
                ":season_id": season_competition.season_id,
                ":competition_id": season_competition.competition_id,
                ":description": season_competition.description,
                ":score_calculator": season_competition.score_calculator,
                ":enabled": season_competition.enabled,
            })
            .is_ok();

        if success {
            Ok(season_competition.id.unwrap())
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn delete(&self, id: Uuid) -> Result<(), GenericError> {
        let Some(_existing) = self.get(id) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();
        let success = db
            .prepare_cached("DELETE FROM \"season_competitions\" WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": id,
            })
            .is_ok();

        if success {
            Ok(())
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }
}
