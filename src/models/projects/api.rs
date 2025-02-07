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
    pub last_test_run_summary: Option<TestRunSummary>,
}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestRunSummary {
    pub last_test_run_date: String,
    pub total_tests: i32,
    pub passed_tests: i32,
    pub failed_tests: i32,
    pub skipped_tests: i32,
    // pub elapsed_time: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    pub id: i32,
    pub name: String,
    pub test_run_count: i32,
    // pub test_runs: Vec<TestRunDB>,
}
