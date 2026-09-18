use egui::Context;

use crate::{
    engine::assets::{gpu_server::Server, ui_server::UiServer}, game::ui::{command::UiCommand, screens::{
        Screen, ScreenId, 
        extras_screen::{ExtrasScreen}, 
        game_overlay::GameOverlay, game_results::GameResults, 
        loading_screen::LoadingScreen, 
        lobby_menu::LobbyMenu, 
        main_menu::{MainMenu, MainMenuAction}, 
        map_selection_menu::MapSelectionMenu, 
        multiplayer::{
            MultiplayerMenu, 
            direct_connect::DirectConnect, 
            host_server::HostServer, 
            server_browser::ServerBrowser
        }, settings_menu::{SettingsMenu, SettingsMenuAction}, 
        single_player::SinglePlayer,
    }},
};

pub mod screens;
pub mod command;

pub struct UI {
    // all screens
    screens: [Box<dyn Screen>; ScreenId::EXTRAS as usize],
    stack: Vec<ScreenId>,
    asset_server: UiServer,
}

impl UI {
    pub fn init(
        context: &egui::Context,
    ) -> Self {
        return Self {
            screens: [
                Box::new(MainMenu::new("Test")),
                Box::new(GameOverlay::new()),
                Box::new(SinglePlayer::new()),

                // multiplayer
                Box::new(MultiplayerMenu::new()), 
                Box::new(ServerBrowser::new()),
                Box::new(DirectConnect::new()),
                Box::new(HostServer::new()),

                Box::new(LoadingScreen::new()),
                Box::new(LobbyMenu::new()),
                Box::new(MapSelectionMenu::new()),
                Box::new(GameResults::new()),
                Box::new(SettingsMenu::new()),
                Box::new(ExtrasScreen::new()),
            ],
            stack: vec![ScreenId::MAIN_MENU],
            asset_server: UiServer::new(&context),
        };
    }
     
    /// renders UI as a stack
    /// - ui stack digested by game
    pub fn frame(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Vec<UiCommand> {
        let Some(stack) = self.stack.last() else {
            return Vec::new();
        };

        let commands = match stack {
            ScreenId::MAIN_MENU => self.render_main_menu(ui),
            ScreenId::GAME_OVERLAY => self.render_game_overlay(ui),
            ScreenId::SINGLE_PLAYER => self.render_single_player(ui),

            // online
            ScreenId::MULTIPLAYER_MENU => { 
                println!(" multiplayer shouldnt get here"); 
                Vec::new()
            },
            ScreenId::SERVER_BROWSER => 
                self.render_multiplayer(ui, ScreenId::SERVER_BROWSER),
            ScreenId::DIRECT_CONNECT => 
                self.render_multiplayer(ui, ScreenId::DIRECT_CONNECT),
            ScreenId::HOST_SERVER => 
                self.render_multiplayer(ui, ScreenId::HOST_SERVER),

            ScreenId::LOADING_SCREEN => self.render_loading_screen(ui),
            ScreenId::HOST_LOBBY => self.render_lobby(ui),
            ScreenId::CLIENT_LOBBY => self.render_lobby(ui),
            ScreenId::MAP_SELECTION => self.render_map_selection(ui),
            ScreenId::GAME_RESULTS => self.render_game_results(ui),

            // general
            ScreenId::SETTINGS => self.render_settings(ui),
            ScreenId::EXTRAS => self.render_extras(ui),
            ScreenId::QUIT => vec![UiCommand::Quit],
        };

        return commands;
    }

	fn render_main_menu(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        let main_menu = &mut self.screens[ScreenId::MAIN_MENU as usize];
        let commands = main_menu.ui(ui);
        return self.handle_commands(commands);
    }

	fn render_game_overlay(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
    }
	fn render_single_player(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
    }

    // rendering is nested 
	fn render_multiplayer(
        &mut self, ui: 
        &mut egui::Ui, 
        nested: ScreenId
    ) -> Vec<UiCommand> {
        let selection_command = 
            self.screens[ScreenId::MULTIPLAYER_MENU as usize].ui(ui);

        let online_commands = match nested {
            ScreenId::SERVER_BROWSER
            | ScreenId::DIRECT_CONNECT
            | ScreenId::HOST_SERVER => 
                self.screens[nested as usize].ui(ui),
            _ => {
                log::debug!("{:?} is not a nested screen", nested);
                vec![UiCommand::None]
            },
        };

        let mut a = self.handle_commands(selection_command);
        a.append(&mut self.handle_commands(online_commands));
        return a;
    }
	fn render_server_browser(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
    }
	fn render_direct_connect(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
	}
	fn render_host(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
	}
	fn render_connecting(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
	}
	fn render_lobby(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
	}
	fn render_host_lobby(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
	}
	fn render_map_selection(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
	}
	fn render_loading_screen(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
	}
	fn render_game_results(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
	}
	fn render_settings(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
	}
	fn render_extras(&mut self, ui: &egui::Ui) -> Vec<UiCommand> {
        let mut commands = Vec::new();
        return self.handle_commands(commands);
	}

    // handling commands
    fn handle_commands(&mut self, commands: Vec<UiCommand>) -> Vec<UiCommand> {
        if commands.is_empty() {
            return Vec::new();
        }

        let mut game_commands = Vec::new(); // leftover that wont be handled by ui
        for cmd in commands {
            if cmd == UiCommand::None {
                continue;
            }

            let Some(stack_top) = self.stack.last() else {
                log::error!("screen stack shouldnt be empty");
                return vec![UiCommand::Quit];
            };

            match cmd {
                UiCommand::Push(screen_id) => {
                    if *stack_top != screen_id { self.stack.push(screen_id); }
                },
                UiCommand::Pop => {
                    self.stack.pop();
                },
                UiCommand::PopTo(screen_id) => {
                    if let Some(i) = self.stack.iter().rposition(|&id|
                        id == screen_id
                    ) {
                        if i > 0 {
                            self.stack.truncate(i + 1);
                        }
                    }
                },

                // residual commands to be handled by the game
                _ => {
                    game_commands.push(cmd);
                },
            }
        }

        return game_commands;
    }
}








