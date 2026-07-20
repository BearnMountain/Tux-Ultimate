use std::sync::{OnceLock, RwLock};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub graphics: GraphicsConfig,
}

#[derive(Debug, Deserialize)]
pub struct GraphicsConfig {
    pub backend: String,
}

static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();

impl Config {
    pub fn init(path: &str) {
        let contents = std::fs::read_to_string(path)
            .expect("Failed to read config.toml");
        let config: Config = toml::from_str(&contents)
            .expect("Failed to parse {path}");

        CONFIG.set(RwLock::new(config)).expect("Config already initialized");
    }

    pub fn get() -> &'static RwLock<Config> {
        return CONFIG.get().expect("Config not initialized");
    }
}
