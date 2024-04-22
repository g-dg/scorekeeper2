use chrono::{DateTime, Utc};
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    Row, ToSql,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub enum ScoreType {
    Group,
    Team,
}
impl ToSql for ScoreType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(match self {
            Self::Group => "group",
            Self::Team => "team",
        }
        .into())
    }
}
impl FromSql for ScoreType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str() {
            Ok("group") => Ok(Self::Group),
            Ok("team") => Ok(Self::Team),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Score {
    pub id: Option<Uuid>,
    pub competition_event_id: Uuid,
    pub score_type: ScoreType,
    pub subject_id: Uuid,
    pub score_data: String,
    pub timestamp: DateTime<Utc>,
    pub valid: bool,
    pub disqualified: bool,
    pub notes: String,
}
impl Score {
    pub const UNION_SELECT: &'static str =
        "SELECT \"id\", \"competition_event_id\", 'group' AS \"score_type\", \"group_participation_id\" AS \"subject_id\", \"score_data\", \"timestamp\", \"valid\", \"disqualified\", \"notes\" FROM \"group_scores\"
UNION ALL SELECT \"id\", \"competition_event_id\", 'team' AS \"score_type\", \"team_id\" AS \"subject_id\", \"score_data\", \"timestamp\", \"valid\", \"disqualified\", \"notes\" FROM \"team_scores\"";

    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row
                .get("id")
                .expect("Failed to get value from database row"),
            competition_event_id: row
                .get("competition_event_id")
                .expect("Failed to get value from database row"),
            score_type: row
                .get("score_type")
                .expect("Failed to get value from database row"),
            subject_id: row
                .get("subject_id")
                .expect("Failed to get value from database row"),
            score_data: row
                .get("score_data")
                .expect("Failed to get value from database row"),
            timestamp: row
                .get("timestamp")
                .expect("Failed to get value from database row"),
            valid: row
                .get("valid")
                .expect("Failed to get value from database row"),
            disqualified: row
                .get("disqualified")
                .expect("Failed to get value from database row"),
            notes: row
                .get("notes")
                .expect("Failed to get value from database row"),
        }
    }
}
