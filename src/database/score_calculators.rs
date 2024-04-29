use rusqlite::Row;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct ScoreCalculator {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub script: String,
    pub default_config: JsonValue,
    pub supports_seasons: bool,
    pub supports_competitions: bool,
    pub supports_events: bool,
    pub score_fields: Option<JsonValue>,
}
impl ScoreCalculator {
    pub const COLUMNS_SQL: &'static str =
        "\"id\", \"name\", \"description\", \"script\", \"default_config\", \"supports_seasons\", \"supports_competitions\", \"supports_events\", \"score_fields\"";

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
            default_config: row
                .get("default_config")
                .expect("Failed to get value from database row"),
            supports_seasons: row
                .get("supports_seasons")
                .expect("Failed to get value from database row"),
            supports_competitions: row
                .get("supports_competitions")
                .expect("Failed to get value from database row"),
            supports_events: row
                .get("supports_events")
                .expect("Failed to get value from database row"),
            score_fields: row
                .get("score_fields")
                .expect("Failed to get value from database row"),
        }
    }
}
