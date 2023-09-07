mod router;

use axum::{response::Html, routing::get, Router, extract::State, Server, Json, http};
use tower_http::services::{ServeDir, ServeFile};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::http::{header, Response, StatusCode};
use axum::response::IntoResponse;
use handlebars::Handlebars;
use serde::Serialize;
use crate::router::root::root_router;

#[derive(Clone)]
pub struct AppState {
    handlebars: Handlebars<'static>,
}


#[tokio::main]
async fn main() {
    let state = AppState {
        handlebars: Handlebars::new(),
    };

    let app = Router::new().nest("/", root_router())
        .fallback(error).with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("⚡️ LynixCA (FloofRender) is now running on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}


#[derive(Serialize)]
struct TemplateData<'a> {
    name: &'a str
}

#[derive(Serialize)]
struct ErrorData<'a> {
    message: &'a str,
    status: i32
}

async fn home(State(state): State<AppState>) -> Html<String> {
    let template = include_str!("../pages/index.hbs");
    let data = TemplateData { name: "Lynix" };
    Html(state.handlebars.render_template(template, &data).unwrap())
}

async fn blog(State(state): State<AppState>) -> Html<String> {
    let template = include_str!("../pages/blog/index.hbs");
    let data = TemplateData { name: "Lynix" };
    Html(state.handlebars.render_template(template, &data).unwrap())
}

async fn article(State(state): State<AppState>) -> Html<String> {
    let template = include_str!("../pages/blog/article.hbs");
    let data = TemplateData { name: "Lynix" };
    Html(state.handlebars.render_template(template, &data).unwrap())
}

async fn error(State(state): State<AppState>) -> Html<String> {
    let template = include_str!("../pages/error.hbs");
    let data = ErrorData{ message: "Page unavailable, try again later.", status: 404 };
    Html(state.handlebars.render_template(template, &data).unwrap())
}

/* Lynix API */

async fn api_home() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "version": "v1.0.0-rs-beta",
    });

    Json(json_response)
}

async fn api_error() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "success": false,
        "msg": "An error occurred."
    });

    Json(json_response)
}
