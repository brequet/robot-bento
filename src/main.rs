use actix_multipart::{form::MultipartFormConfig, test, MultipartError};
use actix_web::{web, App, Error, HttpRequest, HttpServer};
use dotenv;
use services::parser::TestRun;
mod config;
mod models;
mod repositories;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = config::database::setup_database().await;

    let file_path = "./src/services/resources/output_simplified.xml";
    // let file_path = "./robot-data-sample/8-tests-1-ko/output.xml";
    let result = services::parser::get_test_run_from_xml(&file_path);
    match result {
        Ok(test_run) => {
            if let Err(e) = services::robot::RobotService::save_test_run(&pool, test_run).await {
                println!("Ingest error: {:?}", e);
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    let test_run = services::robot::RobotService::get_test_run_by_id(&pool, 1).await;
    match test_run {
        Ok(test_run) => match test_run {
            Some(test_run) => {
                println!("Test run: {:?}", test_run);
            }
            None => {
                println!("Test run not found");
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    let server_config = config::server::load();
    let addr = format!("127.0.0.1:{}", server_config.port);

    println!("Running on {}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(1024 * 1024 * 1024) // 1 GB
                    .memory_limit(10 * 1024 * 1024) // 10 MB
                    .error_handler(handle_multipart_error),
            )
            .configure(routes::ingest::init)
            .configure(|cfg| routes::robot::init(cfg, web::Data::new(pool.clone())))
    })
    .bind(addr)?
    .run()
    .await
}

fn handle_multipart_error(err: MultipartError, _req: &HttpRequest) -> Error {
    print!("Multipart error: {}", err);
    return Error::from(err);
}
