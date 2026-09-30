use egui::Context;

use crate::{engine::assets::{gpu_server::Server, ui_server::UiServer}, game::ui::screens::{
    main_menu::{MainMenu, MainMenuAction}, 
    online_menu::{OnlineMenu, OnlineMenuAction},
    settings_menu::{SettingsMenu, SettingsMenuAction},
}};

pub mod screens;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UIMenuAction {
    MAIN_MENU,
    SINGLE_PLAYER,
    ONLINE_MENU,
    SERVER_BROWSER,
    DIRECT_CONNECT,
    CONNECTING,

    LOBBY, 
    HOST_LOBBY, // provided different permissions

    CHARACTER_SELECT,
    MAP_SELECT,
    LOADING,
    IN_GAME,
    SETTINGS,
    EXTRAS,
    QUIT,
    NONE,
}

/*
/// potential add to deal with state changes
enum UiCommand {
    Open(Screen),
    Back,
    Quit,

    RefreshServers,
    ConnectToServer(ServerAddress),
    CancelConnection,

    SelectCharacter(CharacterId),
    SelectMap(MapId),

    SetReady(bool),
    StartMatch,
    LeaveMatch,

    UpdateSettings(SettingChange),
}
*/

pub struct UI {
    info: String,
    menu_action: UIMenuAction,

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
            menu_action: UIMenuAction::MAIN_MENU,
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
    ) -> UIMenuAction {
        let menu_action;

        match self.menu_action {
            UIMenuAction::MAIN_MENU => 
                menu_action = self.render_main_menu(ui),
            UIMenuAction::ONLINE_MENU => 
                menu_action = self.render_online_menu(ui),
            UIMenuAction::IN_GAME =>
                menu_action = self.render_in_game(ui),
            UIMenuAction::SETTINGS => 
                menu_action = self.render_settings(ui),
            _ => { menu_action = UIMenuAction::NONE },
        }
    
        if menu_action != UIMenuAction::NONE {
            self.menu_action = menu_action;
        }
        return menu_action;
    }

    fn render_main_menu(&mut self, context: &Context) -> UIMenuAction {
        let action;
        match self.main_menu.ui(context) {
            MainMenuAction::SINGLE_PLAYER => {
                println!("Starting single player...");
                action = UIMenuAction::SINGLE_PLAYER;
            }

            MainMenuAction::MULTIPLAYER => {
                println!("Starting multiplayer...");
                action = UIMenuAction::ONLINE_MENU;
            }

            MainMenuAction::SETTINGS => {
                println!("Opening settings...");
                action = UIMenuAction::SETTINGS;
            }

            MainMenuAction::EXTRAS => {
                println!("Opening extras...");
                action = UIMenuAction::EXTRAS;
            }

            MainMenuAction::QUIT => {
                println!("Quitting...");
                action = UIMenuAction::QUIT;
            }

            MainMenuAction::NONE => {
                action = UIMenuAction::NONE;
            },
        }

        return action;
	}

    fn render_single_player_menu(&mut self, _context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}

    fn render_online_menu(&mut self, ui: &mut egui::Ui) -> UIMenuAction {
        let action = UIMenuAction::NONE;

        match self.online_menu.ui(ui) {
            OnlineMenuAction::SERVER_BROWSER(_) => {},
            OnlineMenuAction::DIRECT_CONNECT(_) => {},
            OnlineMenuAction::HOST_SERVER(_) => {},
            OnlineMenuAction::NONE => {},
        }

		return action;
	}

    fn render_server_browser(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}
    fn render_direct_connect(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}
    fn render_connecting(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}

    fn render_lobby(&mut self, context: &Context) -> UIMenuAction {
        let mut action = UIMenuAction::NONE;

		return action;
	}
    fn render_lobby_host(&mut self, context: &Context) -> UIMenuAction {
        let mut action = UIMenuAction::NONE;

		return action;
	}

    fn render_character_select(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}
    fn render_map_select(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}
    fn render_loading(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}

    /// game overlay ui
    fn render_in_game(&mut self, context: &Context) -> UIMenuAction {
        let mut action = UIMenuAction::NONE;

		return action;
	}
    fn render_settings(&mut self, ui: &egui::Ui) -> UIMenuAction {
        let action = UIMenuAction::NONE;

        match self.settings_menu.ui(ui) {
            SettingsMenuAction::SAVE => {},
            SettingsMenuAction::RESET => {},
            SettingsMenuAction::EXIT => {},
            SettingsMenuAction::NONE => {},
        }

		return action;
	}
    fn render_extras(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}
}








