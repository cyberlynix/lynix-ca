use axum::{response::Html, routing::get, Router, extract::State, Server};
use tower_http::services::{ServeDir, ServeFile};
use std::net::SocketAddr;
use handlebars::Handlebars;
use serde::Serialize;
#[derive(Clone)]
struct AppState<'a> {
    handlebars: Handlebars<'a>,
}


#[tokio::main]
async fn main() {
    let state = AppState {
        handlebars: Handlebars::new(),
    };

    let app = Router::new()
        .route("/", get(home))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(error)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

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

async fn home(State(state): State<AppState<'_>>) -> Html<String> {
    let template = include_str!("../pages/index.hbs");
    let data = TemplateData { name: "Lynix" };
    Html(state.handlebars.render_template(template, &data).unwrap())
}

async fn error(State(state): State<AppState<'_>>) -> Html<String> {
    let template = include_str!("../pages/error.hbs");
    let data = ErrorData{ message: "Page unavailable, try again later.", status: 404 };
    Html(state.handlebars.render_template(template, &data).unwrap())
}