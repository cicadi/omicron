use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    app::{App, AppState},
    controllers::root,
};

pub fn build_router(app: Arc<App>) -> Router {
    Router::new()
        .route("/", get(root::get))
        .with_state(AppState(app))
}
