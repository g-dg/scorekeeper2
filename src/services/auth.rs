use std::time::Duration;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{TimeDelta, Utc};
use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::OsRng,
};
use rusqlite::{named_params, OptionalExtension};
use serde_json::json;
use uuid::Uuid;

use crate::database::{
    users::{DbSession, DbUser},
    Database,
};

use super::{audit::AuditService, users::UsersService};

pub const SESSION_MAX_AGE: Duration = Duration::from_secs(60 * 60 * 24);

pub struct AuthService {
    db: Database,
    audit_service: AuditService,
    user_service: UsersService,
}

impl AuthService {
    pub fn new(database: Database) -> Self {
        Self {
            audit_service: AuditService::new(database.clone()),
            user_service: UsersService::new(database.clone()),
            db: database,
        }
    }

    /// Authenticates a user, returning a session id or None if authentication failed
    pub fn authenticate(&self, username: &str, password: &str) -> Option<String> {
        let timestamp = Utc::now();

        let db = self.db.get();

        let user = self.user_service.get_user_by_name(username);

        // check that the user exists
        let Some(user) = user else {
            self.audit_service.log_data(
                None,
                "login_failed",
                json!({"reason": "not_found", "username": username}),
            );
            return None;
        };

        // check if user is enabled
        if !user.enabled {
            self.audit_service.log_data(
                Some(user.id),
                "login_failed",
                json!({"reason": "user_disabled"}),
            );
            return None;
        }

        // check password
        let hash = PasswordHash::new(&user.password).expect("Failed to parse password hash");
        let password_verified = Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok();
        if !password_verified {
            self.audit_service.log_data(
                Some(user.id),
                "login_failed",
                json!({"reason": "password_incorrect"}),
            );
            return None;
        }

        // create session id
        let token = Alphanumeric.sample_string(&mut OsRng, 255);

        // insert session into database
        db
            .prepare_cached("INSERT INTO \"sessions\" (\"token\", \"user_id\", \"timestamp\", \"valid\") VALUES (:token, :user_id, :timestamp, TRUE);")
            .expect("Error occurred preparing session insert database query")
            .execute(named_params! {":token": &token, ":user_id": user.id, ":timestamp": timestamp})
            .expect("Error occurred inserting session into database");

        // audit login
        self.audit_service
            .log_data(Some(user.id), "login_success", json!({"session_id": token}));

        Some(token)
    }

    /// Checks if the user for the provided token is authorized based on required permissions
    pub fn authorize(&self, token: &str, required_permissions: i64) -> Option<DbUser> {
        let now = Utc::now();

        let db = self.db.get();

        // get session
        let session = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"sessions\" WHERE \"token\" = :token;",
                DbSession::COLUMNS_SQL
            ))
            .expect("Error occurred preparing session select database query")
            .query_row(named_params! {":token": token}, |row| {
                Ok(DbSession::from_row(row))
            })
            .optional()
            .expect("Error occurred getting session from database");

        // check that the session exists
        let Some(session) = session else {
            self.audit_service.log_data(
                None,
                "authorize_failed",
                json!({"reason": "not_found", "session_id": token}),
            );
            return None;
        };

        // check if session has valid flag
        if !session.valid {
            self.audit_service.log_data(
                None,
                "authorize_failed",
                json!({"reason": "session_invalid", "session_id": token}),
            );
            return None;
        }

        let session_max_age = TimeDelta::from_std(SESSION_MAX_AGE).unwrap();

        // check if session timestamp expired
        if now - session.timestamp > session_max_age {
            self.audit_service.log_data(
                None,
                "authorize_failed",
                json!({"reason": "session_expired", "session_id": token}),
            );
            return None;
        }

        // update session timestamp if it's older than 60 seconds to prevent unnecessary database writes
        if now - session.timestamp > std::cmp::min(TimeDelta::seconds(60), session_max_age / 2) {
            db.prepare_cached(
                "UPDATE \"sessions\" SET \"timestamp\" = :timestamp WHERE \"token\" = :token;",
            )
            .expect("Error occurred preparing session update database query")
            .execute(named_params! {":timestamp": now, ":token": token})
            .expect("Error occurred updating session timestamp");
        }

        // get session user from database
        let user = self.user_service.get_user_by_id(session.user_id);

        // check that user exists
        let Some(user) = user else {
            self.audit_service.log_data(
                Some(session.user_id),
                "authorize_failed",
                json!({"reason": "session_user_not_found"}),
            );
            return None;
        };

        // check that the user is enabled
        if !user.enabled {
            self.audit_service.log_data(
                Some(user.id),
                "authorize_failed",
                json!({"reason": "user_disabled"}),
            );
            return None;
        }

        // check that the user has an allowed role
        if Self::check_permissions(user.permissions, required_permissions) {
            self.audit_service.log_data(
                Some(user.id),
                "authorize_failed",
                json!({"reason": "invalid_role", "required": required_permissions, "actual": user.permissions}),
            );
            return None;
        }

        Some(user)
    }

    /// Invalidates a session token
    pub fn logout(&self, token: &str) {
        let db = self.db.get();

        // get user id from token
        let user_id = self.get_user_id_from_token(token);

        // set valid flag to false for session
        db.prepare_cached("UPDATE \"sessions\" SET \"valid\" = FALSE WHERE \"token\" = :token;")
            .expect("Error occurred preparing session update database query")
            .execute(named_params! {":token": token})
            .expect("Error occurred logging out session");

        // audit logout
        self.audit_service
            .log_data(user_id, "logout", json!({"session_id": token}));
    }

    pub fn invalidate_sessions(&self, user_id: Uuid, except: Option<&str>) {
        let db = self.db.get();

        if let Some(token) = except {
            db.prepare_cached(
                "UPDATE \"sessions\" SET \"valid\" = FALSE WHERE \"user_id\" = :user_id AND \"token\" != :token;",
            )
            .expect("Error occurred preparing session update database query")
            .execute(named_params! {":user_id": user_id, ":token": token})
            .expect("Error occurred invalidating sessions for user");

            // audit session invalidation
            self.audit_service.log_data(
                Some(user_id),
                "session_invalidate_all_except",
                json!({"except": token}),
            );
        } else {
            db.prepare_cached(
                "UPDATE \"sessions\" SET \"valid\" = FALSE WHERE \"user_id\" = :user_id;",
            )
            .expect("Error occurred preparing session update database query")
            .execute(named_params! {":user_id": user_id})
            .expect("Error occurred invalidating all sessions for user");

            // audit session invalidation
            self.audit_service
                .log(Some(user_id), "session_invalidate_all");
        }
    }

    pub fn get_user_id_from_token(&self, token: &str) -> Option<Uuid> {
        let db = self.db.get();

        let session = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"sessions\" WHERE \"token\" = :token;",
                DbSession::COLUMNS_SQL
            ))
            .expect("Error occurred preparing session select database query")
            .query_row(named_params! {":token": token}, |row| {
                Ok(DbSession::from_row(row))
            })
            .optional()
            .expect("Error occurred getting session from database");

        session.map(|session| session.user_id)
    }

    pub fn check_permissions(user_permissions: i64, required_permissions: i64) -> bool {
        (required_permissions & user_permissions) != required_permissions
    }

    pub fn hash_password(password: &str) -> String {
        Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .expect("Error occurred hashing password")
            .to_string()
    }
}
