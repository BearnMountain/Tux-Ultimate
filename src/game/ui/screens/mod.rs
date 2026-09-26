use egui::{Color32, Vec2};

pub mod main_menu;
pub mod online_menu;
pub mod server_browser;

#[derive(Debug, Clone)]
pub struct MenuTheme {
    pub background: Color32,
    pub panel: Color32,
    pub text: Color32,
    pub muted_text: Color32,
    pub accent: Color32,
    pub button: Color32,
    pub button_hovered: Color32,

    pub title_size: f32,
    pub button_size: Vec2,
    pub spacing: f32,
}

impl Default for MenuTheme {
    fn default() -> Self {
        Self {
            background: Color32::from_rgb(12, 12, 16),
            panel: Color32::from_rgb(20, 20, 26),

            text: Color32::from_rgb(235, 235, 240),
            muted_text: Color32::from_rgb(140, 140, 150),

            accent: Color32::from_rgb(90, 150, 255),

            button: Color32::from_rgb(28, 28, 36),
            button_hovered: Color32::from_rgb(42, 42, 54),

            title_size: 52.0,
            button_size: Vec2::new(280.0, 48.0),
            spacing: 12.0,
        }
    }
}
