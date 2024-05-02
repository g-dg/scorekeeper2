use rusqlite::{named_params, OptionalExtension};
use uuid::Uuid;

use crate::{
    database::{teams::Team, Database},
    helpers::errors::GenericError,
};

pub struct TeamsService {
    db: Database,
}

impl TeamsService {
    pub fn new(database: &Database) -> Self {
        Self {
            db: database.clone(),
        }
    }

    pub fn get(&self, id: Uuid) -> Option<Team> {
        let db = self.db.get();
        let result: Option<Team> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"teams\" WHERE \"id\" = :id;",
                Team::COLUMNS_SQL
            ))
            .unwrap()
            .query_row(named_params! {":id": id}, |row| Ok(Team::from_row(row)))
            .optional()
            .expect("Error occurred getting team by id from database");

        result
    }

    pub fn list(&self) -> Vec<Team> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!("SELECT {} FROM \"teams\";", Team::COLUMNS_SQL))
            .unwrap()
            .query_map(named_params! {}, |row| Ok(Team::from_row(row)))
            .expect("Error occurred getting all teams from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    /// Returns enabled teams in specified group participation
    pub fn list_in_group_participation(&self, group_participation_id: Uuid) -> Vec<Team> {
        let db = self.db.get();
        let result = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"teams\" WHERE \"group_participation_id\" = :group_participation_id AND \"enabled\" != 0;",
                Team::COLUMNS_SQL
            ))
            .unwrap()
            .query_map(named_params! {
                ":group_participation_id": group_participation_id,
            }, |row| Ok(Team::from_row(row)))
            .expect("Error occurred getting teams in group participation from database")
            .map(|x| x.unwrap())
            .collect();

        result
    }

    pub fn create(&self, team: &Team) -> Result<Uuid, GenericError> {
        let id = Uuid::new_v4();

        let db = self.db.get();
        let success = db
            .prepare_cached(&format!(
                "INSERT INTO \"teams\" ({}) VALUES (:id, :group_participation_id, :name, :description, :enabled);",
                Team::COLUMNS_SQL
            ))
            .unwrap()
            .execute(named_params! {
                ":id": id,
                ":group_participation_id": team.group_participation_id,
                ":name": team.name,
                ":description": team.description,
                ":enabled": team.enabled,
            })
            .is_ok();

        if success {
            Ok(id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn update(&self, team: &Team) -> Result<Uuid, GenericError> {
        let Some(_existing) = self.get(team.id.unwrap()) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();
        let success = db.prepare_cached("UPDATE \"teams\" SET \"group_participation_id\" = :group_participation_id, \"name\" = :name, \"description\" = :description, \"enabled\" = :enabled WHERE \"id\" = :id;")
            .unwrap()
            .execute(named_params! {
                ":id": team.id,
                ":group_participation_id": team.group_participation_id,
                ":name": team.name,
                ":description": team.description,
                ":enabled": team.enabled,
            })
            .is_ok();

        if success {
            Ok(team.id.unwrap())
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
            .prepare_cached("DELETE FROM \"teams\" WHERE \"id\" = :id;")
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
