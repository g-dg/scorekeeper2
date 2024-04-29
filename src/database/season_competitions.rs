use rusqlite::Row;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct SeasonCompetition {
    pub id: Option<Uuid>,
    pub season_id: Uuid,
    pub competition_id: Uuid,
    pub description: String,
    pub score_calculator: Option<Uuid>,
    pub calculator_config: JsonValue,
    pub enabled: bool,
}
impl SeasonCompetition {
    pub const TABLE_NAME: &'static str = "season_competitions";

    pub const COLUMNS_SQL: &'static str =
        "\"id\", \"season_id\", \"competition_id\", \"description\", \"score_calculator\", \"calculator_config\", \"enabled\"";

    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row
                .get("id")
                .expect("Failed to get value from database row"),
            season_id: row
                .get("season_id")
                .expect("Failed to get value from database row"),
            competition_id: row
                .get("competition_id")
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
