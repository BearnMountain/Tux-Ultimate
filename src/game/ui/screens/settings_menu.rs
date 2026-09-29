use egui::Key::A;

use crate::game::ui::screens::MenuTheme;


#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SettingsMenuAction {
    SAVE, 
    RESET,
    EXIT,
    NONE,
}

pub struct SettingsMenu {
    
    pub theme: MenuTheme,
}

impl SettingsMenu {
    pub fn new() -> Self {
        return Self {
            theme: MenuTheme::dark(),
        }
    }

    pub fn ui(&mut self, ui: &egui::Ui) -> SettingsMenuAction {
        let mut action = SettingsMenuAction::NONE;



        return action;
    }
}

