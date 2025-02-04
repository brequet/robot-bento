use sqlx::PgPool;
use tracing::info;

use crate::{
    models::projects::ProjectOverview,
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

        Ok(projects)
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
}
