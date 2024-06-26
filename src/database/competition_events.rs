use rusqlite::Row;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use super::scores::ScoreType;

#[derive(Clone, Serialize, Deserialize)]
pub struct CompetitionEvent {
    pub id: Option<Uuid>,
    pub season_competition_id: Uuid,
    pub event_id: Uuid,
    pub description: String,
    pub score_calculator: Option<Uuid>,
    pub calculator_config: JsonValue,
    pub enabled: bool,
    pub score_type: ScoreType,
}
impl CompetitionEvent {
    pub const TABLE_NAME: &'static str = "competition_events";

    pub const COLUMNS_SQL: &'static str =
        "\"id\", \"season_competition_id\", \"event_id\", \"description\", \"score_calculator\", \"calculator_config\", \"enabled\", \"score_type\"";

    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row
                .get("id")
                .expect("Failed to get value from database row"),
            season_competition_id: row
                .get("season_competition_id")
                .expect("Failed to get value from database row"),
            event_id: row
                .get("event_id")
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
            score_type: row
                .get("score_type")
                .expect("Failed to get value from database row"),
        }
    }
}
