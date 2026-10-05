use egui::{Color32, CornerRadius, Stroke, Vec2};

use crate::game::ui::command::UiCommand;

pub mod main_menu;
pub mod game_overlay;

pub mod single_player;

pub mod multiplayer; // covers: server_browser, direct_connect, host
pub mod lobby_menu; // covers host + client
pub mod map_selection_menu;
pub mod loading_screen;
pub mod game_results;
pub mod extras_screen;
pub mod settings_menu;



#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ScreenId {
    MAIN_MENU,
    GAME_OVERLAY,

    // Single player
    SINGLE_PLAYER,

    // Online/Multiplayer
    MULTIPLAYER_MENU,
    SERVER_BROWSER, 
    DIRECT_CONNECT,
    SERVER_HOST, // all screens to connect
    LOADING_SCREEN,
    CLIENT_LOBBY,
    HOST_LOBBY, // advanced control + options
    MAP_SELECTION,
    GAME_RESULTS,

    // General
    SETTINGS,
    EXTRAS,
    QUIT,
}

pub trait Screen {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand>;
}

#[derive(Clone, Debug)]
pub struct MenuTheme {
    // --- surfaces ---
    pub background: Color32, 
    pub panel: Color32,
    pub panel_nested: Color32,
    pub border: Color32,

    // --- text ---
    pub text: Color32,
    pub muted_text: Color32,

    // --- accent ---
    pub accent: Color32,
    pub accent_hovered: Color32,

    // --- buttons ---
    pub button: Color32,
    pub button_hovered: Color32,
    pub button_active: Color32,
    pub button_text: Color32,

    // --- inputs ---
    pub input_bg: Color32,
    // pub input_stroke: Color32,
    // pub input_stroke_focused: Color32,

    // --- selection ---
    pub selected: Color32,
    pub selected_text: Color32,

    // --- semantic ---
    pub error: Color32,
    pub success: Color32,

    // --- typography ---
    pub title_size: f32,
    pub heading_size: f32,
    pub body_size: f32,
    pub small_size: f32,

    // --- layout ---
    pub button_size: Vec2,
    pub input_height: f32,
    pub spacing: f32,
    pub padding: f32,
    pub rounding: f32,
}

impl MenuTheme {
    pub fn dark() -> Self {
        Self {
            background: Color32::from_rgb(18, 19, 23),
            panel: Color32::from_rgb(27, 29, 35),
            panel_nested: Color32::from_rgb(20, 21, 26),
            border: Color32::from_rgb(45, 48, 56),

            text: Color32::from_rgb(230, 231, 235),
            muted_text: Color32::from_rgb(150, 153, 163),

            accent: Color32::from_rgb(88, 130, 255),
            accent_hovered: Color32::from_rgb(110, 150, 255),

            button: Color32::from_rgb(38, 41, 49),
            button_hovered: Color32::from_rgb(50, 54, 64),
            button_active: Color32::from_rgb(30, 33, 40),
            button_text: Color32::from_rgb(230, 231, 235),

            input_bg: Color32::from_rgb(20, 21, 26),

            selected: Color32::from_rgb(88, 130, 255),
            selected_text: Color32::from_rgb(255, 255, 255),

            error: Color32::from_rgb(235, 90, 90),
            success: Color32::from_rgb(90, 200, 130),

            title_size: 24.0,
            heading_size: 17.0,
            body_size: 14.0,
            small_size: 12.0,

            button_size: Vec2::new(120.0, 30.0),
            input_height: 28.0,
            spacing: 10.0,
            padding: 10.0,
            rounding: 6.0,
        }
    }

    pub fn corner_radius(&self) -> CornerRadius {
        CornerRadius::same(self.rounding.round() as u8)
    }

    pub fn stroke(&self, color: Color32) -> Stroke {
        Stroke::new(1.0, color)
    }
}
