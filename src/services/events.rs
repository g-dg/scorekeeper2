use rusqlite::{named_params, OptionalExtension};
use uuid::Uuid;

use crate::{
    database::{events::Event, Database},
    helpers::errors::GenericError,
};

pub struct EventsService {
    db: Database,
}

impl EventsService {
    pub fn new(database: &Database) -> Self {
        Self {
            db: database.clone(),
        }
    }

    pub fn get(&self, id: Uuid) -> Option<Event> {
        let db = self.db.get();
        let result: Option<Event> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"events\" WHERE \"id\" = :id;",
                Event::COLUMNS_SQL
            ))
            .unwrap()
            .query_row(named_params! {":id": id}, |row| Ok(Event::from_row(row)))
            .optional()
            .expect("Error occurred getting event by id from database");

        result
    }

    pub fn list(&self) -> Vec<Event> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!("SELECT {} FROM \"events\";", Event::COLUMNS_SQL))
            .unwrap()
            .query_map(named_params! {}, |row| Ok(Event::from_row(row)))
            .expect("Error occurred getting all events from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    /// Returns enabled events in specified competition
    pub fn list_in_competition(&self, competition_id: Uuid) -> Vec<Event> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"events\" WHERE \"competition_id\" = :competition_id AND \"enabled\" != 0;",
                Event::COLUMNS_SQL
            ))
            .unwrap()
            .query_map(named_params! {
                ":competition_id": competition_id,
            }, |row| Ok(Event::from_row(row)))
            .expect("Error occurred getting events in competition from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn create(&self, event: &Event) -> Result<Uuid, GenericError> {
        let id = Uuid::new_v4();

        let db = self.db.get();
        let success = db
            .prepare_cached(&format!(
                "INSERT INTO \"events\" ({}) VALUES (:id, :competition_id, :name, :description, :enabled);",
                Event::COLUMNS_SQL
            ))
            .unwrap()
            .execute(named_params! {
                ":id": id,
                ":competition_id": event.competition_id,
                ":name": event.name,
                ":description": event.description,
                ":enabled": event.enabled,
            })
            .is_ok();

        if success {
            Ok(id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn update(&self, event: &Event) -> Result<Uuid, GenericError> {
        let Some(_existing) = self.get(event.id.unwrap()) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();
        let success = db.prepare_cached("UPDATE \"events\" SET \"competition_id\" = :competition_id, \"name\" = :name, \"description\" = :description, \"enabled\" = :enabled WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": event.id,
                ":competition_id": event.competition_id,
                ":name": event.name,
                ":description": event.description,
                ":enabled": event.enabled,
            })
            .is_ok();

        if success {
            Ok(event.id.unwrap())
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
            .prepare_cached("DELETE FROM \"events\" WHERE \"id\" = :id;")
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
