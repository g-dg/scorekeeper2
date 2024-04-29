use rusqlite::{named_params, OptionalExtension};
use uuid::Uuid;

use crate::{
    database::{competition_events::CompetitionEvent, Database},
    helpers::errors::GenericError,
};

pub struct CompetitionEventsService {
    db: Database,
}

impl CompetitionEventsService {
    pub fn new(database: Database) -> Self {
        Self { db: database }
    }

    pub fn get(&self, id: Uuid) -> Option<CompetitionEvent> {
        let db = self.db.get();
        let result: Option<CompetitionEvent> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"competition_events\" WHERE \"id\" = :id;",
                CompetitionEvent::COLUMNS_SQL
            ))
            .unwrap()
            .query_row(named_params! {":id": id}, |row| {
                Ok(CompetitionEvent::from_row(row))
            })
            .optional()
            .expect("Error occurred getting competition event by id from database");

        result
    }

    pub fn list(&self) -> Vec<CompetitionEvent> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"competition_events\";",
                CompetitionEvent::COLUMNS_SQL
            ))
            .unwrap()
            .query_map(named_params! {}, |row| Ok(CompetitionEvent::from_row(row)))
            .expect("Error occurred getting all competition events from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    /// Returns enabled competition events in specified season competition
    pub fn list_in_season_competition(&self, season_competition_id: Uuid) -> Vec<CompetitionEvent> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"competition_events\" WHERE \"season_competition_id\" = :season_competition_id AND \"enabled\" != 0;",
                CompetitionEvent::COLUMNS_SQL
            ))
            .unwrap()
            .query_map(named_params! {
                ":season_competition_id": season_competition_id,
            }, |row| Ok(CompetitionEvent::from_row(row)))
            .expect("Error occurred getting all competition events from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn create(&self, competition_event: &CompetitionEvent) -> Result<Uuid, GenericError> {
        let id = Uuid::new_v4();

        let db = self.db.get();
        let success = db
            .prepare_cached(&format!(
                "INSERT INTO \"competition_events\" ({}) VALUES (:id, :season_competition_id, :event_id, :description, :score_calculator, :calculator_config, :enabled, :score_type);",
                CompetitionEvent::COLUMNS_SQL
            ))
            .unwrap()
            .execute(named_params! {
                ":id": id,
                ":season_competition_id": competition_event.season_competition_id,
                ":event_id": competition_event.event_id,
                ":description": competition_event.description,
                ":score_calculator": competition_event.score_calculator,
                ":calculator_config": competition_event.calculator_config,
                ":enabled": competition_event.enabled,
                ":score_type": competition_event.score_type,
            })
            .is_ok();

        if success {
            Ok(id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn update(&self, competition_event: &CompetitionEvent) -> Result<Uuid, GenericError> {
        let Some(_existing) = self.get(competition_event.id.unwrap()) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();
        let success = db.prepare_cached("UPDATE \"competition_events\" SET \"season_competition_id\" = :season_competition_id, \"event_id\" = :event_id, \"description\" = :description, \"score_calculator\" = :score_calculator, \"calculator_config\" = :calculator_config, \"enabled\" = :enabled, \"score_type\" = :score_type WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": competition_event.id,
                ":season_competition_id": competition_event.season_competition_id,
                ":event_id": competition_event.event_id,
                ":description": competition_event.description,
                ":score_calculator": competition_event.score_calculator,
                ":calculator_config": competition_event.calculator_config,
                ":enabled": competition_event.enabled,
                ":score_type": competition_event.score_type,
            })
            .is_ok();

        if success {
            Ok(competition_event.id.unwrap())
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
            .prepare_cached("DELETE FROM \"competition_events\" WHERE \"id\" = :id;")
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
