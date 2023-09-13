use axum::Router;
use axum::routing::get;

use crate::{AppState, article, blog};

pub fn blog_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", get(blog))
        .route("/:id", get(article));

    router
}