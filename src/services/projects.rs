use std::sync::Arc;

use tracing::info;

use crate::{
    models::{
        projects::{api::ProjectOverviewResponse, domain::NewProject},
        projects_legacy::Project,
    },
    repositories::projects::ProjectsRepository,
    utils,
};

use super::robot::RobotService;

pub struct ProjectsService {
    repository: ProjectsRepository,
    robot_service: Arc<RobotService>,
}

impl ProjectsService {
    pub fn new(repository: ProjectsRepository, robot_service: Arc<RobotService>) -> Self {
        Self {
            repository,
            robot_service,
        }
    }

    pub async fn get_projects_overview(
        &self,
    ) -> Result<Vec<ProjectOverviewResponse>, Box<dyn std::error::Error>> {
        let projects = self.repository.get_projects().await?;
        let project_ids = &projects.iter().map(|project| project.id).collect();

        let projects_test_run_data = self
            .robot_service
            .get_test_runs_data_by_project_ids(project_ids)
            .await?;

        let project_overviews = projects
            .iter()
            .map(|project| {
                let test_run_data = projects_test_run_data
                    .iter()
                    .find(|data| data.project_id == project.id);

                let test_run_count = test_run_data.map(|data| data.test_run_count).unwrap_or(0);

                ProjectOverviewResponse {
                    id: project.id,
                    name: project.name.clone(),
                    create_date: utils::date::format_datetime(project.create_date),
                    test_run_count: test_run_count,
                    last_test_run_summary: test_run_data.map(|data| data.to_api()),
                }
            })
            .collect();

        Ok(project_overviews)
    }

    pub async fn get_or_create_project_by_name(
        &self,
        project_name: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let project_id = self.repository.get_project_id_by_name(project_name).await?;
        match project_id {
            Some(id) => {
                info!("Project {} already exists", project_name);
                Ok(id)
            }
            None => {
                info!("Creating project {}", project_name);
                let new_project = NewProject {
                    name: project_name.to_string(),
                };
                let saved_project = self.repository.insert_project(new_project).await?;
                Ok(saved_project.id)
            }
        }
    }

    pub async fn get_project_by_id(
        &self,
        project_id: i32,
    ) -> Result<Option<Project>, Box<dyn std::error::Error>> {
        let project_data = self.repository.get_project_by_id(project_id).await?;
        match project_data {
            Some(project_data) => {
                let test_runs = self
                    .robot_service
                    .get_all_test_runs_by_project_id(project_id)
                    .await?;
                Ok(Some(Project {
                    id: project_data.id,
                    name: project_data.name,
                    test_run_count: test_runs.len() as i32,
                    test_runs: test_runs,
                }))
            }
            None => Ok(None),
        }
    }
}
