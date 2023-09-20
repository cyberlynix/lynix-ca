use std::collections::HashMap;
use std::env;
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

fn optimize_image(image_path: &Path, width: u32, height: u32) -> Result<Vec<u8>, Box<dyn Error>> {

    // Open the image file
    let img = image::open(image_path)?;

    let resized_img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

    let webp_encoder = WebPEncoder::from_image(&resized_img)?;
    let encoded_webp = webp_encoder.encode(65f32);

    let webp: Vec<u8> = encoded_webp.as_bytes().to_vec();

    Ok(webp)
}

const BASE_IMAGE_DIR: &str = "../../static/imgs/";

#[get("/api/img")]
async fn optimize_image_handler(
    query_params: web::Query<ImageQueryParams>
) -> HttpResponse {


    let path = &query_params.path;
    let width = query_params.width;
    let height = query_params.height;

    // Get the current working directory
    let current_dir = env::current_dir().unwrap(); // You may want to handle errors here

    // Build the full image path by joining with "./static/imgs"
    let image_path = current_dir.join(BASE_IMAGE_DIR).join(&path);

    // Check if the image path is within the BASE_IMAGE_DIR
    if !image_path.canonicalize().map_or(false, |canonical| {
        canonical.starts_with(PathBuf::from(BASE_IMAGE_DIR).canonicalize().ok().unwrap())
    }) {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }



        // Check if the image file exists
        let image_file = Path::new(&image_path);
        if !image_file.exists() {
            return HttpResponse::NotFound().body("Image not found");
        }

        // Attempt to optimize and convert the image (as before)
        match optimize_image(image_file, width, height) {
            Ok(webp_data) => {
                // Respond with the WebP image
                HttpResponse::Ok()
                    .content_type("image/webp")
                    .body(webp_data)
            }
            Err(_) => HttpResponse::InternalServerError().body("Image processing error"),
        }
}