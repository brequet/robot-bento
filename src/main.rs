use actix_cors::Cors;
use actix_multipart::{form::MultipartFormConfig, MultipartError};
use actix_web::{App, Error, HttpRequest, HttpServer};
use dotenv;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::error;

mod config;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();

    let pool = config::database::setup_database().await;

    let robot_repository = repositories::robot::RobotRepository::new(pool.clone());
    let projects_repository = repositories::projects::ProjectsRepository::new(pool);

    let robot_output_parser_service = Arc::new(services::parser::RobotOutputParserService::new());
    let robot_service = Arc::new(services::robot::RobotService::new(robot_repository));
    let projects_service = Arc::new(services::projects::ProjectsService::new(
        projects_repository,
        Arc::clone(&robot_service),
    ));

    let server_config = config::server::load();
    let addr = format!("127.0.0.1:{}", server_config.port);

    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let flag_clone = shutdown_flag.clone();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(1024 * 1024 * 1024) // 1 GB
                    .memory_limit(10 * 1024 * 1024) // 10 MB
                    .error_handler(handle_multipart_error),
            )
            .configure(|cfg| {
                routes::robot::RobotHandler::init(
                    cfg,
                    Arc::clone(&robot_service),
                    Arc::clone(&projects_service),
                    Arc::clone(&robot_output_parser_service),
                )
            })
            .configure(|cfg| {
                routes::projects::ProjectsHandler::init(cfg, Arc::clone(&projects_service))
            })
            .configure(|cfg| {
                routes::shutdown::ShutdownHandler::init(cfg, Arc::clone(&shutdown_flag))
            })
            .configure(|cfg| routes::frontend::FrontendHandler::init(cfg))
    })
    .bind(addr)?
    .run();

    let shutdown_check = async move {
        while !flag_clone.load(Ordering::SeqCst) {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    };

    tokio::select! {
        _ = server => println!("Server stopped normally"),
        _ = shutdown_check => println!("Shutdown signal received"),
    }

    Ok(())
}

fn handle_multipart_error(err: MultipartError, _req: &HttpRequest) -> Error {
    error!("Multipart error: {}", err);
    return Error::from(err);
}
