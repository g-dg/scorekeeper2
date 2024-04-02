use chrono::{DateTime, Utc};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct UserPermission {}
impl UserPermission {
    pub const NONE: i64 = 0;
    pub const ADMINISTRATION: i64 = 1;
    pub const RESULTS_VIEW: i64 = 2;
    pub const SCORE_VIEW: i64 = 4;
    pub const SCORE_ENTRY: i64 = 8;
    pub const REGISTRATION_VIEW: i64 = 16;
    pub const REGISTRATION_ENTRY: i64 = 32;
}

#[derive(Serialize, Deserialize)]
pub struct DbUser {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub enabled: bool,
    pub permissions: i64,
}
impl DbUser {
    pub const COLUMNS_SQL: &'static str =
        "\"id\", \"username\", \"password\", \"permissions\", \"enabled\"";

    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row
                .get("id")
                .expect("Failed to get value from database row"),
            username: row
                .get("username")
                .expect("Failed to get value from database row"),
            password: row
                .get("password")
                .expect("Failed to get value from database row"),
            permissions: row
                .get("permissions")
                .expect("Failed to get value from database row"),
            enabled: row
                .get("enabled")
                .expect("Failed to get value from database row"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DbSession {
    pub id: Uuid,
    pub token: String,
    pub user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub valid: bool,
}
impl DbSession {
    pub const COLUMNS_SQL: &'static str =
        "\"id\", \"token\", \"user_id\", \"timestamp\", \"valid\"";

    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row
                .get("id")
                .expect("Failed to get value from database row"),
            token: row
                .get("token")
                .expect("Failed to get value from database row"),
            user_id: row
                .get("user_id")
                .expect("Failed to get value from database row"),
            timestamp: row
                .get("timestamp")
                .expect("Failed to get value from database row"),
            valid: row
                .get("valid")
                .expect("Failed to get value from database row"),
        }
    }
}
