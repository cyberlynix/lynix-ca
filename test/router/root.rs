use axum::body::Body;
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

use crate::{AppState, error, fursona, home, shock};
use crate::router::api::api_router;
use crate::router::blog::blog_router;

pub fn root_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", get(home))
        .route("/shock", get(shock))
        .route("/fursona", get(fursona))
        .nest("/api/", api_router())
        .nest("/blog", blog_router())
        .nest_service("/static", ServeDir::new("static"));

    router
}