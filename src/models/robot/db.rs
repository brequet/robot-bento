use chrono::NaiveDateTime;
use serde_json::Value;
use sqlx::postgres::types::PgInterval;

use crate::utils;

use super::domain::{
    ProjectTestRunSummary, SavedTestRun, TestRunError, TestRunStatistic, TestRunSuite, TestRunTest,
};

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

#[derive(sqlx::FromRow)]
pub struct TestRunDB {
    pub id: i32,
    pub project_id: i32,
    pub rpa: bool,
    pub generator: String,
    pub generated_date: NaiveDateTime,
    pub schema_version: String,
    pub application_version: String,
    pub imported_date: NaiveDateTime,
}

impl TestRunDB {
    pub fn to_test_run(
        &self,
        suites: Vec<TestRunSuite>,
        statistics: Vec<TestRunStatistic>,
        errors: Vec<TestRunError>,
    ) -> SavedTestRun {
        SavedTestRun {
            id: self.id,
            project_id: self.project_id,
            rpa: self.rpa,
            generator: self.generator.clone(),
            generated_date: self.generated_date,
            schema_version: self.schema_version.clone(),
            app_version: self.application_version.clone(),
            imported_date: self.imported_date,
            suites: suites,
            statistics: statistics,
            errors: errors,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct SuiteDB {
    pub id: i32,
    pub name: String,
    pub source: Option<String>,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub doc: Option<String>,
    pub identifier: String,
}

impl SuiteDB {
    pub fn to_test_run_suite(
        &self,
        sub_suites: Vec<TestRunSuite>,
        tests: Vec<TestRunTest>,
    ) -> TestRunSuite {
        TestRunSuite {
            id: self.id,
            name: self.name.clone(),
            source: self.source.clone(),
            status: self.status.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
            doc: self.doc.clone(),
            identifier: self.identifier.clone(),
            suites: sub_suites,
            tests: tests,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct TestDB {
    pub id: i32,
    pub identifier: String,
    pub name: String,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub line: i32,
    pub doc: Option<String>,
    pub timeout: Option<String>,
    pub tag: Option<String>,
}

impl TestDB {
    pub fn to_(&self, tags: Vec<String>) -> TestRunTest {
        TestRunTest {
            id: self.id,
            name: self.name.clone(),
            line: self.line,
            identifier: self.identifier.clone(),
            tags: tags,
            status: self.status.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
            doc: self.doc.clone(),
            timeout: self.timeout.clone(),
        }
    }
}

#[derive(sqlx::Type, Clone)]
#[sqlx(type_name = "stat_type", rename_all = "snake_case")]
pub enum StatisticTypeDB {
    Total,
    Tag,
    Suite,
}

#[derive(sqlx::FromRow)]
pub struct StatisticDB {
    pub id: i32,
    pub stat_type: StatisticTypeDB,
    pub pass_count: i32,
    pub fail_count: i32,
    pub skip_count: i32,
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub text: String,
}

impl StatisticDB {
    pub fn to_test_run_statistic(&self) -> TestRunStatistic {
        TestRunStatistic {
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

#[derive(sqlx::FromRow)]
pub struct ErrorDB {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub level: String,
    pub content: String,
}

impl ErrorDB {
    pub fn to_test_run_error(&self) -> TestRunError {
        TestRunError {
            id: self.id,
            timestamp: self.timestamp,
            level: self.level.clone(),
            content: self.content.clone(),
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct SuiteKeywordRecord {
    pub keyword_type: String,
    pub value: Value,
}

#[derive(sqlx::FromRow, Debug)]
pub struct RawJsonRecord {
    pub value: Value,
}
