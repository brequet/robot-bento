use actix_multipart::{form::MultipartFormConfig, MultipartError};
use actix_web::{App, Error, HttpRequest, HttpServer};
mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_config = config::server::load();
    let addr = format!("127.0.0.1:{}", server_config.port);

    println!("Running on {}", addr);

    HttpServer::new(|| {
        App::new()
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(1024 * 1024 * 1024) // 1 GB
                    .memory_limit(10 * 1024 * 1024) // 10 MB
                    .error_handler(handle_multipart_error),
            )
            .configure(routes::ingest::init)
    })
    .bind(addr)?
    .run()
    .await
}

fn handle_multipart_error(err: MultipartError, _req: &HttpRequest) -> Error {
    print!("Multipart error: {}", err);
    return Error::from(err);
}
