use axum::{
    debug_handler,
    response::{IntoResponse, Redirect, Response},
};

use crate::error::Error;

pub mod page;

#[debug_handler]
pub async fn root() -> Result<Response, Error> {
    Ok(Redirect::to("/w/page/main").into_response())
}
