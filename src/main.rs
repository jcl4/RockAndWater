use log::info;
use serde::Deserialize;
use simple_logger;
use std::{error::Error, fs::File, io::prelude::*};
use toml;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

mod app;
mod input;
mod model;
mod renderer;

use app::App;

#[derive(Debug, Deserialize)]
pub struct Config {
    window: WindowConfig,
    application: AppConfig,
}

#[derive(Debug, Deserialize)]
struct WindowConfig {
    fullscreen: bool,
    vsync: bool,
    width: i32,
    height: i32,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    name: String,
    logging: bool,
}

fn main() -> Result<()> {
    let mut file = File::open("app_settings.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;

    if config.application.logging {
        simple_logger::init_with_level(log::Level::Info)?;
    }

    info!("Config: {:#?}", config);

    let app = App::new(config)?;
    app.run();

    Ok(())
}
