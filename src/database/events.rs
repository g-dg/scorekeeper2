use rusqlite::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Option<Uuid>,
    pub competition_id: Uuid,
    pub name: String,
    pub description: String,
    pub enabled: bool,
}
impl Event {
    pub const TABLE_NAME: &'static str = "events";

    pub const COLUMNS_SQL: &'static str =
        "\"id\", \"competition_id\", \"name\", \"description\", \"enabled\"";

    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row
                .get("id")
                .expect("Failed to get value from database row"),
            competition_id: row
                .get("competition_id")
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
