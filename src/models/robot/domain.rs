use chrono::{Duration, NaiveDateTime};

use crate::{models::projects::api::ApiTestRunSummary, utils};

pub struct ProjectTestRunSummary {
    pub project_id: i32,
    pub test_run_count: i32,
    pub last_test_run_id: i32,
    pub app_version: String,
    pub test_run_date: NaiveDateTime,
    pub elapsed_time: Duration,
    pub total_test_count: i32,
    pub passed_tests: i32,
    pub failed_tests: i32,
    pub skipped_tests: i32,
    pub error_count: i32,
}

impl ProjectTestRunSummary {
    pub fn to_api(&self) -> ApiTestRunSummary {
        ApiTestRunSummary {
            test_run_id: self.last_test_run_id,
            test_run_date: utils::date::format_datetime(self.test_run_date),
            elapsed_time: utils::date::duration_to_string(self.elapsed_time),
            total_tests: self.total_test_count,
            passed_tests: self.passed_tests,
            failed_tests: self.failed_tests,
            skipped_tests: self.skipped_tests,
            error_count: self.error_count,
            app_version: self.app_version.clone(),
        }
    }
}
