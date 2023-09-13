use axum::Router;
use axum::routing::get;

use crate::{api_error, api_home, AppState};

pub fn api_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", get(api_home))
        .fallback(api_error);

    router
}
