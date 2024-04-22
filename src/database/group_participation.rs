use rusqlite::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct GroupParticipation {
    pub id: Option<Uuid>,
    pub group_id: Uuid,
    pub season_id: Uuid,
    pub description: String,
    pub enabled: bool,
}
impl GroupParticipation {
    pub const TABLE_NAME: &'static str = "group_participation";

    pub const COLUMNS_SQL: &'static str =
        "\"id\", \"group_id\", \"season_id\", \"description\", \"enabled\"";

    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row
                .get("id")
                .expect("Failed to get value from database row"),
            group_id: row
                .get("group_id")
                .expect("Failed to get value from database row"),
            season_id: row
                .get("season_id")
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
