use chrono::NaiveDateTime;
use sqlx::postgres::types::PgInterval;

use crate::utils;

use super::domain::ProjectTestRunSummary;

#[derive(sqlx::FromRow)]
pub struct ProjectTestSummaryDB {
    pub project_id: i32,
    pub test_run_count: Option<i32>,
    pub test_run_id: i32,
    pub application_version: String,
    pub test_run_date: NaiveDateTime,
    pub elapsed_time: Option<PgInterval>,
    pub passed_tests: i32,
    pub failed_tests: i32,
    pub skipped_tests: i32,
    pub error_count: Option<i32>,
}

impl ProjectTestSummaryDB {
    pub fn into_summary(self) -> ProjectTestRunSummary {
        ProjectTestRunSummary {
            project_id: self.project_id,
            test_run_count: self.test_run_count.unwrap_or(0),
            last_test_run_id: self.test_run_id,
            app_version: self.application_version,
            test_run_date: self.test_run_date,
            elapsed_time: self
                .elapsed_time
                .map(utils::date::pg_interval_to_duration)
                .unwrap_or_default(),
            total_test_count: self.passed_tests + self.failed_tests + self.skipped_tests,
            passed_tests: self.passed_tests,
            failed_tests: self.failed_tests,
            skipped_tests: self.skipped_tests,
            error_count: self.error_count.unwrap_or(0),
        }
    }
}
