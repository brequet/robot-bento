use serde_json::Value;
use tracing::{info, warn};

use crate::{
    models::{
        self,
        robot::domain::{ProjectTestRunSummary, SuiteKeywords},
    },
    repositories::robot::RobotRepository,
};

use super::{mappers, parser::TestRun};

pub struct TestRunMetadata {
    pub app_name: String,
    pub app_version: String,
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

    pub async fn get_latest_test_runs_data_by_project_ids(
        &self,
        project_ids: &Vec<i32>,
    ) -> Result<Vec<ProjectTestRunSummary>, Box<dyn std::error::Error>> {
        if project_ids.is_empty() {
            return Ok(vec![]);
        }

        let summaries = self
            .repository
            .get_latest_test_runs_summaries_for_projects(project_ids)
            .await?;
        Ok(summaries)
    }

    pub async fn get_test_runs_summaries_by_project_id(
        &self,
        project_id: i32,
    ) -> Result<Vec<ProjectTestRunSummary>, Box<dyn std::error::Error>> {
        let summaries = self
            .repository
            .get_test_runs_summaries_by_project_id(project_id)
            .await?;
        Ok(summaries)
    }

    pub async fn get_test_run_by_id(
        &self,
        id: i32,
    ) -> Result<Option<models::robot::domain::SavedTestRun>, Box<dyn std::error::Error>> {
        let test_run = self.repository.get_test_run_by_id(id).await?;
        Ok(test_run)
    }

    pub async fn get_suite_keywords_by_suite_id(
        &self,
        id: i32,
    ) -> Result<Option<SuiteKeywords>, Box<dyn std::error::Error>> {
        let keywords = self.repository.get_suite_keywords_by_suite_id(id).await?;
        Ok(keywords)
    }

    pub async fn get_test_keywords_by_test_id(
        &self,
        id: i32,
    ) -> Result<Option<Value>, Box<dyn std::error::Error>> {
        let keywords = self.repository.get_test_keywords_by_test_id(id).await?;
        Ok(keywords)
    }
}
