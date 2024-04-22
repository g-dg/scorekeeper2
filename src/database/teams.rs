use rusqlite::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: Option<Uuid>,
    pub group_participation_id: Uuid,
    pub name: String,
    pub description: String,
    pub enabled: bool,
}
impl Team {
    pub const TABLE_NAME: &'static str = "teams";

    pub const COLUMNS_SQL: &'static str =
        "\"id\", \"group_participation_id\", \"name\", \"description\", \"enabled\"";

    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row
                .get("id")
                .expect("Failed to get value from database row"),
            group_participation_id: row
                .get("group_participation_id")
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
