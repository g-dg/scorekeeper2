use rusqlite::{named_params, OptionalExtension};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::{
        users::{DbUser, UserPermission},
        Database,
    },
    helpers::errors::GenericError,
};

use super::auth::AuthService;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: String,
    pub new_password: Option<String>,
    pub enabled: bool,
    pub permissions: i64,
    pub permission_user_admin: bool,
    pub permission_setup_admin: bool,
    pub permission_modify_self: bool,
    pub permission_view_results: bool,
    pub permission_view_scores: bool,
    pub permission_enter_scores: bool,
    pub permission_view_registration: bool,
    pub permission_enter_registration: bool,
}
impl User {
    pub fn from_db_user(user: &DbUser) -> Self {
        Self {
            id: Some(user.id),
            username: user.username.clone(),
            new_password: None,
            enabled: user.enabled,
            permissions: user.permissions,
            permission_user_admin: user.permissions & UserPermission::USER_ADMIN != 0,
            permission_setup_admin: user.permissions & UserPermission::SETUP_ADMIN != 0,
            permission_modify_self: user.permissions & UserPermission::MODIFY_SELF != 0,
            permission_view_results: user.permissions & UserPermission::RESULTS_VIEW != 0,
            permission_view_scores: user.permissions & UserPermission::SCORE_VIEW != 0,
            permission_enter_scores: user.permissions & UserPermission::SCORE_ENTRY != 0,
            permission_view_registration: user.permissions & UserPermission::REGISTRATION_VIEW != 0,
            permission_enter_registration: user.permissions & UserPermission::REGISTRATION_ENTRY
                != 0,
        }
    }
}

pub struct UsersService {
    db: Database,
}

impl UsersService {
    pub fn new(database: Database) -> Self {
        Self { db: database }
    }

    pub fn get_user_by_id(&self, id: Uuid) -> Option<DbUser> {
        let db = self.db.get();

        let user_result: Option<DbUser> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"users\" WHERE \"id\" = :id;",
                DbUser::COLUMNS_SQL
            ))
            .expect("Error occurred preparing user select database query")
            .query_row(named_params! {":id": id}, |row| Ok(DbUser::from_row(row)))
            .optional()
            .expect("Error occurred getting user by id from database");

        user_result
    }

    pub fn get_user_by_name(&self, username: &str) -> Option<DbUser> {
        let db = self.db.get();

        let user_result: Option<DbUser> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"users\" WHERE \"username\" = :username;",
                DbUser::COLUMNS_SQL
            ))
            .expect("Error occurred preparing user select database query")
            .query_row(named_params! {":username": username}, |row| {
                Ok(DbUser::from_row(row))
            })
            .optional()
            .expect("Error occurred getting user by name from database");

        user_result
    }

    pub fn get(&self, id: Uuid) -> Option<User> {
        self.get_user_by_id(id)
            .map(|user| User::from_db_user(&user))
    }

    pub fn list(&self) -> Vec<User> {
        let db = self.db.get();

        let users = db
            .prepare_cached(&format!("SELECT {} FROM \"users\";", DbUser::COLUMNS_SQL))
            .expect("Error occurred preparing user select database query")
            .query_map(named_params! {}, |row| Ok(DbUser::from_row(row)))
            .expect("Error occurred getting all users from database")
            .map(|db_user| User::from_db_user(&db_user.unwrap()))
            .collect();

        users
    }

    pub fn create(&self, user: &User) -> Result<Uuid, GenericError> {
        let user_id = Uuid::new_v4();

        let password_hash =
            AuthService::hash_password(&user.new_password.clone().unwrap_or_default());

        let permissions = Self::build_permissions(user);

        let db = self.db.get();
        let success = db.prepare_cached("INSERT INTO \"users\" (\"id\", \"username\", \"password\", \"enabled\", \"permissions\") VALUES (:id, :username, :password, :enabled, :permissions);")
            .expect("Error preparing user insert statement")
            .execute(named_params! {
                ":id": user_id,
                ":username": user.username,
                ":password": password_hash,
                ":enabled": user.enabled,
                ":permissions": permissions
            })
            .is_ok();

        if success {
            Ok(user_id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn update(&self, user: &User) -> Result<Uuid, GenericError> {
        let Some(user_id) = user.id else {
            return Err(GenericError::BAD_REQUEST);
        };
        let Some(old_user) = self.get_user_by_id(user_id) else {
            return Err(GenericError::NOT_FOUND);
        };

        let password_hash = if let Some(ref new_password) = user.new_password {
            AuthService::hash_password(new_password)
        } else {
            old_user.password
        };

        let permissions = Self::build_permissions(user);

        let db = self.db.get();
        let success = db.prepare_cached("UPDATE \"users\" SET \"username\" = :username, \"password\" = :password, \"enabled\" = :enabled, \"permissions\" = :permissions WHERE \"id\" = :id;")
            .expect("Error preparing user update statement")
            .execute(named_params! {
                ":id": user.id,
                ":username": user.username,
                ":password": password_hash,
                ":enabled": user.enabled,
                ":permissions": permissions
            })
            .is_ok();

        if success {
            Ok(user_id)
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn delete(&self, user_id: Uuid) -> Result<(), GenericError> {
        let Some(user) = self.get_user_by_id(user_id) else {
            return Err(GenericError::NOT_FOUND);
        };

        let db = self.db.get();

        let success = db
            .prepare_cached("DELETE FROM \"users\" WHERE \"id\" = :id;")
            .expect("Error preparing user delete statement")
            .execute(named_params! {
                ":id": user.id,
            })
            .is_ok();

        if success {
            Ok(())
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn change_password(
        &self,
        user_id: Uuid,
        new_password: &str,
        session_to_keep: Option<&str>,
    ) -> Result<(), GenericError> {
        let password_hash = AuthService::hash_password(new_password);

        let db = self.db.get();
        let success = db
            .prepare_cached("UPDATE \"users\" SET \"password\" = :password WHERE \"id\" = :id;")
            .expect("Error preparing user update statement")
            .execute(named_params! {
                ":id": user_id,
                ":password": password_hash,
            })
            .is_ok();

        let auth_service = AuthService::new(self.db.clone());
        auth_service.invalidate_sessions(user_id, session_to_keep);

        if success {
            Ok(())
        } else {
            Err(GenericError::BAD_REQUEST)
        }
    }

    pub fn build_permissions(user: &User) -> i64 {
        (if user.permission_modify_self {
            UserPermission::MODIFY_SELF
        } else {
            0
        }) | (if user.permission_user_admin {
            UserPermission::USER_ADMIN
        } else {
            0
        }) | (if user.permission_setup_admin {
            UserPermission::SETUP_ADMIN
        } else {
            0
        }) | (if user.permission_view_results {
            UserPermission::RESULTS_VIEW
        } else {
            0
        }) | (if user.permission_view_scores {
            UserPermission::SCORE_VIEW
        } else {
            0
        }) | (if user.permission_enter_scores {
            UserPermission::SCORE_ENTRY
        } else {
            0
        }) | (if user.permission_view_registration {
            UserPermission::REGISTRATION_VIEW
        } else {
            0
        }) | (if user.permission_enter_registration {
            UserPermission::REGISTRATION_ENTRY
        } else {
            0
        })
    }
}
