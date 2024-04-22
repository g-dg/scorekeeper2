use rusqlite::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct ScoreCalculator {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub script: String,
    pub config_options: String,
}
impl ScoreCalculator {
    pub const COLUMNS_SQL: &'static str =
        "\"id\", \"name\", \"description\", \"script\", \"config_options\"";

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
            script: row
                .get("script")
                .expect("Failed to get value from database row"),
            config_options: row
                .get("config_options")
                .expect("Failed to get value from database row"),
        }
    }
}
