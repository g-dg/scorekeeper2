use rusqlite::{named_params, OptionalExtension};
use uuid::Uuid;

use crate::{
    database::{groups::Group, Database},
    helpers::errors::GenericError,
};

pub struct GroupsService {
    db: Database,
}

impl GroupsService {
    pub fn new(database: Database) -> Self {
        Self { db: database }
    }

    pub fn get(&self, id: Uuid) -> Option<Group> {
        let db = self.db.get();
        let result: Option<Group> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"groups\" WHERE \"id\" = :id;",
                Group::COLUMNS_SQL
            ))
            .unwrap()
            .query_row(named_params! {":id": id}, |row| Ok(Group::from_row(row)))
            .optional()
            .expect("Error occurred getting group by id from database");

        result
    }

    pub fn list(&self) -> Vec<Group> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!("SELECT {} FROM \"groups\";", Group::COLUMNS_SQL))
            .unwrap()
            .query_map(named_params! {}, |row| Ok(Group::from_row(row)))
            .expect("Error occurred getting all groups from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn create(&self, group: &Group) -> Result<Uuid, GenericError> {
        let id = Uuid::new_v4();

        let db = self.db.get();
        let success = db
            .prepare_cached(&format!(
                "INSERT INTO \"groups\" ({}) VALUES (:id, :name, :description, :enabled);",
                Group::COLUMNS_SQL
            ))
            .unwrap()
            .execute(named_params! {
                ":id": id,
                ":name": group.name,
                ":description": group.description,
                ":enabled": group.enabled,
            })
            .is_ok();

        if success {
            Ok(id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn update(&self, group: &Group) -> Result<Uuid, GenericError> {
        let Some(_existing) = self.get(group.id.unwrap()) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();
        let success = db.prepare_cached("UPDATE \"groups\" SET \"name\" = :name, \"description\" = :description, \"enabled\" = :enabled WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": group.id,
                ":name": group.name,
                ":description": group.description,
                ":enabled": group.enabled,
            })
            .is_ok();

        if success {
            Ok(group.id.unwrap())
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
            .prepare_cached("DELETE FROM \"groups\" WHERE \"id\" = :id;")
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
