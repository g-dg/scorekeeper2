use std::time::Duration;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{TimeDelta, Utc};
use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::OsRng,
};
use rusqlite::{named_params, OptionalExtension};
use uuid::Uuid;

use crate::{
    database::{
        auth::{DbSession, DbUser},
        Database,
    },
    AppState,
};

pub const SESSION_MAX_AGE: Duration = Duration::from_secs(60 * 60 * 24);

pub struct Auth {
    db: Database,
}

impl Auth {
    pub fn new(database: Database) -> Self {
        Self { db: database }
    }

    /// Authenticates a user, returning a session id or None if authentication failed
    pub fn authenticate(&self, username: &str, password: &str) -> Option<String> {
        let timestamp = Utc::now();

        let db = self.db.get();

        let user = self.get_user_by_name(username);

        // check that the user exists
        let Some(user) = user else {
            return None;
        };

        // check if user is enabled
        if !user.enabled {
            return None;
        }

        // check password
        let hash = PasswordHash::new(&user.password).expect("Failed to parse password hash");
        let password_verified = Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok();
        if !password_verified {
            return None;
        }

        // create session id
        let token = Alphanumeric.sample_string(&mut OsRng, 255);

        // insert session into database
        db
            .prepare_cached("INSERT INTO \"sessions\" (\"token\", \"user_id\", \"timestamp\", \"valid\") VALUES (:token, :user_id, :timestamp, TRUE);")
            .expect("Error occurred while preparing session insert database query")
            .execute(named_params! {":token": &token, ":user_id": user.id, ":timestamp": timestamp})
            .expect("Error occurred while inserting session into database");

        Some(token)
    }

    /// Checks if the user for the provided token is authorized based on required permissions
    pub fn authorize(&self, token: &str, required_permissions: i64) -> Option<DbUser> {
        let now = Utc::now();

        let db = self.db.get();

        // get user id from session
        let session = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"sessions\" WHERE \"token\" = :token;",
                DbSession::COLUMNS_SQL
            ))
            .expect("Error occurred while preparing session select database query")
            .query_row(named_params! {":token": token}, |row| {
                Ok(DbSession::from_row(row))
            })
            .optional()
            .expect("Error occurred while selecting session from database");

        // check that the session exists
        let Some(session) = session else {
            return None;
        };

        // check if session has valid flag
        if !session.valid {
            return None;
        }

        let session_max_age = TimeDelta::from_std(SESSION_MAX_AGE).unwrap();

        // check if session timestamp expired
        if now - session.timestamp > session_max_age {
            return None;
        }

        // update session timestamp if it's older than 60 seconds to prevent unnecessary database writes
        if now - session.timestamp > std::cmp::min(TimeDelta::seconds(60), session_max_age / 2) {
            db.prepare_cached(
                "UPDATE \"sessions\" SET \"timestamp\" = :timestamp WHERE \"token\" = :token;",
            )
            .expect("Error occurred while preparing session update database query")
            .execute(named_params! {":timestamp": now, ":token": token})
            .expect("Error occurred while updating session in database");
        }

        // get user from database
        let user = self.get_user_by_id(session.user_id);

        // check that user exists
        let Some(user) = user else {
            return None;
        };

        // check that the user is enabled
        if !user.enabled {
            return None;
        }

        // check that the user has an allowed role
        if Self::check_permissions(user.permissions, required_permissions) {
            return None;
        }

        Some(user)
    }

    /// Invalidates a session token
    pub fn logout(&self, token: &str) {
        let db = self.db.get();

        db.prepare_cached("UPDATE \"sessions\" SET \"valid\" = FALSE WHERE \"token\" = :token;")
            .expect("Error occurred while preparing session update database query")
            .execute(named_params! {":token": token})
            .expect("Error occurred while updating session in database");
    }

    /// Gets a user by username
    pub fn get_user_by_name(&self, username: &str) -> Option<DbUser> {
        let db = self.db.get();

        // get user from database
        let user_result: Option<DbUser> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"users\" WHERE \"username\" = :username;",
                DbUser::COLUMNS_SQL
            ))
            .expect("Error occurred while preparing user select database query")
            .query_row(named_params! {":username": username}, |row| {
                Ok(DbUser::from_row(row))
            })
            .optional()
            .expect("Error occurred while selecting user from database database");

        user_result
    }

    /// Gets a user by user id
    pub fn get_user_by_id(&self, id: Uuid) -> Option<DbUser> {
        let db = self.db.get();

        // get user from database
        let user_result: Option<DbUser> = db
            .prepare_cached(&format!(
                "SELECT {} FROM \"users\" WHERE \"id\" = :id;",
                DbUser::COLUMNS_SQL
            ))
            .expect("Error occurred while preparing user select database query")
            .query_row(named_params! {":id": id}, |row| Ok(DbUser::from_row(row)))
            .optional()
            .expect("Error occurred while selecting user from database database");

        user_result
    }

    pub fn check_permissions(user_permissions: i64, required_permissions: i64) -> bool {
        (required_permissions & user_permissions) != required_permissions
    }

    pub fn hash_password(password: &str) -> String {
        Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .expect("Error occurred while hashing password")
            .to_string()
    }
}

/// Auth token extractor
pub struct AuthToken {
    /// API token
    pub token: String,
}

impl AuthToken {
    /// Gets the API token
    pub fn token(&self) -> &str {
        self.token.as_str()
    }

    /// Helper method to authorize and get the user for the extracted token.
    /// Returns None if authorization failed.
    pub fn authorize(&self, state: &AppState, required_permissions: i64) -> Option<DbUser> {
        state.auth.authorize(&self.token, required_permissions)
    }

    /// Helper method to invalidate the extracted API token
    pub fn logout(&self, state: &AppState) {
        state.auth.logout(&self.token)
    }

    pub fn failure_response() -> Response {
        let mut headers = HeaderMap::new();
        headers.insert("WWW-Authenticate", "Bearer".parse().unwrap());
        (StatusCode::UNAUTHORIZED, headers).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthToken
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, HeaderMap);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_result = parts.extract::<TypedHeader<Authorization<Bearer>>>().await;

        if let Ok(TypedHeader(Authorization(bearer))) = auth_result {
            Ok(Self {
                token: String::from(bearer.token()),
            })
        } else {
            let mut headers = HeaderMap::new();
            headers.insert("WWW-Authenticate", "Bearer".parse().unwrap());
            Err((StatusCode::UNAUTHORIZED, headers))
        }
    }
}
