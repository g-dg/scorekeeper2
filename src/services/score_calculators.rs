use rusqlite::{named_params, OptionalExtension};
use uuid::Uuid;

use crate::{
    database::{score_calculators::ScoreCalculator, Database},
    helpers::errors::GenericError,
};

pub struct ScoreCalculatorsService {
    db: Database,
}

impl ScoreCalculatorsService {
    pub fn new(database: Database) -> Self {
        Self { db: database }
    }

    pub fn get(&self, id: Uuid) -> Option<ScoreCalculator> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"score_calculators\" WHERE \"id\" = :id;",
                ScoreCalculator::COLUMNS_SQL
            ))
            .unwrap()
            .query_row(named_params! {":id": id}, |row| {
                Ok(ScoreCalculator::from_row(row))
            })
            .optional()
            .expect("Error occurred getting score calculator by id from database");

        result
    }

    pub fn list(&self) -> Vec<ScoreCalculator> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"score_calculators\";",
                ScoreCalculator::COLUMNS_SQL
            ))
            .unwrap()
            .query_map(named_params! {}, |row| Ok(ScoreCalculator::from_row(row)))
            .expect("Error occurred getting all score calculators from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn create(&self, score_calculator: &ScoreCalculator) -> Result<Uuid, GenericError> {
        let id = Uuid::new_v4();

        let db = self.db.get();
        let success = db
            .prepare_cached(&format!(
                "INSERT INTO \"score_calculators\" ({}) VALUES (:id, :name, :description, :script, :default_config, :supports_seasons, :supports_competitions, :supports_events, :score_fields);",
                ScoreCalculator::COLUMNS_SQL
            ))
            .unwrap()
            .execute(named_params! {
                ":id": id,
                ":name": score_calculator.name,
                ":description": score_calculator.description,
                ":script": score_calculator.script,
                ":default_config": score_calculator.default_config,
                ":supports_seasons": score_calculator.supports_seasons,
                ":supports_competitions": score_calculator.supports_competitions,
                ":supports_events": score_calculator.supports_events,
                ":score_fields": score_calculator.score_fields,
            })
            .is_ok();

        if success {
            Ok(id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn update(&self, score_calculator: &ScoreCalculator) -> Result<Uuid, GenericError> {
        let Some(_existing) = self.get(score_calculator.id.unwrap()) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();
        let success = db.prepare_cached("UPDATE \"score_calculators\" SET \"name\" = :name, \"description\" = :description, \"script\" = :script, \"default_config\" = :default_config, \"supports_seasons\" = :supports_seasons, \"supports_competitions\" = :supports_competitions, \"supports_events\" = :supports_events, \"score_fields\" = :score_fields WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": score_calculator.id,
                ":name": score_calculator.name,
                ":description": score_calculator.description,
                ":script": score_calculator.script,
                ":default_config": score_calculator.default_config,
                ":supports_seasons": score_calculator.supports_seasons,
                ":supports_competitions": score_calculator.supports_competitions,
                ":supports_events": score_calculator.supports_events,
                ":score_fields": score_calculator.score_fields,
            })
            .is_ok();

        if success {
            Ok(score_calculator.id.unwrap())
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
            .prepare_cached("DELETE FROM \"score_calculators\" WHERE \"id\" = :id;")
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
