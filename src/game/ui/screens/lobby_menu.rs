use egui_extras::{Size, StripBuilder};

use crate::game::{server::net::config::GameServerConfig, ui::{command::UiCommand, screens::{Screen, ScreenId}}};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LobbyMenuAction {
    NONE,
}

pub struct LobbyMenu {
    game_server_config: GameServerConfig,
}

impl Screen for LobbyMenu {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        StripBuilder::new(ui)
            .size(Size::relative(0.7)) // top 70%
            .size(Size::remainder()) // bottom 30%
            .vertical(|mut strip| {
                strip.strip(|builder| { // top
                    builder
                        .size(Size::relative(0.2)) // lobby info
                        .size(Size::relative(0.5)) // character selection
                        .size(Size::remainder()) // player info
                        .horizontal(|mut strip| {
                            strip.cell(|ui| { self.lobby_info_panel(ui); });
                            strip.cell(|ui| { self.character_selection_panel(ui); });
                            strip.cell(|ui| { self.player_info_panel(ui); });
                        });
                });
                strip.strip(|builder| { // bottom
                    builder
                        .size(Size::relative(0.7)) // chat
                        .size(Size::remainder()) // lobby action
                        .horizontal(|mut strip| {
                            strip.cell(|ui| { self.chat_panel(ui); });
                            strip.cell(|ui| { self.lobby_action_panel(ui); });
                        });
                });
            });

        return vec![UiCommand::None];
    }
}

impl LobbyMenu {
    pub fn new() -> Self {
        return Self {
            game_server_config: GameServerConfig::default(),
        };
    }

    pub fn set_game_server_config(&mut self, config: &GameServerConfig) {
        self.game_server_config = config.clone();
    }

    fn lobby_info_panel(&mut self, ui: &mut egui::Ui) {
        ui.label("lobby info");
    }
    fn character_selection_panel(&mut self, ui: &mut egui::Ui) {
        ui.label("character selection");
    }
    fn player_info_panel(&mut self, ui: &mut egui::Ui) {
        ui.label("player info");
    }
    fn chat_panel(&mut self, ui: &mut egui::Ui) {
        ui.label("chat");
    }
    fn lobby_action_panel(&mut self, ui: &mut egui::Ui) {
        ui.label("lobby action");
    }
}
