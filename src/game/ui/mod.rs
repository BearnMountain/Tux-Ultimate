use egui::Context;

use crate::game::ui::main_menu::{MainMenu, MainMenuAction};

pub mod main_menu;

enum UIMode {
    MAIN_MENU,
    SERVER,
    CLIENT,
}

pub struct UI {
    info: String,
    main_menu: MainMenu,
}

impl UI {
    pub fn init() -> Self {
        return Self {
            info: "hello world".into(),
            main_menu: MainMenu::new("ULTIMATE"),
        };
    }
     
    pub fn frame(
        &mut self,
        context: &Context,
    ) {
        match self.main_menu.ui(context) {
            MainMenuAction::SINGLE_PLAYER => {
                println!("Starting single player...");
            }

            MainMenuAction::MULTIPLAYER => {
                println!("Starting multiplayer...");
            }

            MainMenuAction::OPTIONS => {
                println!("Opening settings...");
            }

            MainMenuAction::EXTRAS => {
                println!("Opening extras...");
            }

            MainMenuAction::QUIT => {
                println!("Quitting...");
            }

            MainMenuAction::NONE => {},
        }
    }
}








