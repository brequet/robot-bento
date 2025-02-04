use actix_web::{get, web, Error, HttpResponse};
use sqlx::PgPool;
use tracing::error;

use crate::services::{projects::ProjectsService, robot::RobotService};

pub fn init(cfg: &mut web::ServiceConfig, pool: web::Data<PgPool>) {
    cfg.app_data(pool)
        .service(web::scope("/api/projects").service(get_all_projects));
}

#[get("")]
async fn get_all_projects(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let projects = ProjectsService::get_projects_overview(&pool).await;

    match projects {
        Ok(projects) => Ok(HttpResponse::Ok().json(projects)),
        Err(e) => {
            error!("Error getting projects: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
