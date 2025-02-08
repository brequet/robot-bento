use chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct TestRunOverviewDB {
    pub project_id: i32,
    pub application_version: String,
    pub test_run_count: Option<i32>,
    pub last_test_run_date: Option<NaiveDateTime>,
    pub last_passed_tests: Option<i32>,
    pub last_failed_tests: Option<i32>,
    pub last_skipped_tests: Option<i32>,
}
