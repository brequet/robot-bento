use actix_web::{get, web, Error, HttpResponse};
use sqlx::PgPool;
use tracing::error;

use crate::services::projects::ProjectsService;

pub fn init(cfg: &mut web::ServiceConfig, pool: web::Data<PgPool>) {
    cfg.app_data(pool).service(
        web::scope("/api/projects")
            .service(get_all_projects)
            .service(get_project_by_id),
    );
}

#[get("")]
async fn get_all_projects(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    match ProjectsService::get_projects_overview(&pool).await {
        Ok(projects) => Ok(HttpResponse::Ok().json(projects)),
        Err(e) => {
            error!("Error getting projects: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[get("/{id}")]
async fn get_project_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    match ProjectsService::get_project_by_id(&pool, path.into_inner()).await {
        Ok(Some(project)) => Ok(HttpResponse::Ok().json(project)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => {
            error!("Error getting project: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
