use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use image::{DynamicImage, EncodableLayout, ImageFormat};
use std::path::{Path, PathBuf};
use image::imageops::FilterType;
use serde::Deserialize;
use webp::Encoder as WebPEncoder;

#[derive(Deserialize)]
pub struct ImageQueryParams {
    height: u32,
    width: u32,
    path: String
}

fn optimize_image(image_path: &Path, width: u32, height: &u32) -> Result<Vec<u8>, Box<dyn Error>> {

    // Open the image file
    let img = image::open(image_path)?;

    let resized_img = img.resize(width, *height, image::imageops::FilterType::Lanczos3);

    let webp_encoder = WebPEncoder::from_image(&resized_img)?;
    let encoded_webp = webp_encoder.encode(65f32);

    let webp: Vec<u8> = encoded_webp.as_bytes().to_vec();

    Ok(webp)
}

const BASE_IMAGE_DIR: &str = "./static/imgs/";

fn is_valid_path(path: String) -> bool {
    // Check if the path starts with the base image directory
    path.starts_with(BASE_IMAGE_DIR)
}

#[get("/api/img")]
async fn optimize_image_handler(
    query_params: web::Query<ImageQueryParams>
) -> HttpResponse {


    let path = &query_params.path;
    let width = &query_params.width;
    let height = &query_params.height;

    // Construct the full path to the image
    let image_path = format!("./static/imgs/{}", path);

    if is_valid_path(image_path) {
        let image_path = format!("{}{}", BASE_IMAGE_DIR, path);

        // Check if the image file exists
        let image_file = Path::new(&image_path);
        if !image_file.exists() {
            return HttpResponse::NotFound().body("Image not found");
        }

        // Attempt to optimize and convert the image to WebP with custom dimensions
        match optimize_image(image_file, *width, height) {
            Ok(webp_data) => {
                // Respond with the WebP image
                HttpResponse::Ok()
                    .content_type("image/webp")
                    .body(webp_data)
            }
            Err(_) => HttpResponse::InternalServerError().body("Image processing error"),
        }
    } else {
        HttpResponse::InternalServerError().body(format!("[FloofOptimizer] Unauthorized Access to System Files! Tried to Access Path: {} <br/>This incident will <strong>Reported & Logged</strong>!", path))
    }
}