use log::{info, warn};
use nalgebra as na;
use serde::Deserialize;
use simplelog as sl;
use std::{error::Error, fs::File, io::prelude::*};
use toml;
use futures::executor::block_on;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

mod app;
mod input;
mod objects;
mod renderer;

// pub use renderer::Renderer;
use app::App;
use renderer::Renderer;

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
    bg_color: [f32; 4],
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
        let file = "./notes/log.txt";
        let _ = sl::CombinedLogger::init(vec![
            sl::SimpleLogger::new(sl::LevelFilter::Warn, sl::Config::default()),
            sl::WriteLogger::new(
                sl::LevelFilter::max(),
                sl::Config::default(),
                File::create(file)?,
            ),
        ]);
    }
    warn!("Logging working");
    info!("Config: {:#?}", config);

    let app = block_on(App::new(config))?;
    app.run();

    Ok(())
}
