use chrono::NaiveDateTime;

use super::domain::ProjectLatestTestRunSummary;

#[derive(sqlx::FromRow)]
pub struct ProjectTestSummaryDB {
    pub project_id: i32,
    pub test_run_count: Option<i32>,
    pub last_test_run_id: i32,
    pub last_test_run_date: NaiveDateTime,
    pub last_passed_tests: i32,
    pub last_failed_tests: i32,
    pub last_skipped_tests: i32,
    pub last_error_count: Option<i32>,
}

impl ProjectTestSummaryDB {
    pub fn into_summary(self) -> ProjectLatestTestRunSummary {
        ProjectLatestTestRunSummary {
            project_id: self.project_id,
            test_run_count: self.test_run_count.unwrap_or(0),
            last_test_run_id: self.last_test_run_id,
            last_test_run_date: self.last_test_run_date,
            last_total_test_count: self.last_passed_tests
                + self.last_failed_tests
                + self.last_skipped_tests,
            last_passed_tests: self.last_passed_tests,
            last_failed_tests: self.last_failed_tests,
            last_skipped_tests: self.last_skipped_tests,
            last_error_count: self.last_error_count.unwrap_or(0),
        }
    }
}
