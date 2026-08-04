use std::sync::{OnceLock, RwLock};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub window: WindowConfig,
    pub graphics: GraphicsConfig,
    pub keybinds: KeybindConfig,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub tick_rate: u32,
}

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct GraphicsConfig {
    pub backend: String,
}

#[derive(Debug, Deserialize)]
pub struct KeybindConfig {
    pub player_left: String,
    pub player_right: String,
    pub player_up: String,
    pub player_down: String,
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
