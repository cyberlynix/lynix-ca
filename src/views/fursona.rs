use actix_web::{get, HttpResponse};
use crate::error_types::ErrorKind;
use crate::utils::utils::{ COMPILED_TEMPLATES };

#[get("/fursona")]
pub(crate) async fn show_fursona() -> Result<HttpResponse, ErrorKind> {
    let template = COMPILED_TEMPLATES.render("fursona.html", &tera::Context::new());

    match template {
        Ok(t) => Ok(HttpResponse::Ok().content_type("text/html").body(t)),
        Err(e) => Err(ErrorKind::TemplateError(e.to_string()))
    }
}