use rusqlite::Row;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Season {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub score_calculator: Option<Uuid>,
    pub calculator_config: JsonValue,
    pub enabled: bool,
}
impl Season {
    pub const TABLE_NAME: &'static str = "seasons";

    pub const COLUMNS_SQL: &'static str =
        "\"id\", \"name\", \"description\", \"score_calculator\", \"calculator_config\", \"enabled\"";

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
            score_calculator: row
                .get("score_calculator")
                .expect("Failed to get value from database row"),
            calculator_config: row
                .get("calculator_config")
                .expect("Failed to get value from database row"),
            enabled: row
                .get("enabled")
                .expect("Failed to get value from database row"),
        }
    }
}
