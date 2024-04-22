use rusqlite::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Competition {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub enabled: bool,
}
impl Competition {
    pub const COMPETITIONS: &'static str = "competitions";

    pub const COLUMNS_SQL: &'static str = "\"id\", \"name\", \"description\", \"enabled\"";

    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row
                .get("id")
                .expect("Failed to get value from database row"),
            name: row
                .get("name")
                .expect("Failed to get value from database row"),
            description: row
                .get("description")
                .expect("Failed to get value from database row"),
            enabled: row
                .get("enabled")
                .expect("Failed to get value from database row"),
        }
    }
}
