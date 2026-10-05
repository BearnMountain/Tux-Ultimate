use crate::game::ui::{command::UiCommand, screens::{MenuTheme, Screen}};


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

impl Screen for SettingsMenu {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {

        return vec![UiCommand::None];
    }
}

impl SettingsMenu {
    pub fn new() -> Self {
        return Self {
            theme: MenuTheme::dark(),
        }
    }
}

