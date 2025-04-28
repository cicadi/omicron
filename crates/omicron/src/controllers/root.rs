use axum::{
    debug_handler,
    response::{IntoResponse, Response},
};

use crate::error::Error;

#[debug_handler]
pub async fn get() -> Result<Response, Error> {
    Ok("hello, world!".into_response())
}
