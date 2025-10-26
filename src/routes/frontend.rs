use actix_web::{web, HttpRequest, HttpResponse, Responder};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "frontend/build/"]
struct FrontendAssets;

pub struct FrontendHandler;

impl FrontendHandler {
    pub fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("").default_service(web::to(FrontendHandler::serve_frontend)));
    }

    async fn serve_frontend(req: HttpRequest) -> impl Responder {
        let path = &req.path()[1..];

        match FrontendAssets::get(path) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                HttpResponse::Ok()
                    .content_type(mime.as_ref())
                    .body(content.data.into_owned())
            }
            None => {
                // Serve index.html for unknown paths to support client-side routing
                match FrontendAssets::get("index.html") {
                    Some(index) => HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(index.data.into_owned()),
                    None => HttpResponse::NotFound().body("404 Not Found"),
                }
            }
        }
    }
}
