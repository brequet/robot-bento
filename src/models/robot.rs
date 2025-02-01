use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct TestRunDB {
    pub id: Option<i32>,
    pub rpa: bool,
    pub generator: String,
    pub generated_date: NaiveDateTime,
    pub schema_version: String,
    // TODO sha1: String,
    pub suites: Vec<SuiteDB>,
    pub statistics: Vec<StatDB>,
    pub errors: Vec<ErrorDB>,
}

#[derive(Debug, Serialize)]
pub struct SuiteDB {
    pub id: Option<i32>,
    pub name: String,
    pub source: Option<String>,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub doc: Option<String>,
    pub identifier: String,
    // TODO: children
}

#[derive(Debug, Serialize)]
pub struct StatDB {
    pub id: Option<i32>,
    pub stat_type: StatTypeDB,
    pub pass_count: i32,
    pub fail_count: i32,
    pub skip_count: i32,
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub text: String,
}

#[derive(Debug, Serialize, Clone, sqlx::Type)]
#[sqlx(type_name = "stat_type", rename_all = "snake_case")]
pub enum StatTypeDB {
    Total,
    Tag,
    Suite,
}

#[derive(Debug, Serialize)]
pub struct ErrorDB {
    pub id: Option<i32>,
    pub timestamp: NaiveDateTime,
    pub level: String,
    pub content: String,
}
