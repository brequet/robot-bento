use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectOverviewResponse {
    pub id: i32,
    pub name: String,
    pub create_date: String,
    pub test_run_count: i32,
    pub last_test_run_summary: Option<ApiTestRunSummary>,
}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTestRunSummary {
    pub test_run_id: i32,
    pub test_run_date: String,
    pub elapsed_time: String,
    pub total_tests: i32,
    pub passed_tests: i32,
    pub failed_tests: i32,
    pub skipped_tests: i32,
    pub error_count: i32,
    pub app_version: String,
}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    pub id: i32,
    pub name: String,
    pub create_date: String,
    pub test_run_count: i32,
    pub test_runs_summaries: Vec<ApiTestRunSummary>,
}
