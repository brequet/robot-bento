use actix_web::{get, web, Error, HttpResponse};
use sqlx::PgPool;
use tracing::error;

use crate::services::{self, robot::RobotService};

pub fn init(cfg: &mut web::ServiceConfig, pool: web::Data<PgPool>) {
    cfg.app_data(pool).service(
        web::scope("/api/robot")
            .service(get_all_test_runs)
            .service(get_test_run),
    );
}

#[get("/test-runs")]
async fn get_all_test_runs(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let test_runs = RobotService::get_all_test_runs(&pool).await;

    match test_runs {
        Ok(test_runs) => Ok(HttpResponse::Ok().json(test_runs)),
        Err(e) => {
            error!("Error getting test runs: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[get("/test-runs/{id}")]
async fn get_test_run(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let test_run = RobotService::get_test_run_by_id(&pool, path.into_inner()).await;

    match test_run {
        Ok(Some(test_run)) => Ok(HttpResponse::Ok().json(test_run)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => {
            error!("Error getting test run: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
