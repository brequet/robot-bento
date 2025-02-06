use actix_web::{web, Error, HttpResponse, Scope};
use std::sync::Arc;
use tracing::error;

use crate::services::projects::ProjectsService;

pub struct ProjectsHandler {
    projects_service: Arc<ProjectsService>,
}

impl ProjectsHandler {
    fn new(service: Arc<ProjectsService>) -> Self {
        ProjectsHandler {
            projects_service: service,
        }
    }

    pub fn init(cfg: &mut web::ServiceConfig, projects_service: Arc<ProjectsService>) {
        let handler = ProjectsHandler::new(projects_service);
        cfg.service(handler.routes());
    }

    fn routes(&self) -> Scope {
        web::scope("/api/projects")
            .app_data(web::Data::new(self.projects_service.clone()))
            .route("/overview", web::get().to(Self::get_projects_overview))
            .route("/{id}", web::get().to(Self::get_project_by_id))
    }

    async fn get_projects_overview(
        projects_service: web::Data<Arc<ProjectsService>>,
    ) -> Result<HttpResponse, Error> {
        match projects_service.get_projects_overview().await {
            Ok(projects) => Ok(HttpResponse::Ok().json(projects)),
            Err(e) => {
                error!("Error getting projects: {:?}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        }
    }

    async fn get_project_by_id(
        projects_service: web::Data<Arc<ProjectsService>>,
        path: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        match projects_service.get_project_by_id(path.into_inner()).await {
            Ok(Some(project)) => Ok(HttpResponse::Ok().json(project)),
            Ok(None) => Ok(HttpResponse::NotFound().finish()),
            Err(e) => {
                error!("Error getting project: {:?}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        }
    }
}
