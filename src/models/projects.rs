use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProjectOverview {
    pub id: i32,
    pub name: String,
    pub test_run_count: i32,
    pub last_test_run: ProjectTestRun,
}

#[derive(Debug, Serialize)]
pub struct ProjectTestRun {
    pub last_test_run_date: NaiveDateTime,
    pub total_tests: i32,
    pub passed_tests: i32,
    pub failed_tests: i32,
    pub skipped_tests: i32,
    pub elapsed_time: i32,
}
