mod router;

use std::collections::HashMap;
use axum::{response::Html, routing::get, Router, extract::State, Server, Json, http};
use tower_http::services::{ServeDir, ServeFile};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::Query;
use axum::http::{header, Response, StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::router::root::root_router;
use tera::Tera;
extern crate chrono;

use chrono::{Datelike, Local};

#[derive(Clone)]
pub struct AppState {

}


#[tokio::main]
async fn main() {
    let state = AppState {

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

fn tera_include() -> Tera {
    let tera = Tera::new("pages/**/*").unwrap();
    tera
}

fn common_context() -> tera::Context {
    let mut context = tera::Context::new();
    context.insert("title", "axum-tera");
    context
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
    let tera = tera_include();
    let mut context = common_context();

    let output = tera.render("index.html", &context);
    Html(output.unwrap())
}

#[derive(Deserialize, Debug)]
struct SessionQuery {
    session: Option<String>,
    user: Option<String>,
}

async fn shock(
    params:  Query<SessionQuery>,
    State(state): State<AppState>
) -> Html<String> {
    // Check if both session_query and user_query exist.
    if let (Some(session), Some(user)) = (&params.session, &params.user) {
        let tera = tera_include();
        let mut context = common_context();

        let output = tera.render("shock.html", &context);
        Html(output.unwrap())
    } else {
        let tera = tera_include();
        let mut context = common_context();

        context.insert("status", &404);
        context.insert("message", "Page unavailable, try again later.");

        let output = tera.render("error.html", &context);
        Html(output.unwrap())
    }
}

async fn fursona(State(state): State<AppState>) -> Html<String> {
    let tera = tera_include();
    let mut context = common_context();

    let output = tera.render("fursona.html", &context);
    Html(output.unwrap())
}
async fn blog(State(state): State<AppState>) -> Html<String> {
    let tera = tera_include();
    let mut context = common_context();

    let output = tera.render("blog/index.html", &context);
    Html(output.unwrap())
}

async fn article(State(state): State<AppState>) -> Html<String> {
    let tera = tera_include();
    let mut context = common_context();

    let current_date = Local::now().date();

    let formatted_date = format!(
        "{:02}{} {}, {}",
        current_date.day(),
        match current_date.day() {
            1 | 21 | 31 => "st",
            2 | 22 => "nd",
            3 | 23 => "rd",
            _ => "th",
        },
        match current_date.month() {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "",
        },
        current_date.year(),
    );

    context.insert("title", "Test");
    context.insert("date", &formatted_date);
    context.insert("author", "Lynix");
    context.insert("content", "Test Content doing cyber beeps'n boops.");

    let output = tera.render("blog/article.html", &context);
    Html(output.unwrap())
}

async fn error(State(state): State<AppState>) -> Html<String> {
    let tera = tera_include();
    let mut context = common_context();

    context.insert("status", &404);
    context.insert("message", "Page unavailable, try again later.");

    let output = tera.render("error.html", &context);
    Html(output.unwrap())
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
