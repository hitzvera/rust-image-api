use actix_web::{web, App, HttpServer, HttpResponse, Result};
use image::{ImageFormat, DynamicImage};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::io::Cursor;

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct ThumbnailParams {
    width: u32,
    height: u32,
}

#[derive(Debug, Serialize)]
struct ProcessResponse {
    success: bool,
    image_id: String,
    size_bytes: usize,
    processing_time_ms: u64,
}

/// Health check endpoint
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        version: "0.1.0".to_string(),
    })
}

/// Generate thumbnail from uploaded image
async fn generate_thumbnail(body: web::Bytes) -> Result<HttpResponse> {
    let start = std::time::Instant::now();
    
    // Load image from bytes
    let img = match image::load_from_memory(&body) {
        Ok(img) => img,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid image")),
    };
    
    // Resize to thumbnail (example: 200x200)
    let thumbnail = img.thumbnail(200, 200);
    
    // Encode as JPEG
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    thumbnail.write_to(&mut cursor, ImageFormat::Jpeg).unwrap();
    
    let processing_time = start.elapsed().as_millis() as u64;
    let image_id = Uuid::new_v4().to_string();
    
    Ok(HttpResponse::Ok().json(ProcessResponse {
        success: true,
        image_id,
        size_bytes: buffer.len(),
        processing_time_ms: processing_time,
    }))
}

/// Simple resize endpoint
async fn resize_image(body: web::Bytes, query: web::Query<ThumbnailParams>) -> Result<HttpResponse> {
    let start = std::time::Instant::now();
    
    // Load and resize image
    let img = match image::load_from_memory(&body) {
        Ok(img) => img,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid image")),
    };
    
    let resized = img.resize(query.width, query.height, image::imageops::FilterType::Lanczos3);
    
    // Encode as JPEG
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    resized.write_to(&mut cursor, ImageFormat::Jpeg).unwrap();
    
    let processing_time = start.elapsed().as_millis() as u64;
    let image_id = Uuid::new_v4().to_string();
    
    Ok(HttpResponse::Ok().json(ProcessResponse {
        success: true,
        image_id,
        size_bytes: buffer.len(),
        processing_time_ms: processing_time,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    log::info!("Starting Rust Image API server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/api/v1/thumbnail", web::post().to(generate_thumbnail))
            .route("/api/v1/resize", web::post().to(resize_image))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
