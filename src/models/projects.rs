use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProjectLight {
    pub id: i32,
    pub application_name: String,
    pub application_version: String,
    pub test_run_count: i32,
    pub last_test_run: TestRunLight,
}

#[derive(Debug, Serialize)]
pub struct TestRunLight {
    pub last_test_run_date: NaiveDateTime,
    pub total_tests: i32,
    pub passed_tests: i32,
    pub failed_tests: i32,
    pub skipped_tests: i32,
    pub elapsed_time: i32,
}
