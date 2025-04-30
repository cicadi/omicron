use std::{net::SocketAddr, sync::Arc};

use axum::{ServiceExt, extract::Request};
use tokio::net::TcpListener;
use tower_http::normalize_path::NormalizePathLayer;
use tower_layer::Layer;

use omicron::{App, app::Config, error::Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;

    let app = Arc::new(App::new(Config::from_env()?)?);
    let router =
        NormalizePathLayer::trim_trailing_slash().layer(omicron::build_router(app.clone()));

    let listener = TcpListener::bind(&app.config.server_url).await?;
    println!("> Server started. Listening on: {}", app.config.server_url);

    Ok(axum::serve(
        listener,
        ServiceExt::<Request>::into_make_service_with_connect_info::<SocketAddr>(router),
    )
    .await?)
}
