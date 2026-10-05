use std::process::exit;

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
            OnlineMenu, 
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
    screens: [Box<dyn Screen>; 13],
    stack: Vec<ScreenId>,

    // main_menu: MainMenu,
    // online_menu: OnlineMenu,
    // settings_menu: SettingsMenu,

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
                Box::new(OnlineMenu::new()), 
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
            // info: "hello world".into(),
            // menu_state: UIMenuState::MAIN_MENU,
            // main_menu: MainMenu::new("ULTIMATE"),
            // online_menu: OnlineMenu::new(),
            // settings_menu: SettingsMenu::new(),
            asset_server: UiServer::new(&context),
        };
    }
     
    /// renders UI as a stack
    /// - ui stack digested by game
    pub fn frame(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Vec<UiCommand> {
        let mut commands: Vec<UiCommand> = Vec::new();
        let Some(stack) = self.stack.last() else {
            return commands;
        };

        match stack {
            ScreenId::MAIN_MENU => self.render_main_menu(ui),
            ScreenId::GAME_OVERLAY => self.render_game_overlay(ui),
            ScreenId::SINGLE_PLAYER => self.render_single_player(ui),

            // online
            ScreenId::MULTIPLAYER_MENU => { 
                println!(" multiplayer shouldnt get here"); 
            },
            ScreenId::SERVER_BROWSER => 
                self.render_multiplayer(ui, ScreenId::SERVER_BROWSER),
            ScreenId::DIRECT_CONNECT => 
                self.render_multiplayer(ui, ScreenId::DIRECT_CONNECT),
            ScreenId::SERVER_HOST => 
                self.render_multiplayer(ui, ScreenId::SERVER_HOST),

            ScreenId::LOADING_SCREEN => self.render_loading_screen(ui),
            ScreenId::HOST_LOBBY => self.render_lobby(ui),
            ScreenId::CLIENT_LOBBY => self.render_lobby(ui),
            ScreenId::MAP_SELECTION => self.render_map_selection(ui),
            ScreenId::GAME_RESULTS => self.render_game_results(ui),

            // general
            ScreenId::SETTINGS => self.render_settings(ui),
            ScreenId::EXTRAS => self.render_extras(ui),
            ScreenId::QUIT => {},
        }

        return commands;
    }

	fn render_main_menu(&mut self, ui: &mut egui::Ui) {
        let main_menu = &mut self.screens[ScreenId::MAIN_MENU as usize];
        let mut commands = main_menu.ui(ui);

        for i in commands {
            match i {
                UiCommand::Push(screen_id) => self.stack.push(screen_id),
                UiCommand::Quit => todo!(),
                _ => {},
            }
        }
    }
	fn render_game_overlay(&mut self, ui: &egui::Ui) {}
	fn render_single_player(&mut self, ui: &egui::Ui) {}

    // rendering is nested 
	fn render_multiplayer(&mut self, ui: &mut egui::Ui, nested: ScreenId) {

        let selection_command = 
            self.screens[ScreenId::MULTIPLAYER_MENU as usize].ui(ui);

        let online_commands = match nested {
            ScreenId::SERVER_BROWSER
            | ScreenId::DIRECT_CONNECT
            | ScreenId::HOST_LOBBY => 
                &mut self.screens[nested as usize].ui(ui),
            _ => {
                log::debug!("{:?} is not a nested screen", nested);
                &mut vec![UiCommand::None]
            },
        };

        self.handle_commands(selection_command);
    }
	fn render_server_browser(&mut self, ui: &egui::Ui) {}
	fn render_direct_connect(&mut self, ui: &egui::Ui) {}
	fn render_host(&mut self, ui: &egui::Ui) {}
	fn render_connecting(&mut self, ui: &egui::Ui) {}
	fn render_lobby(&mut self, ui: &egui::Ui) {}
	fn render_host_lobby(&mut self, ui: &egui::Ui) {}
	fn render_map_selection(&mut self, ui: &egui::Ui) {}
	fn render_loading_screen(&mut self, ui: &egui::Ui) {}
	fn render_game_results(&mut self, ui: &egui::Ui) {}
	fn render_settings(&mut self, ui: &egui::Ui) {}
	fn render_extras(&mut self, ui: &egui::Ui) {}

    // handling commands
    fn handle_commands(&mut self, commands: Vec<UiCommand>) {
        for cmd in commands {
            let Some(stack_top) = self.stack.last() else {
                log::error!("screen stack shouldnt be empty");
                exit(1);
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
                UiCommand::Quit => todo!(),
                UiCommand::None => {},
            }
        }
    }
}








