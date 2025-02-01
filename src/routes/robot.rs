use actix_web::{get, web, Error, HttpResponse};
use sqlx::PgPool;

use crate::services;

pub fn init(cfg: &mut web::ServiceConfig, pool: web::Data<PgPool>) {
    cfg.app_data(pool)
        .service(web::scope("/api/robot").service(get_test_run));
}

#[get("/test/{id}")]
async fn get_test_run(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let test_run =
        services::robot::RobotService::get_test_run_by_id(&pool, path.into_inner()).await;

    match test_run {
        Ok(Some(test_run)) => Ok(HttpResponse::Ok().json(test_run)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => {
            eprintln!("Error getting test run: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
