use egui::Context;

use crate::{engine::assets::{gpu_server::Server, ui_server::UiServer}, game::ui::screens::{
    main_menu::{MainMenu, MainMenuAction}, 
    online_menu::{OnlineMenu, OnlineMenuAction},
    settings_menu::{SettingsMenu, SettingsMenuAction},
}};

pub mod screens;

pub trait Screen {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand>;
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UiCommand {
    // Navigation
    Push(ScreenId), 
    Pop, // go back button
    PopTo(ScreenId), // pop to n(usually to main menu root)

    // Online
    // ConnectToServer(ServerAddress),
    // CancelConnection,
    // HostServer(GameServerConfig),
    // SelectCharacter/Map
    // StartMatch

    // System
    Quit,
    // UpdateSettings(SettingsConfig),
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ScreenId {
    MAIN_MENU,
    GAME_OVERLAY,

    // Single player
    SINGLE_PLAYER,

    // Online
    ONLINE_MENU,
    SERVER_BROWSER,
    DIRECT_CONNECT,
    CONNECTING,
    LOBBY,
    HOST_LOBBY, // advanced control + options
    MAP_SELECTION,
    LOADING,
    GAME_RESULTS,

    // General
    SETTINGS,
    EXTRAS,
}

pub struct ScreenStack {
    stack: Vec<Box<dyn Screen>>,

    // save all screen states to remain loaded
    screen_cache: HashMap<ScreenId, Box<dyn Screen>>,
}

impl ScreenStack {
    pub fn push(&mut self, id: ScreenId) {
        self.stack.push(Box::new(id));
    }
}

pub struct UI {
    info: String,
    menu_state: UIMenuState,

    main_menu: MainMenu,
    online_menu: OnlineMenu,
    settings_menu: SettingsMenu,

    asset_server: UiServer,
}

impl UI {
    pub fn init(
        context: &egui::Context,
    ) -> Self {
        return Self {
            info: "hello world".into(),
            menu_state: UIMenuState::MAIN_MENU,
            main_menu: MainMenu::new("ULTIMATE"),
            online_menu: OnlineMenu::new(),
            settings_menu: SettingsMenu::new(),
            asset_server: UiServer::new(&context),
        };
    }
     
    /// renders UI as a state machine
    pub fn frame(
        &mut self,
        ui: &mut egui::Ui,
    ) -> UIMenuState {
        let menu_action;

        match self.menu_state {
            UIMenuState::MAIN_MENU => 
                menu_action = self.render_main_menu(ui),
            UIMenuState::ONLINE_MENU => 
                menu_action = self.render_online_menu(ui),
            UIMenuState::IN_GAME =>
                menu_action = self.render_in_game(ui),
            UIMenuState::SETTINGS => 
                menu_action = self.render_settings(ui),
            _ => { menu_action = UIMenuState::NONE },
        }
    
        if menu_action != UIMenuState::NONE {
            self.menu_state = menu_action;
        }
        return menu_action;
    }

    fn render_main_menu(&mut self, context: &Context) -> UIMenuState {
        let action;
        match self.main_menu.ui(context) {
            MainMenuAction::SINGLE_PLAYER => {
                println!("Starting single player...");
                action = UIMenuState::SINGLE_PLAYER;
            }

            MainMenuAction::MULTIPLAYER => {
                println!("Starting multiplayer...");
                action = UIMenuState::ONLINE_MENU;
            }

            MainMenuAction::SETTINGS => {
                println!("Opening settings...");
                action = UIMenuState::SETTINGS;
            }

            MainMenuAction::EXTRAS => {
                println!("Opening extras...");
                action = UIMenuState::EXTRAS;
            }

            MainMenuAction::QUIT => {
                println!("Quitting...");
                action = UIMenuState::QUIT;
            }

            MainMenuAction::NONE => {
                action = UIMenuState::NONE;
            },
        }

        return action;
	}

    fn render_single_player_menu(&mut self, _context: &Context) -> UIMenuState {
		return UIMenuState::MAIN_MENU;
	}

    fn render_online_menu(&mut self, ui: &mut egui::Ui) -> UIMenuState {
        let action = UIMenuState::NONE;

        // match self.online_menu.ui(ui) {
        //
        // }

		return action;
	}

    fn render_server_browser(&mut self, context: &Context) -> UIMenuState {
		return UIMenuState::MAIN_MENU;
	}
    fn render_direct_connect(&mut self, context: &Context) -> UIMenuState {
		return UIMenuState::MAIN_MENU;
	}
    fn render_connecting(&mut self, context: &Context) -> UIMenuState {
		return UIMenuState::MAIN_MENU;
	}

    fn render_character_select(&mut self, context: &Context) -> UIMenuState {
		return UIMenuState::MAIN_MENU;
	}
    fn render_map_select(&mut self, context: &Context) -> UIMenuState {
		return UIMenuState::MAIN_MENU;
	}
    fn render_loading(&mut self, context: &Context) -> UIMenuState {
		return UIMenuState::MAIN_MENU;
	}

    /// game overlay ui
    fn render_in_game(&mut self, context: &Context) -> UIMenuState {
        let mut action = UIMenuState::NONE;

		return action;
	}
    fn render_settings(&mut self, ui: &egui::Ui) -> UIMenuState {
        let action = UIMenuState::NONE;

        match self.settings_menu.ui(ui) {
            SettingsMenuAction::SAVE => {},
            SettingsMenuAction::RESET => {},
            SettingsMenuAction::EXIT => {},
            SettingsMenuAction::NONE => {},
        }

		return action;
	}
    fn render_extras(&mut self, context: &Context) -> UIMenuState {
		return UIMenuState::MAIN_MENU;
	}
}








