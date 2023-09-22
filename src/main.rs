mod views;
mod utils;
mod error_types;
mod optimizer;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, Responder, web, Error, http};
use actix::{Actor, StreamHandler};
use actix_files as fs;
use actix_governor::{Governor, GovernorConfigBuilder};
use crate::views::index::{show_disabled, show_not_found};
use actix_cors::Cors;
use actix_web::middleware::DefaultHeaders;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(
                Cors::default()
                    .allowed_origin("https://lynix.ca")
                    .supports_credentials()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(
                DefaultHeaders::new()
                    .add(("Server", env!("CARGO_PKG_NAME")))
                    // Standard security headers
                    .add(("X-XSS-Protection", "0"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
                /*.add((
                    "Cross-Origin-Embedder-Policy",
                    "require-corp; report-to=\"default\";",
                ))
                .add(
                    ("Content-Security-Policy",
                        "default-src 'none' style-src 'self'")
                )*/
                    /*.add((
                        "Cross-Origin-Opener-Policy",
                        "same-site; report-to=\"default\";",
                    ))
                    .add(("Cross-Origin-Resource-Policy", "same-site")),*/
            )
            .wrap(Governor::new(
                &GovernorConfigBuilder::default()
                    .per_second(3)
                    .burst_size(20)
                    .finish()
                    .unwrap(),
            ))
            .service(fs::Files::new("/static", "static/"))
            .service(
                web::scope("")
                    .service(views::index::show_index)
                    .service(views::index::show_reactive)
                    .service(views::fursona::show_fursona)
                    .service(views::shock::show_shock)
                    .service(optimizer::lynximage::optimize_image_handler)
            )
            .default_service(
                web::get().to(show_not_found)
            )
    })
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}