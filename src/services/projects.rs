use std::f32::consts::E;

use sqlx::PgPool;
use tracing::info;

use crate::{
    models::projects::{Project, ProjectOverview, ProjectTestRun},
    repositories::projects::{ProjectDB, ProjectsRepository},
};

use super::robot::RobotService;

pub struct ProjectsService;

impl ProjectsService {
    pub async fn get_projects_overview(
        pool: &PgPool,
    ) -> Result<Vec<ProjectOverview>, Box<dyn std::error::Error>> {
        let projects = ProjectsRepository::get_projects(pool).await?;

        let project_ids: Vec<i32> = projects.iter().map(|p| p.id.unwrap()).collect();
        let project_test_run_data =
            RobotService::get_test_run_data_by_project_ids(pool, &project_ids).await?;

        let project_overviews = projects
            .iter()
            .map(|project| {
                let test_run_data = project_test_run_data
                    .iter()
                    .find(|data| data.project_id == project.id.unwrap());

                let mut test_run_count = 0;
                if let Some(data) = test_run_data {
                    test_run_count = data.test_run_count;
                }

                let project_test_run = match test_run_data {
                    Some(data) => {
                        if data.last_test_run_date.is_some() {
                            Some(ProjectTestRun {
                                last_test_run_date: data.last_test_run_date.unwrap(),
                                total_tests: data.last_total_tests.unwrap(),
                                passed_tests: data.last_passed_tests.unwrap(),
                                failed_tests: data.last_failed_tests.unwrap(),
                                skipped_tests: data.last_skipped_tests.unwrap(),
                            })
                        } else {
                            None
                        }
                    }
                    None => None,
                };

                ProjectOverview {
                    id: project.id.unwrap(),
                    name: project.name.clone(),
                    test_run_count: test_run_count,
                    last_test_run: project_test_run,
                }
            })
            .collect();

        Ok(project_overviews)
    }

    pub async fn upsert_project_by_name(
        pool: &PgPool,
        project_name: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let project_id = ProjectsRepository::get_project_id_by_name(pool, project_name).await?;
        match project_id {
            Some(id) => {
                info!("Project {} already exists", project_name);
                Ok(id)
            }
            None => {
                info!("Creating project {}", project_name);
                let project = ProjectDB {
                    id: None,
                    name: project_name.to_string(),
                    create_date: None,
                };
                let project_id = ProjectsRepository::insert_project(pool, project).await?;
                Ok(project_id)
            }
        }
    }

    pub async fn get_project_by_id(
        pool: &PgPool,
        project_id: i32,
    ) -> Result<Option<Project>, Box<dyn std::error::Error>> {
        let project_data = ProjectsRepository::get_project_by_id(pool, project_id).await?;
        match project_data {
            Some(project_data) => {
                let test_runs =
                    RobotService::get_all_test_runs_by_project_id(pool, project_id).await?;
                Ok(Some(Project {
                    id: project_data.id.unwrap(),
                    name: project_data.name,
                    test_run_count: test_runs.len() as i32,
                    test_runs: test_runs,
                }))
            }
            None => Ok(None),
        }
    }
}
