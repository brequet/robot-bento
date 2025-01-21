use std::path::PathBuf;

use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use actix_web::{web, HttpResponse, Responder};

use serde::Deserialize;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/upload").route(web::post().to(upload_robot_file)));
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "500MB")]
    pub file: TempFile,
    pub data: MpJson<Metadata>,
}

async fn upload_robot_file(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    print!(
        "Uploaded file {}, with size: {}",
        form.data.name, form.file.size
    );

    let upload_dir = "./uploads";

    if let Err(e) = std::fs::create_dir_all(upload_dir) {
        return HttpResponse::InternalServerError()
            .body(format!("Failed to create upload directory: {}", e));
    }

    let file_name = form.file.file_name.unwrap_or_else(|| "uploaded.xml".into());
    let dest_path: PathBuf = [upload_dir, &file_name].iter().collect();

    if let Err(e) = form.file.file.persist(&dest_path) {
        return HttpResponse::InternalServerError().body(format!("Failed to save the file: {}", e));
    }

    HttpResponse::Ok().body("File uploaded successfully.")
}
