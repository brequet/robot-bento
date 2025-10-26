use std::{collections::HashMap, sync::Arc};

use chrono::{Duration, NaiveDateTime};
use serde_json::Value;

use crate::{models::projects::api::ApiTestRunSummary, utils};

use super::{
    api::{ApiError, ApiStatistic, ApiSuite, ApiSuiteKeywords, ApiTest, TestRunResponse},
    db::StatisticTypeDB,
};

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

pub struct SavedTestRun {
    pub id: i32,
    pub project_id: i32,
    pub rpa: bool,
    pub generator: String,
    pub generated_date: NaiveDateTime,
    pub schema_version: String,
    pub imported_date: NaiveDateTime,
    pub suites: Vec<TestRunSuite>,
    pub statistics: Vec<TestRunStatistic>,
    pub errors: Vec<TestRunError>,
    pub app_version: String,
}

impl SavedTestRun {
    pub fn to_response(&self) -> TestRunResponse {
        TestRunResponse {
            id: self.id,
            project_id: self.project_id,
            rpa: self.rpa,
            generator: self.generator.clone(),
            generated_date: utils::date::format_datetime(self.generated_date),
            schema_version: self.schema_version.clone(),
            imported_date: utils::date::format_datetime(self.imported_date),
            suites: self.suites.iter().map(TestRunSuite::to_api).collect(),
            statistics: self
                .statistics
                .iter()
                .map(TestRunStatistic::to_api)
                .collect(),
            errors: self.errors.iter().map(TestRunError::to_api).collect(),
            app_version: self.app_version.clone(),
        }
    }
}

pub struct TestRunSuite {
    pub id: i32,
    pub name: String,
    pub source: Option<String>,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub doc: Option<String>,
    pub identifier: String,
    pub suites: Vec<TestRunSuite>,
    pub tests: Vec<TestRunTest>,
}

impl TestRunSuite {
    fn to_api(&self) -> ApiSuite {
        ApiSuite {
            id: self.id,
            name: self.name.clone(),
            source: self.source.clone(),
            status: self.status.clone(),
            start_time: utils::date::format_datetime(self.start_time),
            end_time: utils::date::format_datetime(self.end_time),
            doc: self.doc.clone(),
            identifier: self.identifier.clone(),
            suites: self.suites.iter().map(|suite| suite.to_api()).collect(),
            tests: self.tests.iter().map(|test| test.to_api()).collect(),
        }
    }
}

pub struct TestRunTest {
    pub id: i32,
    pub name: String,
    pub line: i32,
    pub identifier: String,
    pub tags: Vec<String>,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub doc: Option<String>,
    pub timeout: Option<String>,
}

impl TestRunTest {
    fn to_api(&self) -> ApiTest {
        ApiTest {
            id: self.id,
            name: self.name.clone(),
            line: self.line,
            identifier: self.identifier.clone(),
            tags: self.tags.clone(),
            status: self.status.clone(),
            start_time: utils::date::format_datetime(self.start_time),
            end_time: utils::date::format_datetime(self.end_time),
            doc: self.doc.clone(),
            timeout: self.timeout.clone(),
        }
    }
}

#[derive(Clone)]
pub enum StatisticType {
    Total,
    Tag,
    Suite,
}

impl From<StatisticTypeDB> for StatisticType {
    fn from(db_type: StatisticTypeDB) -> Self {
        match db_type {
            StatisticTypeDB::Total => StatisticType::Total,
            StatisticTypeDB::Tag => StatisticType::Tag,
            StatisticTypeDB::Suite => StatisticType::Suite,
        }
    }
}

pub struct TestRunStatistic {
    pub id: i32,
    pub stat_type: StatisticType,
    pub pass_count: i32,
    pub fail_count: i32,
    pub skip_count: i32,
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub text: String,
}

impl TestRunStatistic {
    fn to_api(&self) -> ApiStatistic {
        ApiStatistic {
            id: self.id,
            stat_type: self.stat_type.clone().into(),
            pass_count: self.pass_count,
            fail_count: self.fail_count,
            skip_count: self.skip_count,
            identifier: self.identifier.clone(),
            name: self.name.clone(),
            text: self.text.clone(),
        }
    }
}
pub struct TestRunError {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub level: String,
    pub content: String,
}

impl TestRunError {
    fn to_api(&self) -> ApiError {
        ApiError {
            id: self.id,
            timestamp: utils::date::format_datetime(self.timestamp),
            level: self.level.clone(),
            content: self.content.clone(),
        }
    }
}

pub struct SuiteKeywords {
    pub keywords: Arc<HashMap<String, Value>>,
}

impl SuiteKeywords {
    pub fn setup_keyword(&self) -> Option<&Value> {
        self.keywords.get("setup")
    }

    pub fn teardown_keyword(&self) -> Option<&Value> {
        self.keywords.get("teardown")
    }

    pub fn to_api(&self) -> ApiSuiteKeywords<'_> {
        ApiSuiteKeywords {
            setup_keyword: self.setup_keyword(),
            teardown_keyword: self.teardown_keyword(),
        }
    }
}
