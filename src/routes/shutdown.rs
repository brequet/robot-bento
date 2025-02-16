// routes/shutdown.rs
use actix_web::{web, HttpResponse};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct ShutdownHandler {
    shutdown_flag: Arc<AtomicBool>,
}

impl ShutdownHandler {
    pub fn new(shutdown_flag: Arc<AtomicBool>) -> Self {
        Self { shutdown_flag }
    }

    pub fn init(cfg: &mut web::ServiceConfig, shutdown_flag: Arc<AtomicBool>) {
        let handler = web::Data::new(ShutdownHandler::new(shutdown_flag));
        cfg.app_data(handler.clone())
            .service(web::resource("/api/stop").route(web::post().to(shutdown)));
    }
}

async fn shutdown(handler: web::Data<ShutdownHandler>) -> HttpResponse {
    handler.shutdown_flag.store(true, Ordering::SeqCst);
    HttpResponse::Ok().json("Server shutdown initiated")
}
