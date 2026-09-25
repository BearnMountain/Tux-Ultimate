use egui::Context;

use crate::game::ui::{UIMenuAction::MAIN_MENU, main_menu::{MainMenu, MainMenuAction}};

pub mod main_menu;

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
    CHARACTER_SELECT,
    MAP_SELECT,
    LOADING,
    IN_GAME,
    SETTINGS,
    EXTRAS,
    QUIT,
    NONE,
}

pub struct UI {
    info: String,
    menu_action: UIMenuAction,

    main_menu: MainMenu,
}

impl UI {
    pub fn init() -> Self {
        return Self {
            info: "hello world".into(),
            menu_action: UIMenuAction::MAIN_MENU,
            main_menu: MainMenu::new("ULTIMATE"),
        };
    }
     
    /// renders UI as a state machine
    pub fn frame(
        &mut self,
        context: &Context,
    ) -> UIMenuAction {
        let menu_action;

        match self.menu_action {
            UIMenuAction::MAIN_MENU => 
                menu_action = self.render_main_menu(context),
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

    fn render_single_player_menu(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}
    fn render_online_menu(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
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
		return UIMenuAction::MAIN_MENU;
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
    fn render_in_game(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}
    fn render_settings(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}
    fn render_extras(&mut self, context: &Context) -> UIMenuAction {
		return UIMenuAction::MAIN_MENU;
	}
}








