use actix_web::{get, HttpResponse, web};
use crate::error_types::ErrorKind;
use crate::utils::utils::{ COMPILED_TEMPLATES };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Session {
    session: Option<String>,
    user: Option<String>
}

#[get("/shock")]
pub(crate) async fn show_shock(query_params: web::Query<Session>) -> Result<HttpResponse, ErrorKind> {
    let session = &query_params.session;
    let user = &query_params.user;

    if let (Some(session), Some(user)) = (&session, &user) {
        let template = COMPILED_TEMPLATES.render("shock.html", &tera::Context::new());

        match template {
            Ok(t) => Ok(HttpResponse::Ok().content_type("text/html").body(t)),
            Err(e) => Err(ErrorKind::TemplateError(e.to_string()))
        }
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("status", &404);
        ctx.insert("message", "Page unavailable, try again later.");

        let template = COMPILED_TEMPLATES.render("error.html", &ctx);

        match template {
            Ok(t) => Ok(HttpResponse::Ok().content_type("text/html").body(t)),
            Err(e) => Err(ErrorKind::TemplateError(e.to_string()))
        }

    }
}