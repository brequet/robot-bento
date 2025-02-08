use chrono::NaiveDateTime;

use crate::{models::projects::api::ApiTestRunSummary, utils};

pub struct ProjectLatestTestRunSummary {
    pub project_id: i32,
    pub test_run_count: i32,
    pub last_test_run_id: i32,
    pub last_test_run_date: NaiveDateTime,
    pub last_total_test_count: i32,
    pub last_passed_tests: i32,
    pub last_failed_tests: i32,
    pub last_skipped_tests: i32,
    pub last_error_count: i32,
}

impl ProjectLatestTestRunSummary {
    pub fn to_api(&self) -> ApiTestRunSummary {
        ApiTestRunSummary {
            test_run_id: self.last_test_run_id,
            test_run_date: utils::date::format_datetime(self.last_test_run_date),
            total_tests: self.last_total_test_count,
            passed_tests: self.last_passed_tests,
            failed_tests: self.last_failed_tests,
            skipped_tests: self.last_skipped_tests,
            error_count: self.last_error_count,
        }
    }
}
