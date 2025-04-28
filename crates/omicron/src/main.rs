use std::sync::Arc;

use tokio::net::TcpListener;

use omicron::{App, app::Config, error::Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;

    let app = Arc::new(App::new(Config::from_env()?)?);
    let router = omicron::build_router(app.clone());

    let listener = TcpListener::bind(&app.config.server_url).await?;
    println!("> Server started. Listening on: {}", app.config.server_url);

    Ok(axum::serve(listener, router).await?)
}
