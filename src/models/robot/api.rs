use serde::Serialize;
use serde_json::Value;
use typeshare::typeshare;

use super::domain::StatisticType;

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestRunResponse {
    pub id: i32,
    pub project_id: i32,
    pub rpa: bool,
    pub generator: String,
    pub generated_date: String,
    pub schema_version: String,
    pub imported_date: String,
    pub suites: Vec<ApiSuite>,
    pub statistics: Vec<ApiStatistic>,
    pub errors: Vec<ApiError>,
    pub app_version: String,
}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSuite {
    pub id: i32,
    pub name: String,
    pub source: Option<String>,
    pub status: String,
    pub start_time: String,
    pub end_time: String,
    pub doc: Option<String>,
    pub identifier: String,
    pub suites: Vec<ApiSuite>,
    pub tests: Vec<ApiTest>,
}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTest {
    pub id: i32,
    pub name: String,
    pub line: i32,
    pub identifier: String,
    pub tags: Vec<String>,
    pub status: String,
    pub start_time: String,
    pub end_time: String,
    pub doc: Option<String>,
    pub timeout: Option<String>,
}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiStatisticType {
    Total,
    Tag,
    Suite,
}

impl From<StatisticType> for ApiStatisticType {
    fn from(domain_type: StatisticType) -> Self {
        match domain_type {
            StatisticType::Total => ApiStatisticType::Total,
            StatisticType::Tag => ApiStatisticType::Tag,
            StatisticType::Suite => ApiStatisticType::Suite,
        }
    }
}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiStatistic {
    pub id: i32,
    pub stat_type: ApiStatisticType,
    pub pass_count: i32,
    pub fail_count: i32,
    pub skip_count: i32,
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub text: String,
}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub id: i32,
    pub timestamp: String,
    pub level: String,
    pub content: String,
}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSuiteKeywords<'a> {
    pub setup_keyword: Option<&'a Value>,
    pub teardown_keyword: Option<&'a Value>,
}
