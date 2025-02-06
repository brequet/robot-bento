use std::sync::Arc;

use actix_cors::Cors;
use actix_multipart::{form::MultipartFormConfig, MultipartError};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv;
use mime_guess;
mod config;
mod models;
mod repositories;
mod routes;
mod services;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "frontend/build/"]
struct Assets;

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

    HttpServer::new(move || {
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
            .service(web::scope("").default_service(web::to(serve_frontend)))
    })
    .bind(addr)?
    .run()
    .await
}

async fn serve_frontend(req: HttpRequest) -> impl Responder {
    let path = if req.path() == "/" {
        "index.html"
    } else {
        &req.path()[1..]
    };

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime.as_ref())
                .body(content.data.into_owned())
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

fn handle_multipart_error(err: MultipartError, _req: &HttpRequest) -> Error {
    print!("Multipart error: {}", err);
    return Error::from(err);
}
