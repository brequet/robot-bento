use std::sync::Arc;

use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use actix_web::{web, Error, HttpResponse, Scope};
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info};

use crate::services::{
    self,
    parser::{ParserError, RobotOutputParserService},
    projects::ProjectsService,
    robot::RobotService,
};

// TODO: move to api model layer
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
    #[multipart(limit = "1000MB")]
    pub file: TempFile,
    pub metadata: MpJson<RobotTestRunMetadata>,
}

pub struct RobotHandler {
    robot_service: Arc<RobotService>,
    projects_service: Arc<ProjectsService>,
    robot_output_parser_service: Arc<RobotOutputParserService>,
}

impl RobotHandler {
    fn new(
        robot_service: Arc<RobotService>,
        projects_service: Arc<ProjectsService>,
        robot_output_parser_service: Arc<RobotOutputParserService>,
    ) -> Self {
        RobotHandler {
            robot_service,
            projects_service,
            robot_output_parser_service,
        }
    }

    pub fn init(
        cfg: &mut web::ServiceConfig,
        robot_service: Arc<RobotService>,
        projects_service: Arc<ProjectsService>,
        robot_output_parser_service: Arc<RobotOutputParserService>,
    ) {
        let handler =
            RobotHandler::new(robot_service, projects_service, robot_output_parser_service);
        cfg.service(handler.routes());
    }

    fn routes(&self) -> Scope {
        web::scope("/api/robot")
            .app_data(web::Data::new(self.robot_service.clone()))
            .app_data(web::Data::new(self.projects_service.clone()))
            .app_data(web::Data::new(self.robot_output_parser_service.clone()))
            .route("/test-runs/{id}", web::get().to(Self::get_test_run))
            .route(
                "/suites/{id}/keywords",
                web::get().to(Self::get_suite_keywords),
            )
            .route(
                "/tests/{id}/keywords",
                web::get().to(Self::get_test_keywords),
            )
            .route("/upload", web::post().to(Self::upload_robot_output))
    }

    async fn get_test_run(
        robot_service: web::Data<Arc<RobotService>>,
        test_run_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let test_run = robot_service
            .get_test_run_by_id(test_run_id.into_inner())
            .await;

        match test_run {
            Ok(Some(test_run)) => Ok(HttpResponse::Ok().json(test_run.to_response())),
            Ok(None) => Ok(HttpResponse::NotFound().finish()),
            Err(e) => {
                error!("Error getting test run: {:?}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        }
    }

    async fn get_suite_keywords(
        robot_service: web::Data<Arc<RobotService>>,
        suite_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let keywords = robot_service
            .get_suite_keywords_by_suite_id(suite_id.into_inner())
            .await;

        match keywords {
            Ok(Some(keywords)) => Ok(HttpResponse::Ok().json(keywords.to_api())),
            Ok(None) => Ok(HttpResponse::NotFound().finish()),
            Err(e) => {
                error!("Error getting keywords: {:?}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        }
    }

    async fn get_test_keywords(
        robot_service: web::Data<Arc<RobotService>>,
        test_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let keywords = robot_service
            .get_test_keywords_by_test_id(test_id.into_inner())
            .await;

        match keywords {
            Ok(Some(keywords)) => Ok(HttpResponse::Ok().json(keywords)),
            Ok(None) => Ok(HttpResponse::NotFound().finish()),
            Err(e) => {
                error!("Error getting keywords: {:?}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        }
    }

    async fn upload_robot_output(
        MultipartForm(form): MultipartForm<RobotOuputUploadForm>,
        robot_service: web::Data<Arc<RobotService>>,
        projects_service: web::Data<Arc<ProjectsService>>,
        robot_output_parser_service: web::Data<Arc<RobotOutputParserService>>,
    ) -> Result<HttpResponse, Error> {
        let file_name = form.file.file_name.unwrap_or_default();
        info!(
            "Processing upload: {} [{} - {}]",
            file_name, form.metadata.app_name, form.metadata.app_version
        );
        if form.metadata.app_name.is_empty() {
            error!("Missing appName");
            return Ok(HttpResponse::BadRequest().json(json!({
                "error": "Missing appName"
            })));
        }

        let file_path = form.file.file.path();

        // TODO: check sha1 uniqueness BEFORE parsing...
        match robot_output_parser_service.from_file(file_name, file_path) {
            Ok(test_run) => {
                let metadata = services::robot::TestRunMetadata {
                    app_name: form.metadata.app_name.clone(),
                    app_version: form.metadata.app_version.clone(),
                };

                let project_id = projects_service
                    .get_or_create_project_by_name(form.metadata.app_name.as_str())
                    .await?;

                match robot_service
                    .save_test_run(test_run, metadata, project_id)
                    .await
                {
                    Ok(_) => Ok(HttpResponse::Ok().finish()),
                    Err(e) => {
                        let error_message = format!("Failed to save test run: {}", e);
                        error!("{}", error_message);
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
}
