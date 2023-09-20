mod views;
mod utils;
mod error_types;
mod optimizer;

use std::sync::{Arc, Mutex};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, Responder, web, Error};
use actix::{Actor, StreamHandler};
use actix_files as fs;
use crate::views::index::{show_disabled, show_not_found};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .service(fs::Files::new("/static", "static/"))
            /*.service(
                web::scope("")
                    .service(views::index::show_index)
                    .service(views::index::show_reactive)
                    .service(views::fursona::show_fursona)
                    .service(views::shock::show_shock)
            )*/
            .default_service(
                web::get().to(show_disabled)
            )
    })
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}