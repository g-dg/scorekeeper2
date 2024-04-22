use rusqlite::{named_params, OptionalExtension};
use uuid::Uuid;

use crate::{
    database::{seasons::Season, Database},
    helpers::errors::GenericError,
};

pub struct SeasonsService {
    db: Database,
}

impl SeasonsService {
    pub fn new(database: Database) -> Self {
        Self { db: database }
    }

    pub fn get(&self, id: Uuid) -> Option<Season> {
        let db = self.db.get();
        let result: Option<Season> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"seasons\" WHERE \"id\" = :id;",
                Season::COLUMNS_SQL
            ))
            .unwrap()
            .query_row(named_params! {":id": id}, |row| Ok(Season::from_row(row)))
            .optional()
            .expect("Error occurred getting season by id from database");

        result
    }

    pub fn list(&self) -> Vec<Season> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!("SELECT {} FROM \"seasons\";", Season::COLUMNS_SQL))
            .unwrap()
            .query_map(named_params! {}, |row| Ok(Season::from_row(row)))
            .expect("Error occurred getting all seasons from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn create(&self, season: &Season) -> Result<Uuid, GenericError> {
        let id = Uuid::new_v4();

        let db = self.db.get();
        let success = db
            .prepare_cached(&format!(
                "INSERT INTO \"seasons\" ({}) VALUES (:id, :name, :description, :score_calculator, :enabled);",
                Season::COLUMNS_SQL
            ))
            .unwrap()
            .execute(named_params! {
                ":id": id,
                ":name": season.name,
                ":description": season.description,
                ":score_calculator": season.score_calculator,
                ":enabled": season.enabled,
            })
            .is_ok();

        if success {
            Ok(id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn update(&self, season: &Season) -> Result<Uuid, GenericError> {
        let Some(_existing) = self.get(season.id.unwrap()) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();
        let success = db.prepare_cached("UPDATE \"seasons\" SET \"name\" = :name, \"description\" = :description, \"score_calculator\" = :score_calculator, \"enabled\" = :enabled WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": season.id,
                ":name": season.name,
                ":description": season.description,
                ":score_calculator": season.score_calculator,
                ":enabled": season.enabled,
            })
            .is_ok();

        if success {
            Ok(season.id.unwrap())
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
            .prepare_cached("DELETE FROM \"seasons\" WHERE \"id\" = :id;")
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
