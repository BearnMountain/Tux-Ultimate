use crate::game::ui::{command::UiCommand, screens::Screen};



pub struct GameResults {

}

impl Screen for GameResults {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        println!("map selection menu not impl");
        return vec![UiCommand::None];
    }
}

impl GameResults {
    pub fn new() -> Self {
        return Self {

        };
    }
}
