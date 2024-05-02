use rusqlite::{named_params, OptionalExtension};
use uuid::Uuid;

use crate::{
    database::{competitions::Competition, Database},
    helpers::errors::GenericError,
};

pub struct CompetitionsService {
    db: Database,
}

impl CompetitionsService {
    pub fn new(database: &Database) -> Self {
        Self {
            db: database.clone(),
        }
    }

    pub fn get(&self, id: Uuid) -> Option<Competition> {
        let db = self.db.get();
        let result: Option<Competition> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"competitions\" WHERE \"id\" = :id;",
                Competition::COLUMNS_SQL
            ))
            .unwrap()
            .query_row(named_params! {":id": id}, |row| {
                Ok(Competition::from_row(row))
            })
            .optional()
            .expect("Error occurred getting competition by id from database");

        result
    }

    pub fn list(&self) -> Vec<Competition> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"competitions\";",
                Competition::COLUMNS_SQL
            ))
            .unwrap()
            .query_map(named_params! {}, |row| Ok(Competition::from_row(row)))
            .expect("Error occurred getting all competitions from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn create(&self, competition: &Competition) -> Result<Uuid, GenericError> {
        let id = Uuid::new_v4();

        let db = self.db.get();
        let success = db
            .prepare_cached(&format!(
                "INSERT INTO \"competitions\" ({}) VALUES (:id, :name, :description, :enabled);",
                Competition::COLUMNS_SQL
            ))
            .unwrap()
            .execute(named_params! {
                ":id": id,
                ":name": competition.name,
                ":description": competition.description,
                ":enabled": competition.enabled,
            })
            .is_ok();

        if success {
            Ok(id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn update(&self, competition: &Competition) -> Result<Uuid, GenericError> {
        let Some(_existing) = self.get(competition.id.unwrap()) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();
        let success = db.prepare_cached("UPDATE \"competitions\" SET \"name\" = :name, \"description\" = :description, \"enabled\" = :enabled WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": competition.id,
                ":name": competition.name,
                ":description": competition.description,
                ":enabled": competition.enabled,
            })
            .is_ok();

        if success {
            Ok(competition.id.unwrap())
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
            .prepare_cached("DELETE FROM \"competitions\" WHERE \"id\" = :id;")
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
