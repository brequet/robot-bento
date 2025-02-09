use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

use crate::services::parser::{self};

#[derive(Debug, Serialize, FromRow)]
pub struct TestRunDBLegacy {
    pub id: Option<i32>,
    pub rpa: bool,
    pub generator: String,
    pub generated_date: NaiveDateTime,
    pub schema_version: String,
    pub imported_date: Option<NaiveDateTime>,
    pub suites: Vec<SuiteDBLegacy>,
    pub statistics: Vec<StatDBLegacy>,
    pub errors: Vec<ErrorDBLegacy>,
    pub sha1: String,
    // Metadata
    pub app_name: String,
    pub app_version: String,
}

#[derive(Debug, Serialize)]
pub struct SuiteDBLegacy {
    pub id: Option<i32>,
    pub name: String,
    pub source: Option<String>,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub doc: Option<String>,
    pub identifier: String,
    pub setup_keyword: Option<parser::Keyword>,
    pub suites: Vec<SuiteDBLegacy>,
    pub tests: Vec<TestDBLegacy>,
    pub teardown_keyword: Option<parser::Keyword>,
}

#[derive(Debug, Serialize)]
pub struct TestDBLegacy {
    pub id: Option<i32>,
    pub name: String,
    pub line: i32,
    pub identifier: String,
    pub tags: Vec<String>,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub doc: Option<String>,
    pub timeout: Option<String>,
    pub keywords: Vec<parser::BaseBody>,
}

#[derive(Debug, Serialize)]
pub struct StatDBLegacy {
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
pub struct ErrorDBLegacy {
    pub id: Option<i32>,
    pub timestamp: NaiveDateTime,
    pub level: String,
    pub content: String,
}
