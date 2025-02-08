use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestRunResponse {
    pub id: Option<i32>,
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
pub struct ApiSuite {}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiStatistic {}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {}
