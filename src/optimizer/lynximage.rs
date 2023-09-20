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

const BASE_IMAGE_DIR: &str = "../../src/static/imgs/";

fn contains_special_characters(path: &String) -> bool {
// Remove occurrences of ".." from the path
    let sanitized_path = path.replace("..", ".");

// Define a list of disallowed characters or patterns
    let disallowed_characters = &['\\', '~'];

// Check if the sanitized path contains any disallowed characters
    sanitized_path.chars().any(|c| disallowed_characters.contains(&c))
}

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
    let image_path = current_dir.join("./static/imgs").join(&path);

    // Normalize the image path
    let normalized_image_path = image_path.canonicalize().ok();

    // Check if the image path couldn't be normalized or contains special characters
    if normalized_image_path.is_none() || contains_special_characters(&path) {
        let error_message = "Unauthorized Access";
        return HttpResponse::InternalServerError().body(error_message);
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