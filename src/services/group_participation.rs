use rusqlite::{named_params, OptionalExtension};
use uuid::Uuid;

use crate::{
    database::{group_participation::GroupParticipation, Database},
    helpers::errors::GenericError,
};

pub struct GroupParticipationsService {
    db: Database,
}

impl GroupParticipationsService {
    pub fn new(database: Database) -> Self {
        Self { db: database }
    }

    pub fn get(&self, id: Uuid) -> Option<GroupParticipation> {
        let db = self.db.get();
        let result: Option<GroupParticipation> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"group_participation\" WHERE \"id\" = :id;",
                GroupParticipation::COLUMNS_SQL
            ))
            .unwrap()
            .query_row(named_params! {":id": id}, |row| {
                Ok(GroupParticipation::from_row(row))
            })
            .optional()
            .expect("Error occurred getting group participation by id from database");

        result
    }

    pub fn list(&self) -> Vec<GroupParticipation> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"group_participation\";",
                GroupParticipation::COLUMNS_SQL
            ))
            .unwrap()
            .query_map(named_params! {}, |row| Ok(GroupParticipation::from_row(row)))
            .expect("Error occurred getting all group participations from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    /// Returns enabled group participations in specified season
    pub fn list_in_season(&self, season_id: Uuid) -> Vec<GroupParticipation> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"group_participation\" WHERE \"season_id\" = :season_id AND \"enabled\" != 0;",
                GroupParticipation::COLUMNS_SQL
            ))
            .unwrap()
            .query_map(
                named_params! {
                    ":season_id": season_id,
                },
                |row| Ok(GroupParticipation::from_row(row)),
            )
            .expect("Error occurred getting group participations in season from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn create(&self, group_participation: &GroupParticipation) -> Result<Uuid, GenericError> {
        let id = Uuid::new_v4();

        let db = self.db.get();
        let success = db
            .prepare_cached(&format!(
                "INSERT INTO \"group_participation\" ({}) VALUES (:id, :group_id, :season_id, :description, :enabled);",
                GroupParticipation::COLUMNS_SQL
            ))
            .unwrap()
            .execute(named_params! {
                ":id": id,
                ":group_id": group_participation.group_id,
                ":season_id": group_participation.season_id,
                ":description": group_participation.description,
                ":enabled": group_participation.enabled,
            })
            .is_ok();

        if success {
            Ok(id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn update(&self, group_participation: &GroupParticipation) -> Result<Uuid, GenericError> {
        let Some(_existing) = self.get(group_participation.id.unwrap()) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();
        let success = db.prepare_cached("UPDATE \"group_participation\" SET \"group_id\" = :group_id, \"season_id\" = :season_id, \"description\" = :description, \"enabled\" = :enabled WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": group_participation.id,
                ":group_id": group_participation.group_id,
                ":season_id": group_participation.season_id,
                ":description": group_participation.description,
                ":enabled": group_participation.enabled,
            })
            .is_ok();

        if success {
            Ok(group_participation.id.unwrap())
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
            .prepare_cached("DELETE FROM \"group_participation\" WHERE \"id\" = :id;")
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
