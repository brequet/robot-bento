use actix_files::Files;
use actix_multipart::{form::MultipartFormConfig, MultipartError};
use actix_web::{web, App, Error, HttpRequest, HttpServer};
use dotenv;
mod config;
mod models;
mod repositories;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();

    let pool = config::database::setup_database().await;

    let server_config = config::server::load();
    let addr = format!("127.0.0.1:{}", server_config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(1024 * 1024 * 1024) // 1 GB
                    .memory_limit(10 * 1024 * 1024) // 10 MB
                    .error_handler(handle_multipart_error),
            )
            .configure(|cfg| routes::robot::init(cfg, web::Data::new(pool.clone())))
            .service(Files::new("/", "./frontend/build").index_file("index.html"))
    })
    .bind(addr)?
    .run()
    .await
}

fn handle_multipart_error(err: MultipartError, _req: &HttpRequest) -> Error {
    print!("Multipart error: {}", err);
    return Error::from(err);
}
