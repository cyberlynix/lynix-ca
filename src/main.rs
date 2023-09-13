mod views;
mod utils;
mod error_types;
mod ws;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, Responder, web, Error};
use actix::{Actor, StreamHandler};
use actix_files as fs;
use crate::views::index::show_not_found;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .route("/ws", web::get().to(ws::ws::index))
            .service(fs::Files::new("/static", "static/"))
            .service(
                web::scope("")
                    .service(views::index::show_index)
                    .service(views::fursona::show_fursona)
                    .service(views::shock::show_shock)
            )
            .default_service(
                web::get().to(show_not_found)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}