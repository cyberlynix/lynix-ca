use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

use crate::{AppState, error, home};
use crate::router::api::api_router;
use crate::router::blog::blog_router;

pub fn root_router() -> axum::Router<AppState> {
    let router = Router::new()
        .route("/", get(home))
        .nest("/api/", api_router())
        .nest("/blog", blog_router())
        .nest_service("/assets", ServeDir::new("assets"));

    router
}