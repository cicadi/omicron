use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    app::{App, AppState},
    controllers,
};

pub fn build_router(app: Arc<App>) -> Router {
    Router::new()
        .route("/", get(controllers::root))
        .nest("/w", build_wiki_router())
        .with_state(AppState(app))
}

fn build_wiki_router() -> Router<AppState> {
    Router::new()
        .route("/", get(controllers::wiki::root))
        .route("/page", get(controllers::wiki::page::root))
        .route("/page/{title}", get(controllers::wiki::page::get))
}
