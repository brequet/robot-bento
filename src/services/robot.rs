use chrono::NaiveDateTime;
use tracing::{info, warn};

use crate::{models::robot::TestRunDB, repositories::robot::RobotRepository};

use super::{mappers, parser::TestRun};

pub struct TestRunMetadata {
    pub app_name: String,
    pub app_version: String,
}

pub struct ProjectTestRunData {
    pub project_id: i32,
    pub application_name: String,
    pub application_version: String,
    pub test_run_count: i32,
    pub last_test_run_date: Option<NaiveDateTime>,
    pub last_total_tests: Option<i32>,
    pub last_passed_tests: Option<i32>,
    pub last_failed_tests: Option<i32>,
    pub last_skipped_tests: Option<i32>,
    // pub last_elapsed_time: i32,
}

pub struct RobotService {
    repository: RobotRepository,
}

impl RobotService {
    pub fn new(repository: RobotRepository) -> Self {
        Self { repository }
    }

    pub async fn save_test_run(
        &self,
        parsed_test_run: TestRun,
        metadata: TestRunMetadata,
        project_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_sha1 = parsed_test_run.sha1.as_ref();
        if self.repository.is_sha1_already_inserted(file_sha1).await? {
            warn!("Test run with sha1 {} already exists", &file_sha1);
            Err("Test run already imported")?;
        }

        let test_run = mappers::robot::map_test_run(&parsed_test_run, &metadata)?;

        info!("Saving test run with sha1 {}", file_sha1);
        let test_run_id = self
            .repository
            .insert_test_run(&test_run, project_id)
            .await?;
        info!("Saved test run, id: {}", test_run_id);
        Ok(())
    }

    // TODO clean
    pub async fn get_test_run_data_by_project_ids(
        &self,
        project_ids: &Vec<i32>,
    ) -> Result<Vec<ProjectTestRunData>, Box<dyn std::error::Error>> {
        if project_ids.is_empty() {
            return Ok(vec![]);
        }

        let test_runs_data = self
            .repository
            .get_test_run_data_by_project_ids(project_ids)
            .await?
            .iter()
            .map(|test_run_data| {
                let last_total_tests = test_run_data.last_passed_tests.unwrap_or(0)
                    + test_run_data.last_failed_tests.unwrap_or(0)
                    + test_run_data.last_skipped_tests.unwrap_or(0);
                ProjectTestRunData {
                    project_id: test_run_data.project_id.unwrap(),
                    application_name: test_run_data.application_name.clone(),
                    application_version: test_run_data.application_version.clone(),
                    test_run_count: test_run_data.test_run_count.or_else(|| Some(0)).unwrap(),
                    last_test_run_date: test_run_data.last_test_run_date,
                    last_total_tests: Some(last_total_tests),
                    last_passed_tests: test_run_data.last_passed_tests,
                    last_failed_tests: test_run_data.last_failed_tests,
                    last_skipped_tests: test_run_data.last_skipped_tests,
                }
            })
            .collect();
        Ok(test_runs_data)
    }

    pub async fn get_all_test_runs(&self) -> Result<Vec<TestRunDB>, Box<dyn std::error::Error>> {
        let test_runs = self.repository.get_all_test_runs().await?;
        Ok(test_runs)
    }

    pub async fn get_all_test_runs_by_project_id(
        &self,
        project_id: i32,
    ) -> Result<Vec<TestRunDB>, Box<dyn std::error::Error>> {
        let test_runs = self
            .repository
            .get_all_test_runs_by_project_id(project_id)
            .await?;
        Ok(test_runs)
    }

    pub async fn get_test_run_by_id(
        &self,
        id: i32,
    ) -> Result<Option<TestRunDB>, Box<dyn std::error::Error>> {
        let test_run = self.repository.get_test_run_by_id(id).await?;
        Ok(test_run)
    }
}
