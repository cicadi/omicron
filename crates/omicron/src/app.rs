use std::{ops::Deref, sync::Arc};

use crate::error::Error;

pub struct Config {
    pub server_url: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            server_url: std::env::var("SERVER_URL")?,
            database_url: std::env::var("DATABASE_URL")?,
        })
    }
}

pub struct App {
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Result<Self, Error> {
        Ok(Self { config })
    }
}

#[derive(Clone)]
pub struct AppState(pub Arc<App>);

impl Deref for AppState {
    type Target = App;

    fn deref(&self) -> &App {
        &self.0
    }
}
