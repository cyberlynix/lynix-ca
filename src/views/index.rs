use actix_web::{get, HttpResponse};
use crate::error_types::ErrorKind;
use crate::utils::utils::{ COMPILED_TEMPLATES };

#[get("/")]
pub(crate) async fn show_index() -> Result<HttpResponse, ErrorKind> {
    let template = COMPILED_TEMPLATES.render("index.html", &tera::Context::new());

    match template {
        Ok(t) => Ok(HttpResponse::Ok().content_type("text/html").body(t)),
        Err(e) => Err(ErrorKind::TemplateError(e.to_string()))
    }
}

pub(crate) async fn show_not_found() -> Result<HttpResponse, ErrorKind> {
    let mut ctx = tera::Context::new();
    ctx.insert("status", &404);
    ctx.insert("message", "Page unavailable, try again later.");

    let template = COMPILED_TEMPLATES.render("error.html", &ctx);
    match template {
        Ok(t) => Ok(HttpResponse::NotFound().content_type("text/html").body(t)),
        Err(e) => Err(ErrorKind::TemplateError(e.to_string()))
    }
}