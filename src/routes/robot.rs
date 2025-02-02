use actix_multipart::{
    form::{json::Json as MpJson, tempfile::TempFile, MultipartForm},
    test,
};
use actix_web::{get, post, web, Error, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use tracing::{error, info};

use crate::services::{
    self,
    parser::{ParserError, RobotOutputParserService},
    robot::RobotService,
};

pub fn init(cfg: &mut web::ServiceConfig, pool: web::Data<PgPool>) {
    cfg.app_data(pool).service(
        web::scope("/api/robot")
            .service(get_all_test_runs)
            .service(get_test_run)
            .service(upload_robot_output),
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

/*
Metadata ideas:
- test env (staging, prod, etc)
- triggering system (jenkins, github, etc)
- robot project git hash
- project git hash
- jenkins URL / build number for linking back
 */
#[derive(Debug, Deserialize)]
pub struct RobotTestRunMetadata {
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "appVersion")]
    pub app_version: String,
}

#[derive(Debug, MultipartForm)]
pub struct RobotOuputUploadForm {
    #[multipart(limit = "500MB")]
    pub file: TempFile,
    pub metadata: MpJson<RobotTestRunMetadata>,
}

#[post("/upload")]
async fn upload_robot_output(
    MultipartForm(form): MultipartForm<RobotOuputUploadForm>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let file_name = form.file.file_name.unwrap_or_default();
    info!(
        "Processing upload: {} [{} - {}]",
        file_name, form.metadata.app_name, form.metadata.app_version
    );

    let file_path = form.file.file.path();

    match RobotOutputParserService::from_file(file_name, file_path) {
        Ok(test_run) => {
            let metadata = services::robot::TestRunMetadata {
                app_name: form.metadata.app_name.clone(),
                app_version: form.metadata.app_version.clone(),
            };

            match RobotService::save_test_run(&pool, test_run, metadata).await {
                Ok(_) => Ok(HttpResponse::Ok().finish()),
                Err(e) => {
                    let error_message = format!("Failed to save test run: {}", e);
                    error!(error_message);
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": error_message
                    })))
                }
            }
        }
        Err(ParserError::InvalidFileExtension(message)) => {
            error!("Invalid file extension: {}", message);
            Ok(HttpResponse::BadRequest().json(json!({
                "error": "Invalid file extension, expected XML"
            })))
        }
        Err(e) => {
            error!("Failed to process XML: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to process XML file"
            })))
        }
    }
}
