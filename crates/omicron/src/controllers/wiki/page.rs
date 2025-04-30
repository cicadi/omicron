use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, Redirect, Response},
};

use crate::error::Error;

#[debug_handler]
pub async fn root() -> Result<Response, Error> {
    Ok(Redirect::to("/w/page/main").into_response())
}

#[debug_handler]
pub async fn get(Path(title): Path<String>) -> Result<Response, Error> {
    Ok(format!("visited page `{title}`").into_response())
}
